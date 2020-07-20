/* \file, derived from iup_config.h
 * \brief Configuration file Utilities
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int, c_double};
use ::{Ihandle, Icallback};

extern {

    pub fn IupConfig() -> *mut Ihandle;

    pub fn IupConfigLoad(ih: *mut Ihandle) -> c_int;
    pub fn IupConfigSave(ih: *mut Ihandle) -> c_int;

    /****************************************************************/

    pub fn IupConfigSetVariableStr     (ih: *mut Ihandle, group: *const c_char, key: *const c_char, value: *const c_char);
    pub fn IupConfigSetVariableStrId   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, value: *const c_char);
    pub fn IupConfigSetVariableInt     (ih: *mut Ihandle, group: *const c_char, key: *const c_char, value: c_int);
    pub fn IupConfigSetVariableIntId   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, value: c_int);
    pub fn IupConfigSetVariableDouble  (ih: *mut Ihandle, group: *const c_char, key: *const c_char, value: c_double);
    pub fn IupConfigSetVariableDoubleId(ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, value: c_double);

    pub fn IupConfigGetVariableStr     (ih: *mut Ihandle, group: *const c_char, key: *const c_char) -> *const c_char;
    pub fn IupConfigGetVariableStrId   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int) -> *const c_char;
    pub fn IupConfigGetVariableInt     (ih: *mut Ihandle, group: *const c_char, key: *const c_char) -> c_int;
    pub fn IupConfigGetVariableIntId   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int) -> c_int;
    pub fn IupConfigGetVariableDouble  (ih: *mut Ihandle, group: *const c_char, key: *const c_char) -> c_double;
    pub fn IupConfigGetVariableDoubleId(ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int) -> c_double;

    pub fn IupConfigGetVariableStrDef     (ih: *mut Ihandle, group: *const c_char, key: *const c_char, def: *const c_char) -> *const c_char;
    pub fn IupConfigGetVariableStrIdDef   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, def: *const c_char) -> *const c_char;
    pub fn IupConfigGetVariableIntDef     (ih: *mut Ihandle, group: *const c_char, key: *const c_char, def: c_int) -> c_int;
    pub fn IupConfigGetVariableIntIdDef   (ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, def: c_int) -> c_int;
    pub fn IupConfigGetVariableDoubleDef  (ih: *mut Ihandle, group: *const c_char, key: *const c_char, def: c_double) -> c_double;
    pub fn IupConfigGetVariableDoubleIdDef(ih: *mut Ihandle, group: *const c_char, key: *const c_char, id: c_int, def: c_double) -> c_double;

    pub fn IupConfigCopy(src: *mut Ihandle, dst: *mut Ihandle, exclude_prefix: *const c_char);

    /****************************************************************/

    pub fn IupConfigSetListVariable(ih: *mut Ihandle, group: *const c_char, key: *const c_char, value: *const c_char, add: c_int);

    pub fn IupConfigRecentInit(ih: *mut Ihandle, menu_list: *mut Ihandle, recent_cb: Icallback, max_recent: c_int);
    pub fn IupConfigRecentUpdate(ih: *mut Ihandle, filename: *const c_char);

    pub fn IupConfigDialogShow  (ih: *mut Ihandle, dialog: *mut Ihandle, name: *const c_char);
    pub fn IupConfigDialogClosed(ih: *mut Ihandle, dialog: *mut Ihandle, name: *const c_char);

}
