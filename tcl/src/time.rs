pub struct Time(pub tcl_sys::Tcl_Time);

impl Time {
    pub fn new(sec: i64, usec: i64) -> Self {
        let time = tcl_sys::Tcl_Time {
            sec: sec as _,
            usec: usec as _,
        };
        Time(time)
    }

    pub fn into_raw(self) -> tcl_sys::Tcl_Time {
        self.0
    }

    pub fn as_ptr(&self) -> *const tcl_sys::Tcl_Time {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut tcl_sys::Tcl_Time {
        &mut self.0
    }
}
