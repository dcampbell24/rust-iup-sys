IUP-SYS Rust
=============

[![Build Status](https://travis-ci.org/dcampbell24/rust-iup-sys.svg)](https://travis-ci.org/dcampbell24/rust-iup-sys)
[![Crates.io](https://img.shields.io/crates/v/iup-sys.svg)](https://crates.io/crates/iup-sys)

These are low level Rust bindings to [IUP](http://webserver2.tecgraf.puc-rio.br/iup/).
Bindings to CD and IM are not included.

Installation
-------------

To use this library you must install IUP. Installing CD and/or IM is optional. See [download tips][2] for more information.

[2]: http://www.tecgraf.puc-rio.br/iup/en/download_tips.html

After installing IUP, add this crate as a dependecny in your Cargo.toml file:
``` toml
[dependencies]
iup-sys = "0.0"
```

License
--------

This project is licensed under the MIT license. See LICENSE for the full license
and LICENSE-TECGRAF for the license of the IUP library this library binds to.

build.rs
--------
It's assumed, that dynamic libraries are used for IUP (, CD, IM, FTGL, WebKit), i.e. .dll on Windows or .so on *nix,
otherwise the detection mechanism provided by build.rs won't work and build.rs must be removed (or replaced at Your discretion).

To make a long story short, the rationale of build.rs is: For supported versions You'll get from this IUP binding what You installed "binary-wise".

So far, this binding was a translation of iup.h only and referring to that, the sentence above means: Within the range of IUP library
binary versions 3.19 up to current 3.29 (supported ones), lib.rs matches exactly that installed binary version's header file iup.h.
This is important, as some iup.h functions changed their signature in the past. Outside that range (same as when
build.rs gets removed), the iup.h binding represents library binary version 3.29, but with the signatures offending
ABI-compatibility being inaccessible.

As probably known, some of IUP's other header file's functions extend/call into CD and/or IM library binaries or have
other prerequisites (namely the WebKit installation on Linux/Gtk for using [IupWebBrowser](https://webserver2.tecgraf.puc-rio.br/iup/en/ctrl/iupweb.html)),
therefore support will be different for those.

Since this binding's version 0.0.4, header files beyond iup.h are supported, **if** the IUP library binary is version 3.29
or later **and** the respective prerequisites (if there are any) are installed:
E.g. the additional controls of iupcontrols.h like IupMatrix are accessible only if CD is installed, or
IupPlot is accessible only if CD is installed and the installation of IUP-supplied ftgl library was detected.

Thus my recommendation is to install all of IUP, CD and IM pre-built binaries in most recent versions: It's easy on Linux using the bash script: Run
sudo ./install && ldconfig
from within the extracted "Linux Libraries" tarball folders.
(If there is sufficient demand, I can also include building those from sources by build.rs).
'build.rs' will care for everything else like configuring the binding based on detection results, linking required binaries and pushing it's detection findings to a
dependent crate. There is file dependent_example_build.rs, an example how crates depending on iup-sys can pick up detection findings in their build.rs.

'build.rs' expects, that (shared) library binaries to detect/link/load will be found by the respective tool.
If that doesn't work, one of the possible solutions is to adapt 'build.rs' accordingly: E.g. for Windows import libraries, there is a section in the end of build.rs:
Adapt paths.

There is one problem remaining about libftgl.so on *nix:
FTGL is a cross-platform Open Source library that uses Freetype2 to simplify rendering fonts in OpenGL applications.
It is NEEDED by libiupglcontrols.so and libiup_plot.so (IupGLButton etc., IupPlot).
Distros provide a libftgl-dev package with a libftgl.so, and so does the IUP "Linux Libraries" tarball, and they differ,
thus IUP's variant doesn't get installed by the 'install' script.
If You want to use those controls, then copy IUP's libftgl.so to a location where it will be found by dlopen,
and located in the search path before libftgl-dev's provided libftgl.so location, e.g. by uninstalling the libftgl-dev package.

I recommend to touch build.rs - so that it gets reevaluated - whenever something gets changed referring to IUP and related libraries installation or upon first use
and run cargo run -vv. This will print details about what gets detected.
