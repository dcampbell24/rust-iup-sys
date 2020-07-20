/* \file, derived from iupole.h
 * \brief Ole control.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_int, c_char};
use ::Ihandle;

extern {

    pub fn IupOleControl(progid: *const c_char) -> *mut Ihandle;

    pub fn IupOleControlOpen() -> c_int;

}
