# rdiff-rs TODO

* Parameterize signature generation by file format to support different strong and weak sums?

* Error type for this crate?

* Maybe accumulate stats about how much was read, written, etc.

## C API

* Fix compilation to work on Windows, preferably without Make. Use the C compiler crate?

* Make sure the Rust library is actually built before trying to run the C tests?