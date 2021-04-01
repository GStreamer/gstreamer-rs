// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

#[doc(alias = "gst_type_find_helper")]
pub fn type_find_helper<P: IsA<gst::Pad>>(
    src: &P,
    size: u64,
) -> Result<gst::Caps, glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        Option::<_>::from_glib_full(ffi::gst_type_find_helper(
            src.as_ref().to_glib_none().0,
            size,
        ))
        .ok_or_else(|| glib::bool_error!("Could not find type"))
    }
}

#[doc(alias = "gst_type_find_helper_for_extension")]
pub fn type_find_helper_for_extension<P: IsA<gst::Object>>(
    obj: Option<&P>,
    extension: &str,
) -> Result<gst::Caps, glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        Option::<_>::from_glib_full(ffi::gst_type_find_helper_for_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            extension.to_glib_none().0,
        ))
        .ok_or_else(|| glib::bool_error!("Could not find type"))
    }
}

//#[doc(alias = "gst_type_find_helper_get_range")]
//pub fn type_find_helper_get_range<P: IsA<gst::Object>, Q: IsA<gst::Object>, R: FnMut(&gst::Object, Option<&gst::Object>, u64, u32, &gst::Buffer) -> gst::FlowReturn>(obj: &P, parent: Option<&Q>, func: R, size: u64, extension: Option<&str>) -> (Option<gst::Caps>, gst::TypeFindProbability) {
//    unsafe { TODO: call ffi:gst_type_find_helper_get_range() }
//}

//#[cfg(any(feature = "v1_14_3", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14_3")))]
//#[doc(alias = "gst_type_find_helper_get_range_full")]
//pub fn type_find_helper_get_range_full<P: IsA<gst::Object>, Q: IsA<gst::Object>, R: FnMut(&gst::Object, Option<&gst::Object>, u64, u32, &gst::Buffer) -> gst::FlowReturn>(obj: &P, parent: Option<&Q>, func: R, size: u64, extension: Option<&str>) -> (gst::FlowReturn, gst::Caps, gst::TypeFindProbability) {
//    unsafe { TODO: call ffi:gst_type_find_helper_get_range_full() }
//}
