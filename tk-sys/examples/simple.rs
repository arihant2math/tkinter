fn main() {
    unsafe {
        let interp = tcl_sys::Tcl_CreateInterp();
        tk_sys::Tk_Init(interp);
        let window = tk_sys::Tk_MainWindow(interp);
        tk_sys::Tk_SetAppName(window, b"simple\0".as_ptr() as _);
        tcl_sys::Tcl_Eval(interp, b"button .b -text \"Click Me\" -command {puts \"Hello from Tk\"}\0".as_ptr() as _);
        tcl_sys::Tcl_Eval(interp, b"pack .b\0".as_ptr() as _);

        // Start the main event loop
        tk_sys::Tk_MainLoop();
    }
}