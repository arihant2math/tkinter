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

    pub unsafe fn as_ptr(&self) -> *mut tcl_sys::Tcl_Interp {
        self.0.as_ptr()
    }

    pub fn init(&mut self) -> i32 {
        unsafe {
            tcl_sys::Tcl_Init(self.0.as_ptr()) as i32
        }
    }

    // pub fn app_init(&mut self) -> i32 {
    //     unsafe {
    //         tcl_sys::Tcl_AppInit(self.0.as_ptr()) as i32
    //     }
    // }

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
        unsafe {
            Obj::from_raw(tcl_sys::Tcl_GetObjResult(self.0.as_ptr()))
        }
    }

    pub unsafe fn get_string_result(&self) -> String {
        unsafe {
            let result = self.get_obj_result();
            let result_ptr = tcl_sys::Tcl_GetString(result.0.as_ptr());
            let _ = tcl_sys::Tcl_GetStringResult(self.0.as_ptr());
            let result_str = std::ffi::CStr::from_ptr(result_ptr);
            result_str.to_string_lossy().into_owned()
        }
    }

    pub unsafe fn reset_result(&self) {
        unsafe {
            tcl_sys::Tcl_ResetResult(self.0.as_ptr());
        }
    }

    pub unsafe fn eval(&self, script: &str) -> i32 {
        let script = std::ffi::CString::new(script).unwrap();
        unsafe {
            tcl_sys::Tcl_Eval(self.0.as_ptr(), script.as_ptr()) as i32
        }
    }

    pub unsafe fn eval_ex(&self, script: &str, num_bytes: i32, flags: i32) -> i32 {
        let script = std::ffi::CString::new(script).unwrap();
        unsafe {
            tcl_sys::Tcl_EvalEx(self.0.as_ptr(), script.as_ptr(), num_bytes, flags) as i32
        }
    }

    pub unsafe fn eval_obj(&self, obj: Obj) -> i32 {
        unsafe {
            tcl_sys::Tcl_EvalObj(self.0.as_ptr(), obj.0.as_ptr()) as i32
        }
    }

    pub unsafe fn eval_file(&self, filename: &str) -> i32 {
        let filename = std::ffi::CString::new(filename).unwrap();
        unsafe {
            tcl_sys::Tcl_EvalFile(self.0.as_ptr(), filename.as_ptr()) as i32
        }
    }

    pub unsafe fn expr_string(&self, expr: &str) -> i32 {
        let expr = std::ffi::CString::new(expr).unwrap();
        unsafe {
            tcl_sys::Tcl_ExprString(self.0.as_ptr(), expr.as_ptr()) as i32
        }
    }

    pub unsafe fn expr_obj(&self, obj: Obj) -> i32 {
        todo!()
    }

    pub fn expr_long(&self, expr: &str) -> Option<i64> {
        todo!()
    }

    pub fn expr_double(&self, expr: &str) -> Option<f64> {
        let expr = std::ffi::CString::new(expr).unwrap();
        let mut value: f64 = 0.0;
        let result = unsafe {
            tcl_sys::Tcl_ExprDouble(self.0.as_ptr(), expr.as_ptr(), &mut value)
        };
        if result == 0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_var(&self, name: &str, flags: i32) -> Option<String> {
        let name = std::ffi::CString::new(name).unwrap();
        let text = unsafe {
            tcl_sys::Tcl_GetVar(self.0.as_ptr(), name.as_ptr(), flags)
        };
        if text.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(text) }.to_string_lossy().into_owned())
        }
    }

    pub fn get_var2(&self, name1: &str, name2: &str, flags: i32) -> Option<String> {
        let name1 = std::ffi::CString::new(name1).unwrap();
        let name2 = std::ffi::CString::new(name2).unwrap();
        let text = unsafe {
            tcl_sys::Tcl_GetVar2(self.0.as_ptr(), name1.as_ptr(), name2.as_ptr(), flags)
        };
        if text.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(text) }.to_string_lossy().into_owned())
        }
    }

    pub fn get_var2_obj(&self, name1: &str, name2: &str, flags: i32) -> Option<Obj> {
        let name1 = std::ffi::CString::new(name1).unwrap();
        let name2 = std::ffi::CString::new(name2).unwrap();
        let obj = unsafe {
            tcl_sys::Tcl_GetVar2Ex(self.0.as_ptr(), name1.as_ptr(), name2.as_ptr(), flags)
        };
        if obj.is_null() {
            None
        } else {
            Some(unsafe { Obj::from_raw(obj) })
        }
    }

    pub fn set_var(&mut self, name: &str, value: &str, flags: i32) -> i32 {
        let name = std::ffi::CString::new(name).unwrap();
        let value = std::ffi::CString::new(value).unwrap();
        unsafe {
            tcl_sys::Tcl_SetVar(self.0.as_ptr(), name.as_ptr(), value.as_ptr(), flags) as i32
        }
    }

    pub fn set_var2(&mut self, name1: &str, name2: &str, value: &str, flags: i32) -> i32 {
        let name1 = std::ffi::CString::new(name1).unwrap();
        let name2 = std::ffi::CString::new(name2).unwrap();
        let value = std::ffi::CString::new(value).unwrap();
        unsafe {
            tcl_sys::Tcl_SetVar2(self.0.as_ptr(), name1.as_ptr(), name2.as_ptr(), value.as_ptr(), flags) as i32
        }
    }

    pub fn set_var2_obj(&mut self, name1: &str, name2: &str, obj: Obj, flags: i32) -> i32 {
        let name1 = std::ffi::CString::new(name1).unwrap();
        let name2 = std::ffi::CString::new(name2).unwrap();
        unsafe {
            tcl_sys::Tcl_SetVar2Ex(self.0.as_ptr(), name1.as_ptr(), name2.as_ptr(), obj.0.as_ptr(), flags) as i32
        }
    }
}

impl Drop for Interp {
    fn drop(&mut self) {
        unsafe {
            tcl_sys::Tcl_DeleteInterp(self.0.as_ptr());
        }
    }
}
