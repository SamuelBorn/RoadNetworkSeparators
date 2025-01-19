use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("./dependencies/KaHIP/deploy").exists() {
        let status = Command::new("sh")
            .arg("dependencies/KaHIP/compile_withcmake.sh")
            .status()
            .unwrap_or_else(|_| {
                panic!("Failed to compile KaHIP");
            });

        if !status.success() {
            panic!("Failed to compile KaHIP");
        }
    }

    println!("cargo::rustc-env=LD_LIBRARY_PATH=./dependencies/KaHIP/deploy");
    println!("cargo:rustc-link-search=native=./dependencies/KaHIP/deploy");
    println!("cargo:rustc-link-lib=kahip");
}
