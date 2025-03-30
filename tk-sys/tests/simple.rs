use std::ffi::CString;

fn convert_str(s: &str) -> *const std::ffi::c_char {
    s.as_ptr() as *const std::ffi::c_char
}

#[test]
fn test_basic() {
    unsafe {
        let interp = tk_sys::Tcl_CreateInterp();
        assert!(!interp.is_null());
        tk_sys::Tcl_DeleteInterp(interp);
    }
}

#[test]
fn test_mem() {
    unsafe {
        let interp = tk_sys::Tcl_CreateInterp();
        let ptr = tk_sys::Tcl_Alloc(100);
        assert!(!ptr.is_null());
        tk_sys::Tcl_Free(ptr);
        let mut value = *tk_sys::Tcl_NewStringObj(convert_str("true"), -1);
        tk_sys::Tcl_DecrRefCount(&mut value);
        tk_sys::Tcl_DeleteInterp(interp);
    }
}
