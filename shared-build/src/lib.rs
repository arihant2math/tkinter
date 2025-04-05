use std::env;
use std::path::PathBuf;

#[macro_export]
macro_rules! combine {
    ($A:expr, $B:expr) => {{
        const A: &str = $A;
        const B: &str = $B;
        const LEN: usize = A.len() + B.len();
        const fn combined() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = copy_slice(A.as_bytes(), out, 0);
            out = copy_slice(B.as_bytes(), out, A.len());
            out
        }
        const fn copy_slice(input: &[u8], mut output: [u8; LEN], offset: usize) -> [u8; LEN] {
            let mut index = 0;
            loop {
                output[offset + index] = input[index];
                index += 1;
                if index == input.len() {
                    break;
                }
            }
            output
        }
        const RESULT: &[u8] = &combined();
        // how bad is the assumption that `&str` and `&[u8]` have the same layout?
        const RESULT_STR: &str = unsafe { std::mem::transmute(RESULT) };
        RESULT_STR
    }};
}

pub fn build(
    wrapper_file: &str,
    get_lib_dirs: fn() -> Vec<String>,
    get_include_dirs: fn() -> Vec<String>,
    get_libs: fn() -> Vec<String>,
) {
    println!("cargo:rerun-if-changed={wrapper_file}");
    println!("cargo:rerun-if-changed=build.rs");

    for dir in get_lib_dirs() {
        println!("cargo:rustc-link-search={dir}", dir = dir);
    }

    for link in get_libs() {
        println!("cargo:rustc-link-lib={link}");
    }

    let mut builder = bindgen::Builder::default()
        .header(wrapper_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("Tcl_DecrRefCount")
        .blocklist_function("Tcl_IncrRefCount");
    for dir in get_include_dirs() {
        builder = builder.clang_arg(format!("-I{dir}"));
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let custom_path = PathBuf::from("../custom.rs");
    let custom_file = std::fs::read_to_string(custom_path).expect("Unable to read custom file");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let custom_file_out = out_path.join("custom.rs");
    std::fs::write(custom_file_out, custom_file).expect("Unable to write custom file");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
