use std::ptr::NonNull;
use crate::Obj;

pub struct Interp(pub(crate) NonNull<tcl_sys::Tcl_Interp>);

impl Interp {
    pub fn new() -> Self {
        let interp = unsafe { tcl_sys::Tcl_CreateInterp() };
        unsafe {
            Interp::from_raw(interp)
        }
    }

    pub unsafe fn from_raw(interp: *mut tcl_sys::Tcl_Interp) -> Self {
        Interp(NonNull::new(interp).unwrap())
    }

    pub fn create_slave(&mut self, name: &str, is_safe: i32) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let interp = unsafe {
            tcl_sys::Tcl_CreateSlave(self.0.as_ptr(), name.as_ptr(), is_safe as _)
        };
        unsafe {
            Interp::from_raw(interp)
        }
    }

    pub fn get_slave(&mut self, name: &str) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let interp = unsafe {
            tcl_sys::Tcl_GetSlave(self.0.as_ptr(), name.as_ptr())
        };
        unsafe {
            Interp::from_raw(interp)
        }
    }

    pub unsafe fn get_obj_result(&self) -> Obj {
        Obj::from_raw(tcl_sys::Tcl_GetObjResult(self.0.as_ptr()))
    }

    pub unsafe fn eval(&self, script: &str) -> i32 {
        let script = std::ffi::CString::new(script).unwrap();
        tcl_sys::Tcl_Eval(self.0.as_ptr(), script.as_ptr()) as i32
    }

    pub unsafe fn eval_ex(&self, script: &str, num_bytes: i32, flags: i32) -> i32 {
        let script = std::ffi::CString::new(script).unwrap();
        tcl_sys::Tcl_EvalEx(self.0.as_ptr(), script.as_ptr(), num_bytes, flags) as i32
    }

    pub unsafe fn eval_obj(&self, obj: Obj) -> i32 {
        tcl_sys::Tcl_EvalObj(self.0.as_ptr(), obj.0.as_ptr()) as i32
    }

    pub unsafe fn eval_file(&self, filename: &str) -> i32 {
        let filename = std::ffi::CString::new(filename).unwrap();
        tcl_sys::Tcl_EvalFile(self.0.as_ptr(), filename.as_ptr()) as i32
    }
}

impl Drop for Interp {
    fn drop(&mut self) {
        unsafe {
            tcl_sys::Tcl_DeleteInterp(self.0.as_ptr());
        }
    }
}
