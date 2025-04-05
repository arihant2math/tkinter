const WRAPPER_FILE: &str = "wrapper.h";

#[cfg(target_os = "windows")]
mod os {
    use std::path::PathBuf;

    use shared_build::combine;

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
            "tk86t".to_string(),
            "tkstub86".to_string(),
            "tcl86t".to_string(),
            "tclstub86".to_string(),
        ]
    }

    pub fn get_lib_dirs() -> Vec<String> {
        // Create absolute paths to the lib and bin directories
        let lib_dir = PathBuf::from(LIB_DIR);
        let bin_dir = PathBuf::from(BIN_DIR);
        let lib_dir = lib_dir
            .canonicalize()
            .expect("Failed to canonicalize lib directory");
        let bin_dir = bin_dir
            .canonicalize()
            .expect("Failed to canonicalize bin directory");
        vec![
            lib_dir.to_string_lossy().to_string(),
            bin_dir.to_string_lossy().to_string(),
        ]
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
            .probe("tk")
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
    shared_build::build(WRAPPER_FILE, get_lib_dirs, get_include_dirs, get_libs);
}
