extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::path::PathBuf;

fn main() {
    match pkg_config::find_library("heartbeats-simple") {
        Ok(_) => (),
        Err(_) => {
            let src = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/heartbeats-simple"));
            let mut config = Config::new(&src);
            let dst: PathBuf = config
                // .define("CMAKE_AR",
                //         "/Users/cong/ndk-standalone-18-arm/bin/arm-linux-androideabi-ar")
                .define("CMAKE_RANLIB",
                        "/Users/cong/ndk-standalone-18-arm/bin/arm-linux-androideabi-ranlib")
                .build();
            println!("cargo:rustc-link-lib=static=heartbeats-simple");
            println!("cargo:rustc-link-search=native={}",
                     dst.join("lib").display());
        }
    }
}
