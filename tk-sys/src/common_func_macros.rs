fn Tcl_DecrRefCount(objPtr: *mut Tcl_Obj) {
    let mut obj = unsafe { *objPtr };
    obj.refCount -= 1;
    if obj.refCount == 0 {
        // Free the object
        unsafe { Tcl_Free(objPtr as *mut std::ffi::c_char) };
    }
}

fn Tcl_IncrRefCount(objPtr: *mut Tcl_Obj) {
    let mut obj = unsafe { *objPtr };
    obj.refCount += 1;
}
