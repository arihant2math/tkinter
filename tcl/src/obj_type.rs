pub struct ObjType(pub(crate) tcl_sys::Tcl_ObjType);

impl ObjType {
    pub fn from_raw(obj_type: *const tcl_sys::Tcl_ObjType) -> Self {
        ObjType(unsafe { *obj_type })
    }

    pub fn as_ptr(&self) -> *const tcl_sys::Tcl_ObjType {
        &self.0
    }

    pub fn get(name: &str) -> Option<Self> {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let obj_type = tcl_sys::Tcl_GetObjType(name.as_ptr());
            if obj_type.is_null() {
                return None;
            }
            Some(ObjType(*obj_type))
        }
    }
}
