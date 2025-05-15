use std::ptr::NonNull;

pub struct Condition(pub(crate) NonNull<tcl_sys::Tcl_Condition>);

impl Condition {
    pub unsafe fn from_raw(condition: *mut tcl_sys::Tcl_Condition) -> Self {
        Condition(NonNull::new(condition).expect("Failed to create Tcl_Condition from raw pointer"))
    }

    pub unsafe fn as_ptr(&self) -> *mut tcl_sys::Tcl_Condition {
        self.0.as_ptr()
    }

    pub unsafe fn notify(&self) {
        unsafe {
            tcl_sys::Tcl_ConditionNotify(self.0.as_ptr());
        }
    }

    pub unsafe fn wait(&self, mutex: crate::Mutex, time: &crate::Time) {
        unsafe {
            tcl_sys::Tcl_ConditionWait(self.0.as_ptr(), mutex.as_ptr(), time.as_ptr());
        }
    }

    pub unsafe fn finalize(self) {
        unsafe {
            tcl_sys::Tcl_ConditionFinalize(self.as_ptr());
        }
    }
}
