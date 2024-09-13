fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("Phoenix5-Linux-Example/include"); // include path
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path])
        .extra_clang_args(&["-std=c++17"])
        .build()?;
        // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++17")
     .compile("invibratrac"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");

    if cfg!(target_arch = "x86_64") {
        println!("cargo:rustc-link-search=./Phoenix5-Linux-Example/lib/x86-64");
    } else if cfg!(target_arch = "aarch64") {
        println!("cargo:rustc-link-search=./Phoenix5-Linux-Example/lib/arm64");
    } else if cfg!(target_arch = "arm") {
        println!("cargo:rustc-link-search=./Phoenix5-Linux-Example/lib/arm32");
    }

    println!("cargo:rustc-link-lib=CTRE_Phoenix");
    println!("cargo:rustc-link-lib=CTRE_PhoenixCCI");
    println!("cargo:rustc-link-lib=CTRE_PhoenixTools");
    Ok(())
}