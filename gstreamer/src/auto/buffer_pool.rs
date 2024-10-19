// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Buffer, Object};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstBufferPool")]
    pub struct BufferPool(Object<ffi::GstBufferPool, ffi::GstBufferPoolClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_buffer_pool_get_type(),
    }
}

impl BufferPool {
    pub const NONE: Option<&'static BufferPool> = None;

    #[doc(alias = "gst_buffer_pool_new")]
    pub fn new() -> BufferPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_pool_new()) }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for BufferPool {}
unsafe impl Sync for BufferPool {}

pub trait BufferPoolExt: IsA<BufferPool> + 'static {
    #[doc(alias = "gst_buffer_pool_get_options")]
    #[doc(alias = "get_options")]
    fn options(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_buffer_pool_get_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_buffer_pool_has_option")]
    fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_has_option(
                self.as_ref().to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_buffer_pool_is_active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_buffer_pool_release_buffer")]
    fn release_buffer(&self, buffer: Buffer) {
        unsafe {
            ffi::gst_buffer_pool_release_buffer(
                self.as_ref().to_glib_none().0,
                buffer.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gst_buffer_pool_set_active")]
    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_set_active(self.as_ref().to_glib_none().0, active.into_glib()),
                "Failed to activate buffer pool"
            )
        }
    }

    #[doc(alias = "gst_buffer_pool_set_flushing")]
    fn set_flushing(&self, flushing: bool) {
        unsafe {
            ffi::gst_buffer_pool_set_flushing(self.as_ref().to_glib_none().0, flushing.into_glib());
        }
    }
}

impl<O: IsA<BufferPool>> BufferPoolExt for O {}
