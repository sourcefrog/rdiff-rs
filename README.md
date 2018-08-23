# librdiff-rs [pre-alpha]

This is the start of a compatible pure Rust implementation of librsync.

To reduce confusion with the `rsync` tool and with the C implementation, this
is called `librdiff` but the formats, capabilities and API will be the same.

The plan is to provide this in four layers:

* `lib`
  (crate `librsync`):
  librsync algorithm and format implemented in pure Rust, including
  unit tests.

* `bin/rdiff`
    a pure Rust binary based on `librsync-rs`, providing an
    `rdiff` command line tool similar to and compatible in
    format and command line syntax with
    `rdiff` from C librsync.

* `lib/capi`:
    A C API backed by the Rust implementation.
  * The same as the existing librsync C API except where this is
    infeasible or the existing API is very problematic.
  * Includes a C header file, forked from the C API's `librsync.h`.
  * Build with Cargo(?)
  * Includes tests, in C, for the wrapper and implementation.

* `crosstest`: check interoperability between C and Rust librsync:
  * They produce the exact same signatures, the deltas apply to
    produce the same results, and they can consume each other's
    deltas.

More plans: <https://github.com/sourcefrog/librsync-rs/wiki>
