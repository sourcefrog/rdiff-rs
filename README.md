# librsync-impl-rs [pre-alpha]

This is the start of a compatible pure Rust implementation of librsync
(http://librsync.sourcefrog.net/).

The plan is to provide this in four layers:
  * [`librsync-impl-rs`](https://github.com/sourcefrog/librsync-impl-rs):
    pure Rust, non-blocking state machine, on buffers provided by caller
    * Build just with Cargo.
    * Unit tests in Rust.
    * Cargo crate name `librsync_impl`.

  * [`rdiff-rs`](https://github.com/sourcefrog/rdiff-rs):
    a pure Rust binary based on `librsync-impl-rs`.
    * Cargo crate `rdiff`.

  * [`librsync-capi-rs`](https://github.com/sourcefrog/librsync-capi-rs):
    A C API backed by the Rust implementation.
    * The same as the existing librsync C API except where there are reasons
      to clean it up.
    * Includes a C header file, forked from the C API's `librsync.h`.
    * Build with Cargo(?)
    * Includes tests, in C, for the wrapper and implementation.

  * `librsync-crosstest`: check interoperability
    * They produce the exact same output (however, deltas could be better.)
    * Can consume each other's output.

More plans: https://github.com/sourcefrog/librsync-impl-rs/wiki
