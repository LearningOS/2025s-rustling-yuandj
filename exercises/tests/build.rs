//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.


use std::time::{SystemTime, UNIX_EPOCH};


// build.rs 文件内容：
fn main() {
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
