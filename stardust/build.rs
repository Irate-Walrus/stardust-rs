use std::env;
use std::path::PathBuf;

fn main() {
    // Get necessary environment variables
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // Set linker arguments
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-fno-ident");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections,--build-id=none");
    println!("cargo:rustc-link-arg=-falign-labels=1");
    println!("cargo:rustc-link-arg=-Wall");
    println!("cargo:rustc-link-arg=-fno-asynchronous-unwind-tables");
    println!("cargo:rustc-link-arg=-Wl,-e_start");

    println!(
        "cargo:rustc-link-arg=-Wl,-Map={}",
        PathBuf::from(&out_dir).join("stardust.map").display()
    );
}
