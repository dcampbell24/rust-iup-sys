/* \file, derived from iupdraw.h
 * \brief Canvas Draw API
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int, c_double};
use ::Ihandle;

extern {

    /* all functions can be used only in IUP canvas and inside the ACTION callback */

    pub fn IupDrawBegin(ih: *mut Ihandle);
    pub fn IupDrawEnd(ih: *mut Ihandle);

    /* all functions can be called only between calls to Begin and End */

    pub fn IupDrawSetClipRect(ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn IupDrawGetClipRect(ih: *mut Ihandle, x1: *mut c_int, y1: *mut c_int, x2: *mut c_int, y2: *mut c_int);
    pub fn IupDrawResetClip(ih: *mut Ihandle);

    /* color controlled by the attribute DRAWCOLOR */
    /* line style or fill controlled by the attribute DRAWSTYLE */

    pub fn IupDrawParentBackground(ih: *mut Ihandle);
    pub fn IupDrawLine     (ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn IupDrawRectangle(ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn IupDrawArc      (ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int, a1: c_double, a2:c_double);
    pub fn IupDrawPolygon(ih: *mut Ihandle, points: *mut c_int, count: c_int);
    pub fn IupDrawText(ih: *mut Ihandle, text: *const c_char, len: c_int, x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn IupDrawImage(ih: *mut Ihandle, name: *const c_char,            x: c_int, y: c_int, w: c_int, h: c_int);
    pub fn IupDrawSelectRect(ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn IupDrawFocusRect (ih: *mut Ihandle, x1: c_int, y1: c_int, x2: c_int, y2: c_int);

    pub fn IupDrawGetSize(ih: *mut Ihandle, w: *mut c_int, h: *mut c_int);
    pub fn IupDrawGetTextSize(ih: *mut Ihandle, text: *const c_char, len: c_int, w: *mut c_int, h: *mut c_int);
    pub fn IupDrawGetImageInfo(name: *const c_char, w: *mut c_int, h: *mut c_int, bpp: *mut c_int);

}
