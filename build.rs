fn main() {
    println!("cargo:rustc-link-search=native=../KaHIP/deploy");
    println!("cargo:rustc-link-lib=kahip");
}

