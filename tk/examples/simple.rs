use std::process::exit;
use tcl::*;
use tk::InterpExt;

fn main() {
    unsafe {
        // Create a new Tcl interpreter
        let mut interp = Interp::new();

        // Initialize Tcl
        if interp.init() == TCL_ERROR as i32 {
            let err = interp.get_string_result();
            eprintln!("Tcl_Init error: {}", err);
            exit(1);
        }

        // Initialize Tk
        if interp.tk_init() == TCL_ERROR as i32 {
            let err = interp.get_string_result();
            eprintln!("Tk_Init error: {}", err);
            exit(1);
        }

        // Create the main application window
        let mut main_window = tk::Window::main_window(&interp);
        if main_window.is_none() {
            let err = interp.get_string_result();
            eprintln!("Failed to create main window: {}", err);
            exit(1);
        }
        let mut main_window = main_window.unwrap();

        // Set the application name
        main_window.set_app_name("SimpleTkApp");

        // Create a button widget via Tcl scripting
        interp.eval("button .b -text \"Click Me\" -command {puts \"Hello from Tk\"}");

        // Pack the button into the window
        interp.eval("pack .b");

        // Enter the Tk event loop
        tk::main_loop();
    }
}
