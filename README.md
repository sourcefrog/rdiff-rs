# rdiff-rs [pre-alpha]

rdiff is a tool and library to generate binary deltas against a signature summarizing
the contents of an old file. The deltas can only be applied to the exact old file
contents from which they're generated.

This is the start of a pure Rust implementation of rdiff, both the command line
tool and the library. 
The original C implementation is at https://github.com/librsync/librsync.

rdiff uses the same algorithm, but not the same format, as rsync.

The plan is to provide this in four layers:

* `src/`
  (crate `rdiff`):
  rdiff algorithm and format implemented in pure Rust, including
  unit tests.

* `rdiff` binary from `src/main.rs`:
    a command line tool compatible in format and command line syntax with
    `rdiff` from C librsync.

* `capi/`, crate `rdiff-capi`:
    A C API backed by the Rust implementation.
  * The same as the existing librsync C API except where this is
    infeasible or the existing API is very problematic.
  * Includes a C header file, forked from the C API's `librsync.h`.
  * Build with Cargo(?)
  * Includes tests, in C, for the wrapper and implementation.

* `crosstest`: check interoperability between C and Rust
  versions of `rdiff`:
  1. They produce the exact same signatures
  2. They can consume each others' deltas to produce the same resulting
     new file.
  3. (It's not required they produce the exact same delta.)

More plans: <https://github.com/sourcefrog/rdiff-rs/wiki>

## License

Copyright 2015-2019 Martin Pool.

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.