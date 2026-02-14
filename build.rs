fn main() {
    let python_feature_enabled = std::env::var_os("CARGO_FEATURE_PYTHON").is_some();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Ensure Python extension-style linking on macOS for cdylib builds.
    // This avoids requiring explicit libpython linkage during test/build flows.
    if python_feature_enabled && target_os == "macos" {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}
