/* \file, derived from iupweb.h
 * \brief Web control.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::c_int;
use ::Ihandle;

extern {

    pub fn IupWebBrowserOpen() -> c_int;

    pub fn IupWebBrowser() -> *mut Ihandle;

}

