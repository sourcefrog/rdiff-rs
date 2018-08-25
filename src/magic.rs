// rdiff(rust) -- library for network deltas
// Copyright 2018 Martin Pool.

//! Magic numbers identifying file formats.
//!
//! All are encoded in big-endian u32.
 
#![allow(dead_code)]

/// Delta file formats.
#[derive(Debug, Copy, Clone)]
pub enum DeltaFormat {
    /// A delta file. 
    ///
    /// There's only one format so far.
    Delta = 0x72730236,    // "rs\x026"
}

/// Signature file formats.
#[derive(Debug, Copy, Clone)]
pub enum SignatureFormat {
    /// A signature file with MD4 magic. (Deprecated because insecure).
    Md4Sig = 0x72730136,   // "rs\x016"

    /// A signature file with BLAKE2 strong signatures. (Current.)
    Blake2Sig = 0x72730137,  // "rs\x017"
}