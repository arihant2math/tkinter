/// Decrement the reference count of a Tcl object.
/// If the reference count reaches zero, the object is freed.
/// 
/// # Safety
/// If the reference count is decremented to zero, the object is freed and `objPtr` turns into a null pointer.
/// If objPtr is null, UB occurs.
#[inline(always)]
pub unsafe fn Tcl_DecrRefCount(objPtr: *mut Tcl_Obj) {
    let mut obj = unsafe { *objPtr };
    obj.refCount -= 1;
    if obj.refCount == 0 {
        // Free the object
        unsafe { Tcl_Free(objPtr as _) };
    }
}

/// Increment the reference count of a Tcl object.
/// This is used to indicate that the object is being used and should not be freed.
/// 
/// # Safety
/// If [`Tcl_DecrRefCount`] is not called and the pointer is dropped, the object will be leaked.
/// If objPtr is null, UB occurs.
#[inline(always)]
pub unsafe fn Tcl_IncrRefCount(objPtr: *mut Tcl_Obj) {
    let mut obj = unsafe { *objPtr };
    obj.refCount += 1;
}
