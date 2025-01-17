use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("./dependencies/KaHIP/deploy").exists() {
        //panic!("{}", Path::new("./dependencies/KaHIP/deploy").display());
        let status = Command::new("sh")
            .arg("dependencies/KaHIP/compile_withcmake.sh")
            .status()
            .unwrap_or_else(|e| {
                panic!("Failed to execute compilation script: {}", e);
            });

        if !status.success() {
            panic!("Failed to compile KaHIP");
        }
    }

    println!("cargo:rustc-link-search=native=./dependencies/KaHIP/deploy");
    println!("cargo:rustc-link-lib=kahip");
}
