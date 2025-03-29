#[test]
fn test_basic() {
    unsafe {
        let interp = tk_sys::Tcl_CreateInterp();
        assert!(!interp.is_null());
        tk_sys::Tcl_DeleteInterp(interp);
    }
}