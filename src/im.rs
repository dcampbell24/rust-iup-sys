/* \file, derived from iupim.h
 * \brief Utilities using IM
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_int, c_char};
#[cfg(IM_API)]
use std::os::raw::c_void;
use ::Ihandle;

extern {

    pub fn IupImOpen();  /* optional */

    pub fn IupLoadImage(filename: *const c_char) -> *mut Ihandle;
    pub fn IupSaveImage(ih: *mut Ihandle, filename: *const c_char, format: *const c_char) -> c_int;

    pub fn IupLoadAnimation(filename: *const c_char) -> *mut Ihandle;
    pub fn IupLoadAnimationFrames(filename_list: *mut *const c_char, file_count: c_int) -> *mut Ihandle;

    /* makes sense only together with an IM Rust binding */
    // #ifdef __IM_IMAGE_H
    #[cfg(IM_API)]
    pub fn IupGetNativeHandleImage(handle: *mut c_void) -> *mut ::imImage;
    #[cfg(IM_API)]
    pub fn IupGetImageNativeHandle(image: *const ::imImage) -> *mut c_void;

    #[cfg(IM_API)]
    pub fn IupImageFromImImage(image: *const ::imImage) -> *mut Ihandle;
    #[cfg(IM_API)]
    pub fn IupImageToImImage(iup_image: *mut Ihandle) -> *mut ::imImage;
    // #endif

}
