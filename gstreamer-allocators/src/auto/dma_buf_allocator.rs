// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, FdAllocator};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstDmaBufAllocator")]
    pub struct DmaBufAllocator(Object<ffi::GstDmaBufAllocator, ffi::GstDmaBufAllocatorClass>) @extends FdAllocator, gst::Allocator;

    match fn {
        type_ => || ffi::gst_dmabuf_allocator_get_type(),
    }
}

impl DmaBufAllocator {
    pub const NONE: Option<&'static DmaBufAllocator> = None;

    #[doc(alias = "gst_dmabuf_allocator_new")]
    pub fn new() -> DmaBufAllocator {
        assert_initialized_main_thread!();
        unsafe { gst::Allocator::from_glib_full(ffi::gst_dmabuf_allocator_new()).unsafe_cast() }
    }
}

impl Default for DmaBufAllocator {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for DmaBufAllocator {}
unsafe impl Sync for DmaBufAllocator {}
