// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::*;
use gst::prelude::*;

use crate::{ffi, GLContext};

#[repr(transparent)]
#[doc(alias = "GstGLSyncMeta")]
pub struct GLSyncMeta(ffi::GstGLSyncMeta);

unsafe impl Send for GLSyncMeta {}
unsafe impl Sync for GLSyncMeta {}

impl GLSyncMeta {
    #[doc(alias = "gst_buffer_add_gl_sync_meta")]
    pub fn add<'a, C: IsA<GLContext>>(
        buffer: &'a mut gst::BufferRef,
        context: &C,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_gl_sync_meta(
                context.as_ref().to_glib_none().0,
                buffer.as_mut_ptr(),
            );
            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_context")]
    #[inline]
    pub fn context(&self) -> &GLContext {
        unsafe { &*(&self.0.context as *const *mut ffi::GstGLContext as *const GLContext) }
    }

    #[doc(alias = "gst_gl_sync_meta_set_sync_point")]
    pub fn set_sync_point(&self, context: &impl IsA<GLContext>) {
        unsafe {
            ffi::gst_gl_sync_meta_set_sync_point(
                mut_override(&self.0),
                context.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_sync_meta_wait")]
    pub fn wait(&self, context: &impl IsA<GLContext>) {
        unsafe {
            ffi::gst_gl_sync_meta_wait(mut_override(&self.0), context.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_sync_meta_wait_cpu")]
    pub fn wait_cpu(&self, context: &impl IsA<GLContext>) {
        unsafe {
            ffi::gst_gl_sync_meta_wait_cpu(
                mut_override(&self.0),
                context.as_ref().to_glib_none().0,
            );
        }
    }
}

unsafe impl MetaAPI for GLSyncMeta {
    type GstType = ffi::GstGLSyncMeta;

    #[doc(alias = "gst_gl_sync_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_gl_sync_meta_api_get_type()) }
    }
}

impl fmt::Debug for GLSyncMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GLSyncMeta")
            .field("context", &self.context())
            .finish()
    }
}
