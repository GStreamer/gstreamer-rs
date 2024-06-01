// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Object, Pad, ProxyPad};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstGhostPad")]
    pub struct GhostPad(Object<ffi::GstGhostPad, ffi::GstGhostPadClass>) @extends ProxyPad, Pad, Object;

    match fn {
        type_ => || ffi::gst_ghost_pad_get_type(),
    }
}

impl GhostPad {
    pub const NONE: Option<&'static GhostPad> = None;
}

unsafe impl Send for GhostPad {}
unsafe impl Sync for GhostPad {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GhostPad>> Sealed for T {}
}

pub trait GhostPadExt: IsA<GhostPad> + sealed::Sealed + 'static {
    #[doc(alias = "gst_ghost_pad_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_ghost_pad_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_ghost_pad_set_target")]
    fn set_target(&self, newtarget: Option<&impl IsA<Pad>>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_ghost_pad_set_target(
                    self.as_ref().to_glib_none().0,
                    newtarget.map(|p| p.as_ref()).to_glib_none().0
                ),
                "Failed to set target"
            )
        }
    }
}

impl<O: IsA<GhostPad>> GhostPadExt for O {}
