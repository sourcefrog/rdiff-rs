// rdiff (Rust) network deltas.
// Copyright 2018 Martin Pool.

//! Generate file signatures.
//!
//! Signatures describe a 'base' or 'old' file, and allow deltas to be generated without
//! access to the old file.

use std::io::{BufWriter, Read, Write, Result};
use std::vec::Vec;

use byteorder::{BigEndian, WriteBytesExt};
use cast::usize;

use super::magic::SignatureFormat;
use super::rollsum::{Rollsum, Rollsum1};

/// Configuration options for a generated signature file.
/// 
/// The values from `SignatureOptions::default()` are usually good, but applications
/// might want to set the `block_len`.
#[derive(Debug, Copy, Clone)]
pub struct SignatureOptions {
    /// Format of the signature, identified by its magic number.
    pub magic: SignatureFormat,

    /// Length of a block in bytes.
    /// 
    /// Smaller blocks produce larger signatures because there are more blocks, but allow matching
    /// smaller common regions between files.
    pub block_len: u32,

    /// Length of strong signatures.
    /// 
    /// This is normally best left at the default, which is the strong hash, but
    /// they may be truncated to get smaller signatures although with a risk of exploitable
    /// collisions.
    pub strong_len: u32,
}

impl SignatureOptions {
    pub fn default() -> SignatureOptions {
        SignatureOptions {
            magic: SignatureFormat::Blake2Sig,
            block_len: super::DEFAULT_BLOCK_LEN,
            strong_len: 8, // Whole Blake2 hash length.
        }
    }
}

fn write_u32be(f: &mut Write, a: u32) -> Result<()> {
    f.write_u32::<BigEndian>(a)
}

/// Fill a block buffer with data from the input file, retrying if necessary.
///
/// There are three possibilities:
/// 
/// 1. The input is already at end-of-file: we read zero bytes and will not try again.
/// 
/// 2. There's a regular full size block.
/// 
/// 3. There is less than a full block, and then the end of the file. In this case we
/// return the contents, but we don't want to try again next time, as that could generate
/// two short blocks.
/// 
/// We need to distinguish these even though any particular read from the file might
/// return short. There might be following blocks iff this block is full sized.
/// 
/// `buf.len()` is the block length.
/// 
/// Returns Ok(bytes_read).
fn fill_buffer(inf: &mut Read, buf: &mut [u8]) -> Result<usize> {
    let mut bytes_read: usize = 0;
    while bytes_read < buf.len() {
        let l = inf.read(&mut buf[bytes_read..])?;
        if l == 0 {
            break; // eof
        } else {
            bytes_read += l;
        }
    }
    return Ok(bytes_read);

}

/// Generate a signature, reading a basis file and writing a signature file.
pub fn generate_signature(basis: &mut Read, options: &SignatureOptions, sig: &mut Write) -> Result<()> {
    // This cast should be always be safe on 32-bit platforms and will work on platforms
    // with 16-bit pointers (I think) as long as the blocks are <64k. And blocks that are
    // too large to fit in memory aren't likely to work well anyhow...
    let mut buf = Vec::with_capacity(usize(options.block_len));

    let sig = &mut BufWriter::new(sig);
    write_u32be(sig, options.magic as u32)?;
    write_u32be(sig, options.block_len)?;
    write_u32be(sig, options.strong_len)?;

    loop {
        let l = fill_buffer(basis, &mut buf)?;
        if l == 0 { break; }
        let b = &buf[..l];
        {
            let mut rs = Rollsum1::new();
            rs.update(b);
            write_u32be(sig, rs.digest())?;
        }
        // TODO: Calculate and write out strong sum!
        if l < buf.len() { break; } // Short block must be the last.
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::vec::Vec;
    use std::io::Cursor;
    use super::*;
    
    #[test]
    pub fn empty_signature_header() {
        let mut sig_buf = Cursor::new(Vec::<u8>::new());
        let mut empty_input = Cursor::new(Vec::<u8>::new());
        let options = SignatureOptions::default();
        assert_eq!(options.block_len, 2 << 10);

        generate_signature(&mut empty_input, &options, &mut sig_buf).unwrap();
        assert_eq!(*sig_buf.get_ref(),
            [b'r', b's', 0x01, 0x37,  // BLAKE2 sig magic
            0, 0, 8, 0, // 2kB blocks
            0, 0, 0, 8, // 8 byte BLAKE2 hashes
            ]);
    }
}