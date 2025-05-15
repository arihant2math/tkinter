
mod interp;
mod window;

pub use interp::InterpExt;
pub use window::Window;

pub use tk_sys as raw;
pub use tk_sys::Atom;

pub fn get_num_windows() -> i32 {
    unsafe {
        tk_sys::Tk_GetNumMainWindows() as i32
    }
}

pub fn main_loop() {
    unsafe {
        tk_sys::Tk_MainLoop();
    }
}
