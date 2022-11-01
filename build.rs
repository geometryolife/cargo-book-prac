// Example custom build script
// fn main() {
// Tell Cargo that if the given file changes, to return this build script.
// println!("cargo:rerun-if-changed=src/hello.c");
// println!("cargo:warning=nice boy!");
// Use the `cc` crate to build a C file and statically link it.
// cc::Build::new().file("src/hello.c").compile("hello good");
// }

// Example in The Cargo Book
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        ",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
