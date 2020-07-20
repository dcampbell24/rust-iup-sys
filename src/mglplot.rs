/* \file, derived from iup_mglplot.h
 * \brief Plot component for Iup.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_char, c_int, c_double, c_void};
use ::Ihandle;

extern {

    /* Initialize IupMglPlot widget class */
    pub fn IupMglPlotOpen();

    /* Create an IupMglPlot widget instance */
    pub fn IupMglPlot() -> *mut Ihandle;

    /***********************************************/
    /*           Additional API                    */

    /* Linear Data Only */
    pub fn IupMglPlotBegin(ih: *mut Ihandle, dim: c_int);
    pub fn IupMglPlotAdd1D(ih: *mut Ihandle, name: *const c_char, y: c_double);
    pub fn IupMglPlotAdd2D(ih: *mut Ihandle, x: c_double, y: c_double);
    pub fn IupMglPlotAdd3D(ih: *mut Ihandle, x: c_double, y: c_double, z: c_double);
    pub fn IupMglPlotEnd(ih: *mut Ihandle) -> c_int;

    /* Linear (dim=1,2,3), Planar (dim=1), Volumetric (dim=1) */
    pub fn IupMglPlotNewDataSet(ih: *mut Ihandle, dim: c_int) -> c_int;

    /* Linear Data Only */
    pub fn IupMglPlotInsert1D(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, names: *mut *const c_char, y: *const c_double, count: c_int);
    pub fn IupMglPlotInsert2D(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *const c_double, y: *const c_double, count: c_int);
    pub fn IupMglPlotInsert3D(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *const c_double, y: *const c_double, z: *const c_double, count: c_int);

    /* Linear Data Only */
    pub fn IupMglPlotSet1D(ih: *mut Ihandle, ds_index: c_int, names: *mut *const c_char, y: *const c_double, count: c_int);
    pub fn IupMglPlotSet2D(ih: *mut Ihandle, ds_index: c_int, x: *const c_double, y: *const c_double, count: c_int);
    pub fn IupMglPlotSet3D(ih: *mut Ihandle, ds_index: c_int, x: *const c_double, y: *const c_double, z: *const c_double, count: c_int);
    pub fn IupMglPlotSetFormula(ih: *mut Ihandle, ds_index: c_int, formulaX: *const c_char, formulaY: *const c_char, formulaZ: *const c_char, count: c_int);

    /* Linear (dim=1), Planar (dim=1), Volumetric (dim=1) */
    pub fn IupMglPlotSetData       (ih: *mut Ihandle, ds_index: c_int,   data: *const c_double, count_x: c_int, count_y: c_int, count_z: c_int);
    pub fn IupMglPlotLoadData      (ih: *mut Ihandle, ds_index: c_int, filename: *const c_char, count_x: c_int, count_y: c_int, count_z: c_int);
    pub fn IupMglPlotSetFromFormula(ih: *mut Ihandle, ds_index: c_int,  formula: *const c_char, count_x: c_int, count_y: c_int, count_z: c_int);

    /* Only inside callbacks */
    pub fn IupMglPlotTransform(ih: *mut Ihandle, x: c_double, y: c_double, z: c_double, ix: *mut c_int, iy: *mut c_int);
    pub fn IupMglPlotTransformTo(ih: *mut Ihandle, ix: c_int, iy: c_int, x: *mut c_double, y: *mut c_double, z: *mut c_double);

    /* Only inside callbacks */
    pub fn IupMglPlotDrawMark(ih: *mut Ihandle, x: c_double, y: c_double, z: c_double);
    pub fn IupMglPlotDrawLine(ih: *mut Ihandle, x1: c_double, y1: c_double, z1: c_double, x2: c_double, y2: c_double, z2: c_double);
    pub fn IupMglPlotDrawText(ih: *mut Ihandle, text: *const c_char, x: c_double, y: c_double, z: c_double);

    pub fn IupMglPlotPaintTo(ih: *mut Ihandle, format: *const c_char, w: c_int, h: c_int, dpi: c_double, data: *mut c_void);

    /***********************************************/

    /* Utility label for showing TeX labels */
    pub fn IupMglLabel(title: *const c_char) -> *mut Ihandle;

}
