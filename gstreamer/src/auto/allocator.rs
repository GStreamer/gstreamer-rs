// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, AllocationParams, Memory, Object};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstAllocator")]
    pub struct Allocator(Object<ffi::GstAllocator, ffi::GstAllocatorClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_allocator_get_type(),
    }
}

impl Allocator {
    pub const NONE: Option<&'static Allocator> = None;

    #[doc(alias = "gst_allocator_find")]
    pub fn find(name: Option<&str>) -> Option<Allocator> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_allocator_find(name.to_glib_none().0)) }
    }
}

unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

pub trait AllocatorExt: IsA<Allocator> + 'static {
    #[doc(alias = "gst_allocator_alloc")]
    fn alloc(
        &self,
        size: usize,
        params: Option<&AllocationParams>,
    ) -> Result<Memory, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_allocator_alloc(
                self.as_ref().to_glib_none().0,
                size,
                mut_override(params.to_glib_none().0),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate memory"))
        }
    }

    #[doc(alias = "gst_allocator_set_default")]
    fn set_default(self) {
        unsafe {
            ffi::gst_allocator_set_default(self.upcast().into_glib_ptr());
        }
    }
}

impl<O: IsA<Allocator>> AllocatorExt for O {}
