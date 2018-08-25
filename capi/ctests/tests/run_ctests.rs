/// Call out to tests written in C, which will then exercise the rdiff-rs C API.

extern crate rdiff_capi;

extern "C" {
    fn version_test() -> bool;
}

#[test]
pub fn run_version_test() {
    unsafe {
        assert!(version_test());
    }
}
