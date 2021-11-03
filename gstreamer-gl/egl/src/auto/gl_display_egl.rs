// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstGLDisplayEGL")]
    pub struct GLDisplayEGL(Object<ffi::GstGLDisplayEGL, ffi::GstGLDisplayEGLClass>) @extends gst_gl::GLDisplay, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_display_egl_get_type(),
    }
}

impl GLDisplayEGL {
    #[doc(alias = "gst_gl_display_egl_new")]
    pub fn new() -> GLDisplayEGL {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_display_egl_new()) }
    }

    //#[doc(alias = "gst_gl_display_egl_new_with_egl_display")]
    //#[doc(alias = "new_with_egl_display")]
    //pub fn with_egl_display(display: /*Unimplemented*/Option<Fundamental: Pointer>) -> GLDisplayEGL {
    //    unsafe { TODO: call ffi:gst_gl_display_egl_new_with_egl_display() }
    //}

    #[doc(alias = "gst_gl_display_egl_from_gl_display")]
    pub fn from_gl_display(display: &impl IsA<gst_gl::GLDisplay>) -> Option<GLDisplayEGL> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_gl_display_egl_from_gl_display(
                display.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_gl_display_egl_get_from_native")]
    //#[doc(alias = "get_from_native")]
    //pub fn from_native(type_: /*Ignored*/gst_gl::GLDisplayType, display: /*Unimplemented*/Fundamental: UIntPtr) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gst_gl_display_egl_get_from_native() }
    //}
}

impl Default for GLDisplayEGL {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLDisplayEGL {}
unsafe impl Sync for GLDisplayEGL {}

impl GLDisplayEGL {
    pub const NONE: Option<&'static GLDisplayEGL> = None;
}
