mod condition;
mod interp;
mod mutex;
mod obj;
mod obj_type;
mod time;

pub use condition::Condition;
pub use interp::Interp;
pub use mutex::Mutex;
pub use obj::Obj;
pub use obj_type::ObjType;
pub use time::Time;

pub use tcl_sys as raw;
pub use tcl_sys::TCL_ERROR;
pub use tcl_sys::TCL_OK;
pub use tcl_sys::Tcl_ThreadId_;

pub fn get_errno() -> i32 {
    unsafe {
        tcl_sys::Tcl_GetErrno() as _
    }
}

pub fn set_errno(errno: i32) {
    unsafe {
        tcl_sys::Tcl_SetErrno(errno as _);
    }
}

pub fn get_current_thread() -> Tcl_ThreadId_ {
    unsafe {
        *tcl_sys::Tcl_GetCurrentThread()
    }
}

pub unsafe fn exit(code: i32) -> ! {
    unsafe {
        tcl_sys::Tcl_Exit(code as _);
    }
}
