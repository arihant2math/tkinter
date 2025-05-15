pub trait InterpExt {
    fn tk_init(&self) -> i32;

    fn delete_image(&self, name: &str);
}

impl InterpExt for tcl::Interp {
    fn tk_init(&self) -> i32 {
        unsafe {
            tk_sys::Tk_Init(self.as_ptr()) as i32
        }
    }

    fn delete_image(&self, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            tk_sys::Tk_DeleteImage(self.as_ptr(), name.as_ptr());
        }
    }
}


