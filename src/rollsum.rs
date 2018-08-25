// librdiff(rust) -- library for network deltas
// Copyright 2018 Martin Pool.

//! Weak rolling checksums: the first layer of rdiff finding similar blocks between
//! files, even with insertions or removals.

#![allow(dead_code)]

use std::num::Wrapping;

/// We should make this something other than zero to improve the checksum
/// algorithm: tridge suggests a prime number.
const CHAR_OFFSET: Wrapping<u16> = Wrapping(31u16);

/// Accumulated rollsum state.
#[derive(Debug, Default, Copy, Clone)]
pub struct Rollsum {
    /// Truncated number of bytes included in the summed state.
    count: Wrapping<u16>,

    /// s1 part of sum.
    s1: Wrapping<u16>,
    /// s2 part of sum.
    s2: Wrapping<u16>,
}

impl Rollsum {
    pub fn new() -> Rollsum {
        Rollsum::default()
    }

    /// Return consolidated u32 rolling sum digest.
    pub fn digest(&self) -> u32 {
        (self.s2.0 as u32) << 16 | (self.s1.0 as u32)
    }

    /// Add ("roll in") one byte of input to the sum.
    pub fn roll_in(&mut self, a: u8) {
        self.s1 += CHAR_OFFSET + Wrapping(a as u16);
        self.s2 += self.s1;
        self.count += Wrapping(1);
    }

    pub fn roll_out(&mut self, c_out: u8) {
        let c_out = Wrapping(c_out as u16);
        self.s1 -= c_out + CHAR_OFFSET;
        self.s2 -= self.count * (c_out + CHAR_OFFSET);
        self.count -= Wrapping(1);
    }

    /// Remove one byte from the tail, and add a new one at the head, rolling forward.
    pub fn rotate(&mut self, c_out: u8, c_in: u8) {
        let c_in = Wrapping(c_in as u16);
        let c_out = Wrapping(c_out as u16);
        // TODO: Fix wrapping
        self.s1 += c_in - c_out;
        self.s2 += self.s1 - (self.count * (c_out + CHAR_OFFSET));
    }

    /// Update the state from a block of bytes.Rollsum.
    pub fn update(&mut self, buf: &[u8]) {
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
        s1 += ll * CHAR_OFFSET;
        s2 += trilen * CHAR_OFFSET;

        self.count += ll;
        self.s1 = s1;
        self.s2 = s2;
    }
}


#[cfg(test)]
mod test {
    use super::Rollsum;

    #[test]
    pub fn default_value() {
        let rs = Rollsum::new();
        assert_eq!(rs.count.0, 0);
        assert_eq!(rs.s1.0, 0);
        assert_eq!(rs.s2.0, 0);
        assert_eq!(rs.digest(), 0u32);
    }

    #[test]
    pub fn rollsum() {
        // TODO: Check behavior on high u8 inputs, and check against C librsync.
        let mut rs = Rollsum::new();
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
        let mut rs = Rollsum::new();
        let mut buf = [0u8; 256];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }
        rs.update(&buf);
        assert_eq!(rs.digest(), 0x3a009e80);
    }
}