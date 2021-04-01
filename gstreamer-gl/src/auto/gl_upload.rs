// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib::wrapper! {
    pub struct GLUpload(Object<ffi::GstGLUpload, ffi::GstGLUploadClass>) @extends gst::Object;

    match fn {
        get_type => || ffi::gst_gl_upload_get_type(),
    }
}

impl GLUpload {
    #[doc(alias = "gst_gl_upload_new")]
    pub fn new<P: IsA<GLContext>>(context: &P) -> GLUpload {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_gl_upload_new(context.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_upload_get_caps")]
    pub fn get_caps(&self) -> (gst::Caps, gst::Caps) {
        unsafe {
            let mut in_caps = ptr::null_mut();
            let mut out_caps = ptr::null_mut();
            ffi::gst_gl_upload_get_caps(self.to_glib_none().0, &mut in_caps, &mut out_caps);
            (from_glib_full(in_caps), from_glib_full(out_caps))
        }
    }

    #[doc(alias = "gst_gl_upload_set_caps")]
    pub fn set_caps(
        &self,
        in_caps: &gst::Caps,
        out_caps: &gst::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_upload_set_caps(
                    self.to_glib_none().0,
                    in_caps.to_glib_none().0,
                    out_caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    #[doc(alias = "gst_gl_upload_set_context")]
    pub fn set_context<P: IsA<GLContext>>(&self, context: &P) {
        unsafe {
            ffi::gst_gl_upload_set_context(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_upload_transform_caps")]
    pub fn transform_caps<P: IsA<GLContext>>(
        &self,
        context: &P,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: &gst::Caps,
    ) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_gl_upload_transform_caps(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
                direction.to_glib(),
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_upload_get_input_template_caps")]
    pub fn get_input_template_caps() -> gst::Caps {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_upload_get_input_template_caps()) }
    }
}

unsafe impl Send for GLUpload {}
unsafe impl Sync for GLUpload {}
