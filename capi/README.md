# librdiff-capi-rs [pre-alpha]

This is a C API binding for the
[Rust librdiff library](https://github.com/sourcefrog/librdiff-rs).

The intention is to be fairly compatible with the existing librsync public API
([`librsync.h`](https://github.com/librsync/librsync/blob/master/src/librsync.h)).

However total compatibility is not guaranteed: API warts may be removed,
and some things may be impractical to map to Rust.

## Building

`cargo build` should be enough to produce a
`librdiff.a` static library in `target/debug/`.

For an optimized build use `cargo build --release` and the output will be in
`target/release/`.

## Testing

`cargo test` will build and run tests written in C that exercise the C API.

## Changes vs librsync 2.0

* The library version is available through `rs_version()` rather than
  `rs_librsync_version[]`.
