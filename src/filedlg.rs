/* \file, derived from iupfiledlg.h
 * \brief New FileDlg (Windows Only).
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::c_int;

extern  {

    /* the only exported function,
       once called it will replace regular IupFileDlg */

    pub fn IupNewFileDlgOpen() -> c_int;

}
