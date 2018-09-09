// rdiff(rust) -- library for network deltas
// Copyright 2018 Martin Pool.

//! Weak rolling checksums: the first layer of rdiff finding similar blocks between
//! files, even with insertions or removals.

#![allow(dead_code)]

use std::num::Wrapping;

/// Generic rollsum algorithm trait.
///
/// Rollsums hold a checksum across a contiguous range of bytes, which can roll
/// forward through a file, adding a new byte to the right hand side and, separately
/// removing one from the end.
pub trait Rollsum {
    /// Return consolidated u32 rolling sum digest.
    fn digest(&self) -> u32;

    /// Add one byte of data to the leading end of the rolling sum window.
    fn roll_in(&mut self, c_in: u8);

    /// Remove one byte of data from the trailing end of the rolling sum window.
    fn roll_out(&mut self, c_out: u8);

    /// In a single operation add one byte to the leading end and remove one byte from the trailing end.
    fn rotate(&mut self, c_out: u8, c_in: u8);

    /// Add a slice of bytes to the rollsum state.
    fn update(&mut self, buf: &[u8]);
}

/// The rollsum algorithm used by rdiff 1.0 and 2.0.
#[derive(Debug, Default, Copy, Clone)]
pub struct Rollsum1 {
    /// Truncated number of bytes included in the summed state.
    count: Wrapping<u16>,

    /// s1 part of sum.
    s1: Wrapping<u16>,
    /// s2 part of sum.
    s2: Wrapping<u16>,
}

impl Rollsum1 {
    const CHAR_OFFSET: Wrapping<u16> = Wrapping(31);

    pub fn new() -> Rollsum1 {
        Rollsum1::default()
    }
}

impl Rollsum for Rollsum1 {
    fn digest(&self) -> u32 {
        (self.s2.0 as u32) << 16 | (self.s1.0 as u32)
    }

    fn roll_in(&mut self, c_in: u8) {
        self.s1 += Rollsum1::CHAR_OFFSET + Wrapping(c_in as u16);
        self.s2 += self.s1;
        self.count += Wrapping(1);
    }

    fn roll_out(&mut self, c_out: u8) {
        let c_out = Wrapping(c_out as u16);
        self.s1 -= c_out + Rollsum1::CHAR_OFFSET;
        self.s2 -= self.count * (c_out + Rollsum1::CHAR_OFFSET);
        self.count -= Wrapping(1);
    }

    fn rotate(&mut self, c_out: u8, c_in: u8) {
        let c_in = Wrapping(c_in as u16);
        let c_out = Wrapping(c_out as u16);
        self.s1 += c_in - c_out;
        self.s2 += self.s1 - (self.count * (c_out + Rollsum1::CHAR_OFFSET));
    }

    fn update(&mut self, buf: &[u8]) {
        let mut s1 = self.s1;
        let mut s2 = self.s2;
        for c in buf {
            s1 += Wrapping(*c as u16);
            s2 += s1;
        }
        let len = buf.len() as u32;
        let ll = Wrapping(buf.len() as u16);
        let trilen = Wrapping(((len * (len + 1)) / 2) as u16);
        // Now add the corresponding char offsets.
        s1 += ll * Rollsum1::CHAR_OFFSET;
        s2 += trilen * Rollsum1::CHAR_OFFSET;

        self.count += ll;
        self.s1 = s1;
        self.s2 = s2;
    }
}


#[cfg(test)]
mod test {
    use super::{Rollsum, Rollsum1};

    #[test]
    pub fn default_value() {
        let rs = Rollsum1::new();
        assert_eq!(rs.count.0, 0);
        assert_eq!(rs.s1.0, 0);
        assert_eq!(rs.s2.0, 0);
        assert_eq!(rs.digest(), 0u32);
    }

    #[test]
    pub fn rollsum() {
        // TODO: Check behavior on high u8 inputs, and check against C librsync.
        let mut rs = Rollsum1::new();
        rs.roll_in(0u8);
        assert_eq!(rs.count.0, 1);
        assert_eq!(rs.digest(), 0x001f001f);

        rs.roll_in(1u8);
        rs.roll_in(2u8);
        rs.roll_in(3u8);            // [0, 1, 2, 3]
        assert_eq!(rs.count.0, 4);
        assert_eq!(rs.digest(), 0x01400082);

        // Test rotations.
        //
        // Roll out the 0 from the start of the ring, and roll in a 4.
        // Final state: [1,2,3,4]
        rs.rotate(0, 4);
        assert_eq!(rs.count.0, 4);
        assert_eq!(rs.digest(), 0x014a0086);

        // Rotate through three more bytes.
        rs.rotate(1, 5);
        rs.rotate(2, 6);
        rs.rotate(3, 7);
        assert_eq!(rs.count.0, 4);
        assert_eq!(rs.digest(), 0x01680092);

        // Test rolling data out, shortening the window.
        rs.roll_out(4); // [5, 6, 7]
        assert_eq!(rs.count.0, 3);
        assert_eq!(rs.digest(), 0x00dc006f);

        rs.roll_out(5);
        rs.roll_out(6);
        rs.roll_out(7); // []
        assert_eq!(rs.count.0, 0);
        assert_eq!(rs.digest(), 0);
    }

    #[test]
    pub fn update() {
        let mut rs = Rollsum1::new();
        let mut buf = [0u8; 256];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }
        rs.update(&buf);
        assert_eq!(rs.digest(), 0x3a009e80);
    }
}