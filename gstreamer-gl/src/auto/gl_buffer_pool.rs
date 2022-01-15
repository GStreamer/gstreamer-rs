// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::GLAllocationParams;
use crate::GLContext;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstGLBufferPool")]
    pub struct GLBufferPool(Object<ffi::GstGLBufferPool, ffi::GstGLBufferPoolClass>) @extends gst::BufferPool, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_buffer_pool_get_type(),
    }
}

impl GLBufferPool {
    pub const NONE: Option<&'static GLBufferPool> = None;

    #[doc(alias = "gst_gl_buffer_pool_new")]
    pub fn new(context: &impl IsA<GLContext>) -> GLBufferPool {
        skip_assert_initialized!();
        unsafe {
            gst::BufferPool::from_glib_none(ffi::gst_gl_buffer_pool_new(
                context.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for GLBufferPool {}
unsafe impl Sync for GLBufferPool {}

pub trait GLBufferPoolExt: 'static {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_gl_buffer_pool_get_gl_allocation_params")]
    #[doc(alias = "get_gl_allocation_params")]
    fn gl_allocation_params(&self) -> Option<GLAllocationParams>;
}

impl<O: IsA<GLBufferPool>> GLBufferPoolExt for O {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    fn gl_allocation_params(&self) -> Option<GLAllocationParams> {
        unsafe {
            from_glib_full(ffi::gst_gl_buffer_pool_get_gl_allocation_params(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}