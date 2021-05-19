// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;
use std::mem;

#[doc(alias = "gst_type_find_helper_for_data")]
pub fn type_find_helper_for_data<P: IsA<gst::Object>, R: AsRef<[u8]>>(
    obj: Option<&P>,
    data: R,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(alias = "gst_type_find_helper_for_data_with_extension")]
pub fn type_find_helper_for_data_with_extension<P: IsA<gst::Object>, R: AsRef<[u8]>>(
    obj: Option<&P>,
    data: R,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let data = data.as_ref();
        let (ptr, len) = (data.as_ptr(), data.len());
        let ret = ffi::gst_type_find_helper_for_data_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            mut_override(ptr),
            len,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[doc(alias = "gst_type_find_helper_for_buffer")]
pub fn type_find_helper_for_buffer<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(alias = "gst_type_find_helper_for_buffer_with_extension")]
pub fn type_find_helper_for_buffer_with_extension<P: IsA<gst::Object>>(
    obj: Option<&P>,
    buf: &gst::Buffer,
    extension: Option<&str>,
) -> Result<(gst::Caps, gst::TypeFindProbability), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        let mut prob = mem::MaybeUninit::uninit();
        let ret = ffi::gst_type_find_helper_for_buffer_with_extension(
            obj.map(|p| p.as_ref()).to_glib_none().0,
            buf.to_glib_none().0,
            extension.to_glib_none().0,
            prob.as_mut_ptr(),
        );
        if ret.is_null() {
            Err(glib::bool_error!("No type could be found"))
        } else {
            Ok((from_glib_full(ret), from_glib(prob.assume_init())))
        }
    }
}
