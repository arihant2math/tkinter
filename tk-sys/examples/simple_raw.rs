use std::ffi::CStr;
use std::process::exit;
use tk_sys::*;
use tcl_sys::*;

fn main() {
    unsafe {
        // Create a new Tcl interpreter
        let interp = Tcl_CreateInterp();

        // Initialize Tcl
        if Tcl_Init(interp) == TCL_ERROR as i32 {
            let err = CStr::from_ptr(Tcl_GetStringResult(interp));
            eprintln!("Tcl_Init error: {}", err.to_string_lossy());
            exit(1);
        }

        // Initialize Tk
        if Tk_Init(interp) == TCL_ERROR as i32 {
            let err = CStr::from_ptr(Tcl_GetStringResult(interp));
            eprintln!("Tk_Init error: {}", err.to_string_lossy());
            exit(1);
        }

        // Create the main application window
        let main_window = Tk_MainWindow(interp);
        if main_window.is_null() {
            let err = CStr::from_ptr(Tcl_GetStringResult(interp));
            eprintln!("Failed to create main window: {}", err.to_string_lossy());
            exit(1);
        }

        // Set the application name
        let app_name = b"SimpleTkApp\0".as_ptr() as *const i8;
        Tk_SetAppName(main_window, app_name);

        // Create a button widget via Tcl scripting
        let create_btn = b"button .b -text \"Click Me\" -command {puts \"Hello from Tk\"}\0".as_ptr() as *const i8;
        Tcl_Eval(interp, create_btn);

        // Pack the button into the window
        let pack_btn = b"pack .b\0".as_ptr() as *const i8;
        Tcl_Eval(interp, pack_btn);

        // Enter the Tk event loop
        Tk_MainLoop();
    }
}
