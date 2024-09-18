fn main() -> miette::Result<()> {
    autocxx_build::Builder::new("src/main.rs", &["/usr/include/phoenix6/", "src/"])
        .extra_clang_args(&["-std=c++17"])
        .build()?
        .flag_if_supported("-std=c++17")
        .compile("invibratrac");    
    println!("cargo:rerun-if-changed=src/main.rs");

    println!("cargo:rustc-link-search=/usr/lib/phoenix6/");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/phoenix6/");

    println!("cargo:rustc-link-lib=CTRE_Phoenix6");
    println!("cargo:rustc-link-lib=CTRE_PhoenixTools");

    Ok(())
}