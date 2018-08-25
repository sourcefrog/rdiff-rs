# Rust rdiff C API C tests

This is:

* a driver in Rust
* that calls test functions written in C
* that exercise the C API provided by Rust rdiff

This is in a separate subcrate so that its build script doesn't pollute the C API
itself: we don't need the tests linked into the installed C library.