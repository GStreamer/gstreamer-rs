// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{GLDisplay, GLSLProfile, GLSLVersion};
use glib::{prelude::*, translate::*};
use std::mem;

#[doc(alias = "gst_gl_check_extension")]
pub fn gl_check_extension(name: &str, ext: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_gl_check_extension(
            name.to_glib_none().0,
            ext.to_glib_none().0,
        ))
    }
}

#[doc(alias = "gst_gl_element_propagate_display_context")]
pub fn gl_element_propagate_display_context(
    element: &impl IsA<gst::Element>,
    display: &impl IsA<GLDisplay>,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_gl_element_propagate_display_context(
            element.as_ref().to_glib_none().0,
            display.as_ref().to_glib_none().0,
        );
    }
}

//#[cfg(feature = "v1_24")]
//#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
//#[doc(alias = "gst_gl_swizzle_invert")]
//pub fn gl_swizzle_invert(swizzle: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 14 }; 4, inversion: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 14 }; 4) {
//    unsafe { TODO: call ffi:gst_gl_swizzle_invert() }
//}

//#[cfg(feature = "v1_24")]
//#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
//#[doc(alias = "gst_gl_video_format_swizzle")]
//pub fn gl_video_format_swizzle(video_format: /*Ignored*/gst_video::VideoFormat, swizzle: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 14 }; 4) -> bool {
//    unsafe { TODO: call ffi:gst_gl_video_format_swizzle() }
//}

#[doc(alias = "gst_glsl_string_get_version_profile")]
pub fn glsl_string_get_version_profile(s: &str) -> Option<(GLSLVersion, GLSLProfile)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut version = mem::MaybeUninit::uninit();
        let mut profile = mem::MaybeUninit::uninit();
        let ret = from_glib(ffi::gst_glsl_string_get_version_profile(
            s.to_glib_none().0,
            version.as_mut_ptr(),
            profile.as_mut_ptr(),
        ));
        if ret {
            Some((
                from_glib(version.assume_init()),
                from_glib(profile.assume_init()),
            ))
        } else {
            None
        }
    }
}
