fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,./lib");
}
