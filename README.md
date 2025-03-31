This project provides raw API bindings to Tcl/Tk.

# Installation

## Windows
Just add the crate, binaries are bundled with the installation.

## MacOS/ Linux
Install `tk-dev` and `pkgconfig` with your package manager.

apt:
```
sudo apt install tk-dev pkg-config
```
or brew:
```
brew install tcl-tk pkg-config
```

then add the crate.

## Debugging install
If not on windows try running `pkg-config --cflags --libs tk`

The output should be something like:

`-I/usr/include/tcl8.6 -ltk8.6 -ltkstub8.6 -ltcl8.6 -ltclstub8.6`

if it is something like

```
Package tk was not found in the pkg-config search path.
Perhaps you should add the directory containing `tk.pc'
to the PKG_CONFIG_PATH environment variable
No package 'tk' found
```

then tk/tcl has not properly been installed.

# License

Under Apache License 2.0 or MIT License, at your will.
