// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GstGLBaseMemoryAllocator")]
    pub struct GLBaseMemoryAllocator(Object<ffi::GstGLBaseMemoryAllocator, ffi::GstGLBaseMemoryAllocatorClass>) @extends gst::Allocator, gst::Object;

    match fn {
        type_ => || ffi::gst_gl_base_memory_allocator_get_type(),
    }
}

impl GLBaseMemoryAllocator {
    pub const NONE: Option<&'static GLBaseMemoryAllocator> = None;
}

unsafe impl Send for GLBaseMemoryAllocator {}
unsafe impl Sync for GLBaseMemoryAllocator {}
