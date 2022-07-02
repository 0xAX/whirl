fn main() {
    println!("cargo:rustc-link-search=/lib64");
    println!("cargo:rustc-link-lib=luajit-5.1");
}
