# librdiff-capi-rs [pre-alpha]

This is a C API binding for the [Rust librdiff library](https://github.com/sourcefrog/librdiff-rs).

The intention is to be fairly compatible with the existing librsync public API
([`librsync.h`](https://github.com/librsync/librsync/blob/master/src/librsync.h)).

However total compatibility is not guaranteed: API warts may be removed,
and some things may be impractical to map to Rust.

The `ctests` subdirectory contains tests written in C that exercise the C API.
Run them with

```shell
make -C ctests check
```
