use std::env;
use std::path::PathBuf;

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
                output[offset+index] = input[index];
                index += 1;
                if index == input.len() { break }
            }
            output
        }
        const RESULT: &[u8] = &combined();
        // how bad is the assumption that `&str` and `&[u8]` have the same layout?
        const RESULT_STR: &str = unsafe { std::mem::transmute(RESULT) };
        RESULT_STR
    }}
}

const WRAPPER_FILE: &str = "wrapper.h";

#[cfg(target_os = "windows")]
mod os {
    #[cfg(target_arch = "x86_64")]
    const ARCH_FOLDER: &str = "amd64";
    #[cfg(target_arch = "x86")]
    const ARCH_FOLDER: &str = "win32";
    #[cfg(target_arch = "aarch64")]
    const ARCH_FOLDER: &str = "arm64";
    const LIB_DIR: &str = combine!(combine!("../src/win/", ARCH_FOLDER), "/lib");
    const INCLUDE_DIR: &str = combine!(combine!("../src/win/", ARCH_FOLDER), "/include");
    const BIN_DIR: &str = combine!(combine!("../src/win/", ARCH_FOLDER), "/bin");
    pub fn get_bin_dir() -> String {
        BIN_DIR.to_string()
    }
    pub fn get_lib_dir() -> String {
        LIB_DIR.to_string()
    }
    pub fn get_include_dir() -> String {
        INCLUDE_DIR.to_string()
    }
}

#[cfg(target_os = "linux")]
mod os {
    
}


use os::{get_bin_dir, get_lib_dir, get_include_dir};



fn main() {
    println!("cargo:rerun-if-changed={WRAPPER_FILE}");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-link-search={bin_dir}", bin_dir = get_bin_dir());
    println!(r"cargo:rustc-link-search={lib_dir}", lib_dir = get_lib_dir());

    const LINKS: [&str; 4] = [
        "tk86t",
        "tkstub86",
        "tcl86t",
        "tclstub86",
    ];
    for link in LINKS.iter() {
        println!("cargo:rustc-link-lib={link}");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(WRAPPER_FILE)
        .clang_arg(format!("-I{}", get_include_dir()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("Tcl_DecrRefCount")
        .blocklist_function("Tcl_IncrRefCount")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
