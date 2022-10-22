// Example custom build script
fn main() {
    // Tell Cargo that if the given file changes, to return this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello good good good");
}
