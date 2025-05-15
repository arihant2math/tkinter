pub struct Time(pub tcl_sys::Tcl_Time);

impl Time {
    pub fn new(sec: i64, usec: i64) -> Self {
        let time = tcl_sys::Tcl_Time {
            sec: sec as _,
            usec: usec as _,
        };
        Time(time)
    }
}
