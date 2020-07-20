/* \file, derived from iup_scintilla.h
 * \brief Scintilla control.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_uint};
use ::Ihandle;

extern {

    pub fn IupScintillaOpen();

    pub fn IupScintilla() -> *mut Ihandle;
    pub fn IupScintillaDlg() -> *mut Ihandle;

    // #ifdef SCINTILLA_H
    pub fn IupScintillaSendMessage(ih: *mut Ihandle, iMessage: c_uint, wParam: usize, lParam: isize) -> isize;
    // #endif

}
