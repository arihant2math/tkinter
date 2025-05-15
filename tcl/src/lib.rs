mod interp;
mod obj;
mod time;

pub use interp::Interp;
pub use obj::Obj;
pub use time::Time;

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
