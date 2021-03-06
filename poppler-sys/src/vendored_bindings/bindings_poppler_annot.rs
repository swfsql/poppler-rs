/* automatically generated by rust-bindgen */

use crate::dep_types::*;
use crate::poppler::*;

#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotType {
    POPPLER_ANNOT_UNKNOWN = 0,
    POPPLER_ANNOT_TEXT = 1,
    POPPLER_ANNOT_LINK = 2,
    POPPLER_ANNOT_FREE_TEXT = 3,
    POPPLER_ANNOT_LINE = 4,
    POPPLER_ANNOT_SQUARE = 5,
    POPPLER_ANNOT_CIRCLE = 6,
    POPPLER_ANNOT_POLYGON = 7,
    POPPLER_ANNOT_POLY_LINE = 8,
    POPPLER_ANNOT_HIGHLIGHT = 9,
    POPPLER_ANNOT_UNDERLINE = 10,
    POPPLER_ANNOT_SQUIGGLY = 11,
    POPPLER_ANNOT_STRIKE_OUT = 12,
    POPPLER_ANNOT_STAMP = 13,
    POPPLER_ANNOT_CARET = 14,
    POPPLER_ANNOT_INK = 15,
    POPPLER_ANNOT_POPUP = 16,
    POPPLER_ANNOT_FILE_ATTACHMENT = 17,
    POPPLER_ANNOT_SOUND = 18,
    POPPLER_ANNOT_MOVIE = 19,
    POPPLER_ANNOT_WIDGET = 20,
    POPPLER_ANNOT_SCREEN = 21,
    POPPLER_ANNOT_PRINTER_MARK = 22,
    POPPLER_ANNOT_TRAP_NET = 23,
    POPPLER_ANNOT_WATERMARK = 24,
    POPPLER_ANNOT_3D = 25,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotFlag {
    POPPLER_ANNOT_FLAG_UNKNOWN = 0,
    POPPLER_ANNOT_FLAG_INVISIBLE = 1,
    POPPLER_ANNOT_FLAG_HIDDEN = 2,
    POPPLER_ANNOT_FLAG_PRINT = 4,
    POPPLER_ANNOT_FLAG_NO_ZOOM = 8,
    POPPLER_ANNOT_FLAG_NO_ROTATE = 16,
    POPPLER_ANNOT_FLAG_NO_VIEW = 32,
    POPPLER_ANNOT_FLAG_READ_ONLY = 64,
    POPPLER_ANNOT_FLAG_LOCKED = 128,
    POPPLER_ANNOT_FLAG_TOGGLE_NO_VIEW = 256,
    POPPLER_ANNOT_FLAG_LOCKED_CONTENTS = 512,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotMarkupReplyType {
    POPPLER_ANNOT_MARKUP_REPLY_TYPE_R = 0,
    POPPLER_ANNOT_MARKUP_REPLY_TYPE_GROUP = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotExternalDataType {
    POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_3D = 0,
    POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_UNKNOWN = 1,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotTextState {
    POPPLER_ANNOT_TEXT_STATE_MARKED = 0,
    POPPLER_ANNOT_TEXT_STATE_UNMARKED = 1,
    POPPLER_ANNOT_TEXT_STATE_ACCEPTED = 2,
    POPPLER_ANNOT_TEXT_STATE_REJECTED = 3,
    POPPLER_ANNOT_TEXT_STATE_CANCELLED = 4,
    POPPLER_ANNOT_TEXT_STATE_COMPLETED = 5,
    POPPLER_ANNOT_TEXT_STATE_NONE = 6,
    POPPLER_ANNOT_TEXT_STATE_UNKNOWN = 7,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PopplerAnnotFreeTextQuadding {
    POPPLER_ANNOT_FREE_TEXT_QUADDING_LEFT_JUSTIFIED = 0,
    POPPLER_ANNOT_FREE_TEXT_QUADDING_CENTERED = 1,
    POPPLER_ANNOT_FREE_TEXT_QUADDING_RIGHT_JUSTIFIED = 2,
}
#[repr(C)]
pub struct _PopplerAnnotCalloutLine {
    pub multiline: gboolean,
    pub x1: gdouble,
    pub y1: gdouble,
    pub x2: gdouble,
    pub y2: gdouble,
    pub x3: gdouble,
    pub y3: gdouble,
}
#[test]
fn bindgen_test_layout__PopplerAnnotCalloutLine() {
    assert_eq!(
        ::std::mem::size_of::<_PopplerAnnotCalloutLine>(),
        56usize,
        concat!("Size of: ", stringify!(_PopplerAnnotCalloutLine))
    );
    assert_eq!(
        ::std::mem::align_of::<_PopplerAnnotCalloutLine>(),
        8usize,
        concat!("Alignment of ", stringify!(_PopplerAnnotCalloutLine))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).multiline as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(multiline)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x1 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y1 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x2 as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y2 as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).x3 as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(x3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PopplerAnnotCalloutLine>())).y3 as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_PopplerAnnotCalloutLine),
            "::",
            stringify!(y3)
        )
    );
}
extern "C" {
    pub fn poppler_annot_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_get_annot_type(poppler_annot: *mut PopplerAnnot) -> PopplerAnnotType;
}
extern "C" {
    pub fn poppler_annot_get_contents(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_set_contents(poppler_annot: *mut PopplerAnnot, contents: *const gchar);
}
extern "C" {
    pub fn poppler_annot_get_name(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_get_modified(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_get_flags(poppler_annot: *mut PopplerAnnot) -> PopplerAnnotFlag;
}
extern "C" {
    pub fn poppler_annot_set_flags(poppler_annot: *mut PopplerAnnot, flags: PopplerAnnotFlag);
}
extern "C" {
    pub fn poppler_annot_get_color(poppler_annot: *mut PopplerAnnot) -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_annot_set_color(
        poppler_annot: *mut PopplerAnnot,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_get_page_index(poppler_annot: *mut PopplerAnnot) -> gint;
}
extern "C" {
    pub fn poppler_annot_get_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_set_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_markup_get_label(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_markup_set_label(
        poppler_annot: *mut PopplerAnnotMarkup,
        label: *const gchar,
    );
}
extern "C" {
    pub fn poppler_annot_markup_has_popup(poppler_annot: *mut PopplerAnnotMarkup) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup(
        poppler_annot: *mut PopplerAnnotMarkup,
        popup_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
        is_open: gboolean,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    ) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_markup_set_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_opacity(poppler_annot: *mut PopplerAnnotMarkup) -> gdouble;
}
extern "C" {
    pub fn poppler_annot_markup_set_opacity(
        poppler_annot: *mut PopplerAnnotMarkup,
        opacity: gdouble,
    );
}
extern "C" {
    pub fn poppler_annot_markup_get_date(poppler_annot: *mut PopplerAnnotMarkup) -> *mut GDate;
}
extern "C" {
    pub fn poppler_annot_markup_get_subject(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_markup_get_reply_to(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> PopplerAnnotMarkupReplyType;
}
extern "C" {
    pub fn poppler_annot_markup_get_external_data(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> PopplerAnnotExternalDataType;
}
extern "C" {
    pub fn poppler_annot_text_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_text_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_get_is_open(poppler_annot: *mut PopplerAnnotText) -> gboolean;
}
extern "C" {
    pub fn poppler_annot_text_set_is_open(poppler_annot: *mut PopplerAnnotText, is_open: gboolean);
}
extern "C" {
    pub fn poppler_annot_text_get_icon(poppler_annot: *mut PopplerAnnotText) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_text_set_icon(poppler_annot: *mut PopplerAnnotText, icon: *const gchar);
}
extern "C" {
    pub fn poppler_annot_text_get_state(
        poppler_annot: *mut PopplerAnnotText,
    ) -> PopplerAnnotTextState;
}
extern "C" {
    pub fn poppler_annot_text_markup_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_highlight(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_squiggly(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_strikeout(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_new_underline(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_text_markup_set_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
        quadrilaterals: *mut GArray,
    );
}
extern "C" {
    pub fn poppler_annot_text_markup_get_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
    ) -> *mut GArray;
}
extern "C" {
    pub fn poppler_annot_free_text_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_free_text_get_quadding(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> PopplerAnnotFreeTextQuadding;
}
extern "C" {
    pub fn poppler_annot_free_text_get_callout_line(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_attachment(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut PopplerAttachment;
}
extern "C" {
    pub fn poppler_annot_file_attachment_get_name(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_movie_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_movie_get_title(poppler_annot: *mut PopplerAnnotMovie) -> *mut gchar;
}
extern "C" {
    pub fn poppler_annot_movie_get_movie(
        poppler_annot: *mut PopplerAnnotMovie,
    ) -> *mut PopplerMovie;
}
extern "C" {
    pub fn poppler_annot_screen_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_screen_get_action(
        poppler_annot: *mut PopplerAnnotScreen,
    ) -> *mut PopplerAction;
}
extern "C" {
    pub fn poppler_annot_line_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_line_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_line_set_vertices(
        poppler_annot: *mut PopplerAnnotLine,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    );
}
extern "C" {
    pub fn poppler_annot_callout_line_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_callout_line_new() -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_callout_line_copy(
        callout: *mut PopplerAnnotCalloutLine,
    ) -> *mut PopplerAnnotCalloutLine;
}
extern "C" {
    pub fn poppler_annot_callout_line_free(callout: *mut PopplerAnnotCalloutLine);
}
extern "C" {
    pub fn poppler_annot_circle_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_circle_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_circle_set_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_circle_get_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
    ) -> *mut PopplerColor;
}
extern "C" {
    pub fn poppler_annot_square_get_type() -> GType;
}
extern "C" {
    pub fn poppler_annot_square_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
}
extern "C" {
    pub fn poppler_annot_square_set_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
        poppler_color: *mut PopplerColor,
    );
}
extern "C" {
    pub fn poppler_annot_square_get_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
    ) -> *mut PopplerColor;
}
