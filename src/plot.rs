/* \file, derived from iup_plot.h
 * \brief Plot component for Iup.
 *
 * See Copyright Notice in "iup.h"/LICENSE-TECGRAF/LICENSE
 */

use std::os::raw::{c_int, c_double, c_char};
use ::Ihandle;

extern {

    /* Initialize IupPlot widget class */
    pub fn IupPlotOpen();

    /* Create an IupPlot widget instance */
    pub fn IupPlot() -> *mut Ihandle;

/***********************************************/
/*           Additional API                    */

    pub fn IupPlotBegin(ih: *mut Ihandle, strXdata: c_int);
    pub fn IupPlotAdd(ih: *mut Ihandle, x: c_double, y: c_double);
    pub fn IupPlotAddStr(ih: *mut Ihandle, x: *const c_char, y: c_double);
    pub fn IupPlotAddSegment(ih: *mut Ihandle, x: c_double, y: c_double);
    pub fn IupPlotEnd(ih: *mut Ihandle) -> c_int;

    pub fn IupPlotLoadData(ih: *mut Ihandle, filename: *const c_char, strXdata: c_int) -> c_int;

    /* available only when linking with "iupluaplot" */
    //pub fn IupPlotSetFormula(ih: *mut Ihandle, sample_count: c_int, formula: *const c_char, init: *const c_char) -> c_int;

    pub fn IupPlotInsert       (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: c_double, y: c_double);
    pub fn IupPlotInsertStr    (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *const c_char, y: c_double);
    pub fn IupPlotInsertSegment(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: c_double, y: c_double);

    pub fn IupPlotInsertStrSamples(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *mut *const c_char, y: *mut c_double, count: c_int);
    pub fn IupPlotInsertSamples   (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *mut c_double, y: *mut c_double, count: c_int);

    pub fn IupPlotAddSamples   (ih: *mut Ihandle, ds_index: c_int, x: *mut c_double, y: *mut c_double, count: c_int);
    pub fn IupPlotAddStrSamples(ih: *mut Ihandle, ds_index: c_int, x: *mut *const c_char, y: *mut c_double, count: c_int );

    pub fn IupPlotGetSample         (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *mut c_double, y: *mut c_double);
    pub fn IupPlotGetSampleStr      (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *mut *const c_char, y: *mut c_double);
    pub fn IupPlotGetSampleSelection(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int) -> c_int;
    pub fn IupPlotGetSampleExtra    (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int) -> c_double;
    pub fn IupPlotSetSample         (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: c_double, y: c_double);
    pub fn IupPlotSetSampleStr      (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, x: *const c_char, y: c_double);
    pub fn IupPlotSetSampleSelection(ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, selected: c_int);
    pub fn IupPlotSetSampleExtra    (ih: *mut Ihandle, ds_index: c_int, sample_index: c_int, extra: c_double);

    pub fn IupPlotTransform  (ih: *mut Ihandle, x: c_double, y: c_double, cnv_x: *mut c_double, cnv_y: *mut c_double);
    pub fn IupPlotTransformTo(ih: *mut Ihandle, cnv_x: c_double, cnv_y: c_double, x: *mut c_double, y: *mut c_double);

    pub fn IupPlotFindSample (ih: *mut Ihandle, cnv_x: c_double, cnv_y: c_double, ds_index: *mut c_int, sample_index:  *mut c_int) -> c_int;
    pub fn IupPlotFindSegment(ih: *mut Ihandle, cnv_x: c_double, cnv_y: c_double, ds_index: *mut c_int, sample_index1: *mut c_int, sample_index2: *mut c_int) -> c_int;

    pub fn IupPlotPaintTo(ih: *mut Ihandle, cnv: *mut ::cdCanvas);

/***********************************************/

}
