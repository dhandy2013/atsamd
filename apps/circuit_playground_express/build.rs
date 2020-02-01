// Build script to assemble lowlevel.s and link it with the Rust lib

fn main() {
    cc::Build::new()
        .file("src/asmhelpers.s")
        .file("src/chelpers.c")
        .compile("lowlevel");
    println!("cargo:rerun-if-changed=src/asmhelpers.s");
    println!("cargo:rerun-if-changed=src/chelpers.c");
}
