use std::ffi::{CStr, CString};

fn main() {
    // readline
    let mut var1 = String::new();
    std::io::stdin()
        .read_line(&mut var1)
        .expect("Failed to read line");
    let mut var2 = String::new();
    std::io::stdin()
        .read_line(&mut var2)
        .expect("Failed to read line");
    let var1 = var1.trim();
    let var2 = var2.trim();
    println!("var1: {}", var1);
    println!("var2: {}", var2);
    let var1 = CString::new(var1).unwrap();
    let var2 = CString::new(var2).unwrap();
    let var2 = if var2.is_empty() {
        std::ptr::null()
    } else {
        var2.as_ptr()
    };
    unsafe {
        let interp = tcl_sys::Tcl_CreateInterp();
        tcl_sys::Tcl_Init(interp);
        let out = tcl_sys::Tcl_GetVar(interp, var1.as_ptr() as _, tcl_sys::TCL_LEAVE_ERR_MSG as _);
        if out.is_null() {
            let err_obj = tcl_sys::Tcl_GetObjResult(interp);
            let err_msg = tcl_sys::Tcl_GetString(err_obj);
            let err = CStr::from_ptr(err_msg as _);
            println!("Variable not found: {err:?}");
        } else {
            println!("Variable found");
            let converted_string = CStr::from_ptr(out as _);
            println!("{converted_string:?}");
        }
        let res = tcl_sys::Tcl_GetVar2Ex(interp, var1.as_ptr() as _, var2 as _, tcl_sys::TCL_LEAVE_ERR_MSG as _);
        if res.is_null() {
            let err_obj = tcl_sys::Tcl_GetObjResult(interp);
            let err_msg = tcl_sys::Tcl_GetString(err_obj);
            let err = CStr::from_ptr(err_msg as _);
            println!("Variable not found: {err:?}");
        } else {
            println!("Variable found");
            let string = tcl_sys::Tcl_GetString(res);
            let converted_string = CStr::from_ptr(string as _);
            println!("{converted_string:?}");
        }
    }
}
