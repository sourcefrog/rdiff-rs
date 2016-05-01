use std::env;

pub fn main() {
    // Make the OUT_DIR from build time available at run time.
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=profile={:?}", profile);
    }
}
