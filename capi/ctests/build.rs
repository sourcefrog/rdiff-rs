/// Build an external library of tests in C, to test the C API.

extern crate cc;

pub fn main() {
    // Build a C library containing C test functions that we can then call from Rust.
    cc::Build::new()
        .include("../include")
        .file("tests/version_test.c")
        .compile("rdiff_capi_ctests");
}
