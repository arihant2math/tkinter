use std::ptr::NonNull;

pub struct Obj(pub(crate) NonNull<tcl_sys::Tcl_Obj>);

impl Obj {
    pub fn new() -> Self {
        let obj = unsafe { tcl_sys::Tcl_NewObj() };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub unsafe fn from_raw(obj: *mut tcl_sys::Tcl_Obj) -> Self {
        Obj(NonNull::new(obj).expect("Failed to create Tcl_Obj from raw pointer"))
    }


    pub unsafe fn as_ptr(&self) -> *mut tcl_sys::Tcl_Obj {
        self.0.as_ptr()
    }

    pub fn from_string(string: &str) -> Self {
        let c_str = std::ffi::CString::new(string).expect("Failed to create CString");
        let obj = unsafe { tcl_sys::Tcl_NewStringObj(c_str.as_ptr(), c_str.as_bytes().len() as i32) };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn from_boolean(value: bool) -> Self {
        let obj = if value {
            unsafe { tcl_sys::Tcl_NewBooleanObj(1) }
        } else {
            unsafe { tcl_sys::Tcl_NewBooleanObj(0) }
        };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn from_int(value: i32) -> Self {
        let obj = unsafe { tcl_sys::Tcl_NewIntObj(value) };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn from_double(value: f64) -> Self {
        let obj = unsafe { tcl_sys::Tcl_NewDoubleObj(value) };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn from_long(value: i64) -> Self {
        let obj = unsafe { tcl_sys::Tcl_NewLongObj(value as _) };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn from_list(values: &[Obj]) -> Self {
        let obj = unsafe { tcl_sys::Tcl_NewListObj(values.len() as i32, values.as_ptr() as *const _) };
        unsafe {
            Self::from_raw(obj)
        }
    }

    pub fn get_string(&self) -> Option<String> {
        let c_str = unsafe { tcl_sys::Tcl_GetString(self.0.as_ptr()) };
        if c_str.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(c_str) }.to_string_lossy().into_owned())
        }
    }

    pub fn get_int(&self) -> Option<i32> {
        let mut value: i32 = 0;
        let result = unsafe { tcl_sys::Tcl_GetIntFromObj(std::ptr::null_mut(), self.0.as_ptr(), &mut value) };
        if result == 0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_double(&self) -> Option<f64> {
        let mut value: f64 = 0.0;
        let result = unsafe { tcl_sys::Tcl_GetDoubleFromObj(std::ptr::null_mut(), self.0.as_ptr(), &mut value) };
        if result == 0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_long(&self) -> Option<i64> {
        let mut value = 0 as _;
        let result = unsafe { tcl_sys::Tcl_GetLongFromObj(std::ptr::null_mut(), self.0.as_ptr(), &mut value) };
        if result == 0 {
            Some(value as _)
        } else {
            None
        }
    }

    pub fn get_list(&self) -> Option<Vec<Obj>> {
        let mut length: i32 = 0;
        let mut elements: *mut *mut tcl_sys::Tcl_Obj = std::ptr::null_mut();
        let result = unsafe { tcl_sys::Tcl_ListObjGetElements(std::ptr::null_mut(), self.0.as_ptr(), &mut length, &mut elements) };
        if result == 0 {
            let slice = unsafe { std::slice::from_raw_parts(elements, length as usize) };
            Some(slice.iter().map(|&obj| Obj(NonNull::new(obj).unwrap())).collect())
        } else {
            None
        }
    }
    pub fn get_boolean(&self) -> Option<bool> {
        let mut value: i32 = 0;
        let result = unsafe { tcl_sys::Tcl_GetBooleanFromObj(std::ptr::null_mut(), self.0.as_ptr(), &mut value) };
        if result == 0 {
            Some(value != 0)
        } else {
            None
        }
    }

    pub fn get_list_length(&self) -> Option<i32> {
        let mut length: i32 = 0;
        let result = unsafe { tcl_sys::Tcl_ListObjLength(std::ptr::null_mut(), self.0.as_ptr(), &mut length) };
        if result == 0 {
            Some(length)
        } else {
            None
        }
    }

    pub fn get_list_element(&self, index: i32) -> Option<Obj> {
        let mut element: *mut tcl_sys::Tcl_Obj = std::ptr::null_mut();
        let result = unsafe { tcl_sys::Tcl_ListObjIndex(std::ptr::null_mut(), self.0.as_ptr(), index, &mut element) };
        if result == 0 {
            Some(Obj(NonNull::new(element).unwrap()))
        } else {
            None
        }
    }

    pub fn set_string(&self, string: &str) {
        let c_str = std::ffi::CString::new(string).expect("Failed to create CString");
        unsafe { tcl_sys::Tcl_SetStringObj(self.0.as_ptr(), c_str.as_ptr(), c_str.as_bytes().len() as i32) };
    }

    pub fn set_int(&self, value: i32) {
        unsafe { tcl_sys::Tcl_SetIntObj(self.0.as_ptr(), value) };
    }

    pub fn set_double(&self, value: f64) {
        unsafe { tcl_sys::Tcl_SetDoubleObj(self.0.as_ptr(), value) };
    }

    pub fn set_long(&self, value: i64) {
        unsafe { tcl_sys::Tcl_SetLongObj(self.0.as_ptr(), value as _) };
    }

    pub fn set_list(&self, values: &[Obj]) {
        unsafe { tcl_sys::Tcl_SetListObj(self.0.as_ptr(), values.len() as i32, values.as_ptr() as *const _) };
    }

    pub fn set_boolean(&self, value: bool) {
        unsafe { tcl_sys::Tcl_SetBooleanObj(self.0.as_ptr(), value as _) };
    }
}

impl Clone for Obj {
    fn clone(&self) -> Self {
        unsafe {
            tcl_sys::Tcl_IncrRefCount(self.0.as_ptr());
        }
        Obj(self.0)
    }
}

impl Drop for Obj {
    fn drop(&mut self) {
        unsafe {
            tcl_sys::Tcl_DecrRefCount(self.0.as_ptr());
        }
    }
}
