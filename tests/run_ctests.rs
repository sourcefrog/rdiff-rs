// Thanks to Zarathustra30 in https://stackoverflow.com/a/31760328/243712

// librdiff_capi is not actually used here, but linking it in seems the simplest
// way to add a dependency and make sure it's ready to be linked into the C tests.
extern crate librdiff_capi;

use std::process::Command;

/// Build and run the tests written in C that exercise the Rust->C bindings.
#[test]
pub fn run_ctests() {
    // TODO: Maybe automatically use the release directory if doing a release
    // build? Seems a bit complicated.
    // https://users.rust-lang.org/t/conditional-compilation-for-debug-release/1098/7?u=sourcefrog

    match Command::new("make")
        .args(&["-C", "ctests", "check"])
        .status() {
        Ok(status) if status.success() => (),
        Err(e) => panic!("failed to run ctests: {}", e),
        Ok(rc) => panic!("ctests failed with status: {}", rc),
    }
}
