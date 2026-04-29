fn main() {
    // 1. ask llvm-config for everything needed
    let llvm_config = std::env::var("LLVM_CONFIG")
        .unwrap_or_else(|_| "llvm-config".to_string());

    let cxxflags = std::process::Command::new(&llvm_config)
        .arg("--cxxflags")
        .output()
        .expect("llvm-config not found")
        .stdout;
    let cxxflags = String::from_utf8(cxxflags).unwrap();

    let ldflags = std::process::Command::new(&llvm_config)
        .args(["--ldflags", "--libs", "core", "orcjit", "native", "passes"])
        .output()
        .expect("llvm-config failed")
        .stdout;
    let ldflags = String::from_utf8(ldflags).unwrap();

    // 2. cxx bridge compile
    let mut build = cxx_build::bridge("src/codegen.rs"); // or ffi.rs
    build.file("src-cpp/llvm_bridge.cpp")
         .std("c++17");

    // pass llvm cxxflags to cc::Build
    for flag in cxxflags.split_whitespace() {
        build.flag_if_supported(flag);
    }
    build.compile("quetzal-llvm");

    // 3. link flags → cargo
    for flag in ldflags.split_whitespace() {
        if let Some(lib) = flag.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={lib}");
        } else if let Some(dir) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={dir}");
        }
    }

    // 4. rerun triggers
    println!("cargo:rerun-if-changed=src-cpp/llvm_bridge.cpp");
    println!("cargo:rerun-if-changed=src-cpp/llvm_bridge.hpp");
    println!("cargo:rerun-if-changed=src/codegen.rs");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG");
}
