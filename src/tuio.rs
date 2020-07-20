/* \file, derived from iuptuio.h
 * \brief IupTuioClient control
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_int};
use ::Ihandle;

extern {

    pub fn IupTuioOpen() -> c_int;
    pub fn IupTuioClient(port: c_int) -> *mut Ihandle;

}
