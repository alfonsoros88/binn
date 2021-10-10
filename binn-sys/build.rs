fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("failed to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings");

    cc::Build::new()
        .file("binn/src/binn.c")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-implicit-fallthrough")
        .compile("libbinn.a");
}
