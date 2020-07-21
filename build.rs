extern crate libloading;

use libloading::{Library, Symbol, Error};
use std::ffi::CStr;

enum FTGLfont {}
type VFunc1  = unsafe fn() -> *const std::os::raw::c_char;
type VFunc2  = unsafe fn() -> std::os::raw::c_uint;
type FTGLfun = unsafe fn(*mut FTGLfont) -> std::os::raw::c_float;

fn parse_version_string(r#in: &str, prefix: &str) -> String {
    let mut result = String::from("cargo:rustc-cfg=");
    let v: Vec<&str> = r#in.splitn(3, '.').collect();
    assert!(v.len()>=2 && v.len()<=3);
    for (i, elem) in v.iter().enumerate() {
        if i==0 { result.push_str(prefix); }
        else    { result.push('_'); }
        result.push_str(*elem);
    }
    if result.as_str() == "cargo:rustc-cfg=v3_19_1" { result.truncate(result.len()-2); } // both have the same API
    println!("{}", result);
    result
}

fn main() {
    /* IUP library binary detection */
    match Library::new(if cfg!(unix) {"libiup.so"}
                       else if cfg!(macos) {"libiup.dylib"}
                       else if cfg!(windows) {"iup"}
                       else {"unknown_library_iup"} )
    {
        Ok(lib_dyn) =>
            unsafe {
                let func_dyn: Symbol<VFunc1> = lib_dyn.get(b"IupVersion").unwrap();
                let cargo_string = parse_version_string(CStr::from_ptr(func_dyn()).to_str().unwrap(), "v");
                println!("cargo:IUPVERSION={}", &cargo_string.as_str()[16..]);
            },
        Err(e) => {
            match &e {
                Error::DlOpen { desc: _ } => { println!("libloading DlOpen: {}", e); },
                Error::DlOpenUnknown => { println!("libloading DlOpenUnknown: {}", e); },
                Error::LoadLibraryW { source: _ } => { println!("libloading LoadLibraryW: {}", e); },
                Error::LoadLibraryWUnknown => { println!("libloading LoadLibraryWUnknown: {}", e); },
                _ => { println!("libloading: {}", e); },
            }
            unreachable!(); // intentionally panic if IUP is not installed or detectable this way
        }
    }


    /* CD library binary detection  */
    match Library::new(if cfg!(unix) {"libcd.so"}
                       else if cfg!(macos) {"libcd.dylib"}
                       else if cfg!(windows) {"cd"}
                       else {"unknown_library_cd"} )
    {
        Ok(lib_dyn) =>
            unsafe {
                let func_dyn: Symbol<VFunc1> = lib_dyn.get(b"cdVersion").unwrap();
                let cargo_string = parse_version_string(CStr::from_ptr(func_dyn()).to_str().unwrap(), "cdv");
                println!("cargo:rustc-cfg=installedCD");
                println!("cargo:rustc-link-lib=dylib=iup_plot"); // also depends on ftgl
                println!("cargo:rustc-link-lib=dylib=iupcontrols");
                println!("cargo:rustc-link-lib=dylib=iupcd");
                // println!("cargo:rustc-link-lib=dylib=cdcontextplus"); // ?
                println!("cargo:rustc-link-lib=dylib=cd"); // dependents may call into directly, like iup-rust does
                println!("cargo:CDVERSION={}", &cargo_string.as_str()[16..]);
            },
        Err(e) => {
            match &e {
                Error::DlOpen { desc: _ } => { println!("libloading DlOpen: {}", e); },
                Error::DlOpenUnknown => { println!("libloading DlOpenUnknown: {}", e); },
                Error::LoadLibraryW { source: _ } => { println!("libloading LoadLibraryW: {}", e); },
                Error::LoadLibraryWUnknown => { println!("libloading LoadLibraryWUnknown: {}", e); },
                _ => { println!("libloading: {}", e); },
            }
        }
    }


    /* IM library binary detection  */
    match Library::new(if cfg!(unix) {"libim.so"}
                       else if cfg!(macos) {"libim.dylib"}
                       else if cfg!(windows) {"im"}
                       else {"unknown_library_im"} )
    {
        Ok(lib_dyn) => {
            unsafe {
                let func_dyn: Symbol<VFunc1> = lib_dyn.get(b"imVersion").unwrap();
                let cargo_string = parse_version_string(CStr::from_ptr(func_dyn()).to_str().unwrap(), "imv");
                println!("cargo:rustc-cfg=installedIM");
                println!("cargo:rustc-link-lib=dylib=iupim");
                println!("cargo:IMVERSION={}", &cargo_string.as_str()[16..]);
            }
        },
        Err(e) => {
            match &e {
                Error::DlOpen { desc: _ } => {
                    println!("libloading DlOpen: {}", e);
                    if e.to_string().ends_with("undefined symbol: png_get_sRGB") {
                        println!("cargo:rustc-cfg=installedIM");
                        println!("cargo:rustc-link-lib=dylib=iupim");
                        println!("cargo:IMVERSION=installedIM");
                    }
                },
                Error::DlOpenUnknown => { println!("libloading DlOpenUnknown: {}", e); },
                Error::LoadLibraryW { source: _ } => { println!("libloading LoadLibraryW: {}", e); },
                Error::LoadLibraryWUnknown => { println!("libloading LoadLibraryWUnknown: {}", e); },
                _ => { println!("libloading: {}", e); },
            }
        }
    }


    /* Webkit library binary detection */
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=iupweb");
        println!("cargo:rustc-link-lib=dylib=iupfiledlg");
        println!("cargo:rustc-link-lib=dylib=iupole");
    }
    else {
        match Library::new(if cfg!(unix) {"libwebkitgtk-3.0.so.0"}
                           else if cfg!(macos) {"libwebkitgtk-3.0.dylib"}
                           else {"unknown_library_web"} )
        {
            Ok(lib_dyn) =>
                unsafe {
                    let func_dyn: Symbol<VFunc2> = lib_dyn.get(b"webkit_major_version").unwrap();
                    let mut version_str = func_dyn().to_string();
                    version_str.push('.');
                    let func_dyn: Symbol<VFunc2> = lib_dyn.get(b"webkit_minor_version").unwrap();
                    version_str.push_str(func_dyn().to_string().as_str());
                    let cargo_string = parse_version_string(version_str.as_str(), "webv");
                    println!("cargo:rustc-cfg=installedWebkit");
                    println!("cargo:rustc-link-lib=dylib=iupweb");
                    println!("cargo:WEBVERSION={}", &cargo_string.as_str()[16..]);
                },
            Err(e) => {
                match &e {
                    Error::DlOpen { desc: _ } => { println!("libloading DlOpen: {}", e); },
                    Error::DlOpenUnknown => { println!("libloading DlOpenUnknown: {}", e); },
                    Error::LoadLibraryW { source: _ } => { println!("libloading LoadLibraryW: {}", e); },
                    Error::LoadLibraryWUnknown => { println!("libloading LoadLibraryWUnknown: {}", e); },
                    _ => { println!("libloading: {}", e); },
                }
            }
        }
    }


    /* FTGL library binary detection */
    match Library::new(if cfg!(unix) {"libftgl.so"}
                       else if cfg!(macos) {"libftgl.dylib"}
                       else if cfg!(windows) {"ftgl"}
                       else {"unknown_library_ftgl"} )
    {
        Ok(lib_dyn) =>
            unsafe {
                let func_dyn: Result<Symbol<FTGLfun>, Error> = lib_dyn.get(b"ftglGetFontMaxWidth");
                if func_dyn.is_ok() {
                    println!("cargo:rustc-cfg=installedFTGL");
                    println!("cargo:rustc-link-lib=dylib=iupglcontrols");
                    println!("cargo:rustc-link-lib=dylib=ftgl"); // optional on Linux: NEEDED only by iupglcontrols and iup_plot
                    println!("cargo:FTGLVERSION=IUP");
                }
                else { // float ftglGetFontMaxWidth(FTGLfont* font); is available in the IUP supplied library only
                    println!("libloading: The loadable ftgl library is NOT the one supplied by IUP (or CD): All components depending on IUP's ftgl library will be inaccessible ");
                }
            },
        Err(e) => {
            match &e {
                Error::DlOpen { desc: _ } => { println!("libloading DlOpen: {}", e); },
                Error::DlOpenUnknown => { println!("libloading DlOpenUnknown: {}", e); },
                Error::LoadLibraryW { source: _ } => { println!("libloading LoadLibraryW: {}", e); },
                Error::LoadLibraryWUnknown => { println!("libloading LoadLibraryWUnknown: {}", e); },
                _ => { println!("libloading: {}", e); },
            }
        }
    }

    println!("cargo:rustc-link-lib=dylib=iupgl");
    println!("cargo:rustc-link-lib=dylib=iup_mglplot");
    println!("cargo:rustc-link-lib=dylib=iup_scintilla");
    println!("cargo:rustc-link-lib=dylib=iuptuio");
    println!("cargo:rustc-link-lib=dylib=iupimglib");
    println!("cargo:rustc-link-lib=dylib=iup");

    /*
        All the iup* libraries listed won't necessarily be linked actually: It depends on whether required to solve references.
        For Linux, rustc emits --as-needed to the linker, akin for windows: I.e. non-needed ones will be discarded.
    */

    #[cfg(unix)]
    {
        // println!("cargo:rustc-link-search=.../iup-3.29_Linux415_64_lib/ftgl/lib/Linux415_64");
        // somehow set rpath to the path specified in previous line, but rustc doesn't seem to allow that
    }
    #[cfg(windows)]
    {
        println!(r###"cargo:rustc-link-search=C:\bin\iup_3_29"###); // location of IUP import libraries: iup.lib etc.
        println!(r###"cargo:rustc-link-search=C:\bin\cd_5_13"###);  // location of CD  import libraries: cd.lib etc.
        println!(r###"cargo:rustc-link-search=C:\bin\im_3_14"###);  // location of IM  import libraries: im.lib etc.
    }
}
