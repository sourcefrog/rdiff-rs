// rdiff-rs capi -- C api to a Rust library for network deltas.
// Copyright 2015, 2016, 2018 Martin Pool.

extern crate libc;
extern crate rdiff;

/// Nul-terminated version number for ease of C binding.
// 
// Unfortunately this can't be automatically generated from the crate
// version, it seems.
pub static VERSION: &'static str = "0.0.0\0";

// NB: These should stay in sync with the C result enums.
//
// TODO: Maybe these should be attributes of rust-library error types?
#[repr(C)]
pub enum RsResult {
    Done = 0,

    /// Blocked waiting for more data.
    Blocked =    1,

    /// The job is still running, and not yet finished or blocked.
    Running  =       2,

    TestSkipped =   77,     //< Test neither passed or failed.

    IoError =    100,    //< Error in file or network IO. */
    SyntaxError =   101,    //< Command line syntax error. */
    MemError =    102,    //< Out of memory. */
    /// Unexpected end of input file, perhaps due to a truncated file
    /// or dropped network connection.
    InputEnded =    103,
    /// Bad magic number at start of stream.  Probably not a librsync file,
    /// or possibly the wrong kind of file or from an incompatible
    /// library version.
    BadMagic =      104,
    Unimplemented =  105,    //< Author is lazy. */
    Corrupt =        106,    //< Unbelievable value in stream. */
    InternalError = 107,    //< Probably a library bug. */
    /// Bad value passed in to library, probably an application bug.
    ParamErorr =    108,
}

#[no_mangle]
pub extern fn rs_version() -> *const libc::c_char {
    // Version from environment has nul termination (I think we can count on this?)
    return VERSION.as_ptr() as *const libc::c_char;
}

#[no_mangle]
pub extern fn rs_strerror(r: RsResult) -> *const libc::c_char {
    match r {
        RsResult::Done => b"OK\0".as_ptr() as *const libc::c_char,

        // case RS_DONE:
        //     return "OK";
        // case RS_RUNNING:
        //     return "still running";
        // case RS_BLOCKED:
        //     return "blocked waiting for input or output buffers";
        // case RS_BAD_MAGIC:
        //     return "bad magic number at start of stream";
        // case RS_INPUT_ENDED:
        //     return "unexpected end of input";
        // case RS_CORRUPT:
        //     return "stream corrupt";
        // case RS_UNIMPLEMENTED:
        //     return "unimplemented case";
        // case RS_MEM_ERROR:
        //     return "out of memory";
        // case RS_IO_ERROR:
        //     return "IO error";
        // case RS_SYNTAX_ERROR:
        //     return "bad command line syntax";
        // case RS_INTERNAL_ERROR:
        //     return "library internal error";
        //
        _ => b"unexplained problem\0".as_ptr() as *const libc::c_char,
    }
}

#[cfg(test)]
#[test]
pub fn test_versions_consistent() {
    // I can't work out how to automatically store a static CString, but
    // let's at least check they're in sync, and that ours has a nul.
    assert_eq!(VERSION.as_bytes()[VERSION.len()-1], 0);
    let their_v = rdiff::VERSION;
    let l = their_v.len();
    assert_eq!(VERSION.len(), l + 1);
    assert_eq!(VERSION[0..l], their_v.to_string());

    // It should also be consistent with the Cargo version for rdiff-capi-rs.
    assert_eq!(their_v, env!("CARGO_PKG_VERSION"));
}