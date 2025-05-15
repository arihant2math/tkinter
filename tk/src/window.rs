pub struct Window(pub tk_sys::Tk_Window);

impl Window {
    pub fn main_window(interp: &tcl::Interp) -> Option<Self> {
        let window = unsafe { tk_sys::Tk_MainWindow(interp.as_ptr()) };

        if window.is_null() {
            return None;
        }
        unsafe {
            Some(Window::from_raw(window))
        }
    }

    pub unsafe fn from_raw(window: tk_sys::Tk_Window) -> Self {
        Window(window)
    }

    pub fn display_name(&self) -> String {
        let name = unsafe { tk_sys::Tk_DisplayName(self.0) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        name.to_string_lossy().into_owned()
    }

    pub fn set_app_name(&mut self, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            tk_sys::Tk_SetAppName(self.0, name.as_ptr());
        }
    }

    pub fn set_window_border(&mut self, pixel: u64) {
        unsafe {
            tk_sys::Tk_SetWindowBorder(self.0, pixel as _);
        }
    }

    pub fn set_window_border_width(&mut self, width: i32) {
        unsafe {
            tk_sys::Tk_SetWindowBorderWidth(self.0, width);
        }
    }
}
