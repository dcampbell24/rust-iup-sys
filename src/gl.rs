/* \file, derived from iupgl.h
 * \brief OpenGL canvas for Iup.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int, c_float};
use ::Ihandle;

extern {

    pub fn IupGLCanvasOpen();

    pub fn IupGLCanvas(action: *const c_char) -> *mut Ihandle;
    pub fn IupGLBackgroundBox(child: *mut Ihandle) -> *mut Ihandle;

    pub fn IupGLMakeCurrent(ih: *mut Ihandle);
    pub fn IupGLIsCurrent(ih: *mut Ihandle) -> c_int;
    pub fn IupGLSwapBuffers(ih: *mut Ihandle);
    pub fn IupGLPalette(ih: *mut Ihandle, index: c_int, r: c_float, g: c_float, b: c_float);
    pub fn IupGLUseFont(ih: *mut Ihandle, first: c_int, count: c_int, list_base: c_int);
    pub fn IupGLWait(gl: c_int);

}
