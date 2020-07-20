/* \file, derived from iupglcontrols.h
 * \brief GL Controls.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int};
use ::Ihandle;

extern {

    pub fn IupGLControlsOpen() -> c_int;

    pub fn IupGLCanvasBoxv(children: *mut *mut Ihandle) -> *mut Ihandle;
    pub fn IupGLCanvasBox(child: *mut Ihandle, ...) -> *mut Ihandle;

    pub fn IupGLSubCanvas() -> *mut Ihandle;

    pub fn IupGLLabel(title: *const c_char) -> *mut Ihandle;
    pub fn IupGLSeparator() -> *mut Ihandle;
    pub fn IupGLButton(title: *const c_char) -> *mut Ihandle;
    pub fn IupGLToggle(title: *const c_char) -> *mut Ihandle;
    pub fn IupGLLink(url: *const c_char, title: *const c_char) -> *mut Ihandle;
    pub fn IupGLProgressBar() -> *mut Ihandle;
    pub fn IupGLVal() -> *mut Ihandle;
    pub fn IupGLFrame(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGLExpander(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGLScrollBox(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGLSizeBox(child: *mut Ihandle) -> *mut Ihandle;
    pub fn IupGLText() -> *mut Ihandle;


    /* Utilities */
    pub fn IupGLDrawImage(ih: *mut Ihandle, name: *const c_char, x: c_int, y: c_int, active: c_int);
    pub fn IupGLDrawText(ih: *mut Ihandle, str: *const c_char, len: c_int, x: c_int, y: c_int);
    pub fn IupGLDrawGetTextSize(ih: *mut Ihandle, str: *const c_char, w: *mut c_int, h: *mut c_int);
    pub fn IupGLDrawGetImageInfo(name: *const c_char, w: *mut c_int, h: *mut c_int, bpp: *mut c_int);

}
