use std::ptr::NonNull;

pub struct Mutex(pub(crate) NonNull<tcl_sys::Tcl_Mutex>);

impl Mutex {
    pub unsafe fn from_raw(mutex: *mut tcl_sys::Tcl_Mutex) -> Self {
        Mutex(NonNull::new(mutex).expect("Failed to create Tcl_Mutex from raw pointer"))
    }

    pub unsafe fn as_ptr(&self) -> *mut tcl_sys::Tcl_Mutex {
        self.0.as_ptr()
    }

    pub unsafe fn lock(&self) {
        unsafe {
            tcl_sys::Tcl_MutexLock(self.0.as_ptr());
        }
    }

    pub unsafe fn unlock(&self) {
        unsafe {
            tcl_sys::Tcl_MutexUnlock(self.0.as_ptr());
        }
    }

    pub unsafe fn finalize(self) {
        unsafe {
            tcl_sys::Tcl_MutexFinalize(self.as_ptr());
        }
    }
}
