fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,./lib");
    // println!("cargo:rustc-link-arg=-Wl,-rpath,/home/priyanshu/Dev/aftershoot_task/lib");
}
