/* \file, derived from iupcontrols.h
 * \brief initializes dial, gauge, colorbrowser, colorbar controls.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int};
use ::Ihandle;

extern {

    pub fn IupControlsOpen() -> c_int;

    pub fn IupCells() -> *mut Ihandle;
    pub fn IupMatrix(action: *const c_char) -> *mut Ihandle;
    pub fn IupMatrixList() -> *mut Ihandle;
    pub fn IupMatrixEx() -> *mut Ihandle;

    /* available only when linking with "iupluamatrix" */
    // pub fn IupMatrixSetFormula(ih: *mut Ihandle, col: c_int, formula: *const c_char, init: *const c_char);
    // pub fn IupMatrixSetDynamic(ih: *mut Ihandle,                                     init: *const c_char);

}
