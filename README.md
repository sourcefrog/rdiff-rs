# librdiff-rs [pre-alpha]

This is the start of a compatible pure Rust implementation of librsync.

To reduce confusion with the `rsync` tool and with the C implementation, this
is called `librdiff` but the formats, capabilities and API will be the same.

The plan is to provide this in four layers:

* `src/`
  (crate `librdiff`):
  librsync algorithm and format implemented in pure Rust, including
  unit tests.

* `src/bin/rdiff`
    a pure Rust binary based on `librsync-rs`, providing an
    `rdiff` command line tool similar to and compatible in
    format and command line syntax with
    `rdiff` from C librsync.

* `capi/`, crate `librdiff-capi`:
    A C API backed by the Rust implementation.
  * The same as the existing librsync C API except where this is
    infeasible or the existing API is very problematic.
  * Includes a C header file, forked from the C API's `librsync.h`.
  * Build with Cargo(?)
  * Includes tests, in C, for the wrapper and implementation.

* `src/bin/crosstest`: check interoperability between C and Rust
    versions of `rdiff`:
  1. They produce the exact same signatures
  2. They can consume each others' deltas to produce the same resulting
     new file.
  3. (It's not required they produce the exact same delta.)

More plans: <https://github.com/sourcefrog/librsync-rs/wiki>
