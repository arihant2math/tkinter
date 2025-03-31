use std::env;
use std::path::PathBuf;

#[allow(clippy::unused)]
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

    pub fn get_libs() -> Vec<String> {
        vec![
            "tcl86t".to_string(),
            "tclstub86".to_string(),
        ]
    }

    pub fn get_lib_dirs() -> Vec<String> {
        vec![BIN_DIR.to_string(), LIB_DIR.to_string()]
    }

    pub fn get_include_dirs() -> Vec<String> {
        vec![INCLUDE_DIR.to_string()]
    }
}

#[cfg(not(target_os = "windows"))]
mod os {
    fn get_config() -> pkg_config::Library {
        pkg_config::Config::new()
            .atleast_version("8.6")
            .probe("tcl")
            .unwrap()
    }

    pub fn get_libs() -> Vec<String> {
        get_config().libs
    }

    pub fn get_lib_dirs() -> Vec<String> {
        get_config()
            .link_paths
            .into_iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect()
    }

    pub fn get_include_dirs() -> Vec<String> {
        get_config()
            .include_paths
            .into_iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect()
    }
}

use os::{get_include_dirs, get_lib_dirs, get_libs};

fn main() {
    println!("cargo:rerun-if-changed={WRAPPER_FILE}");
    println!("cargo:rerun-if-changed=build.rs");

    for dir in get_lib_dirs() {
        println!("cargo:rustc-link-search={dir}", dir = dir);
    }

    for link in get_libs() {
        println!("cargo:rustc-link-lib={link}");
    }

    let mut builder = bindgen::Builder::default()
        .header(WRAPPER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("Tcl_DecrRefCount")
        .blocklist_function("Tcl_IncrRefCount");
    for dir in get_include_dirs() {
        builder = builder.clang_arg(format!("-I{dir}"));
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
