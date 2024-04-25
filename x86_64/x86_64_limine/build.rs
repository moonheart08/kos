fn main() {
    println!("cargo:rustc-link-arg=-Tx86_64_limine/linker.ld");
    println!("cargo:rerun-if-changed=linker.ld");
}