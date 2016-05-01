use std::env;

pub fn main() {
    // Make the PROFILE from build time available at run time, so that C tests
    // can link against the debug or release library.
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=profile={:?}", profile);
    }
}
