// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Element;
use crate::Object;
use crate::Toc;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct TocSetter(Interface<ffi::GstTocSetter, ffi::GstTocSetterInterface>) @requires Element, Object;

    match fn {
        get_type => || ffi::gst_toc_setter_get_type(),
    }
}

unsafe impl Send for TocSetter {}
unsafe impl Sync for TocSetter {}

pub const NONE_TOC_SETTER: Option<&TocSetter> = None;

pub trait TocSetterExt: 'static {
    #[doc(alias = "gst_toc_setter_get_toc")]
    fn get_toc(&self) -> Option<Toc>;

    #[doc(alias = "gst_toc_setter_reset")]
    fn reset(&self);

    #[doc(alias = "gst_toc_setter_set_toc")]
    fn set_toc(&self, toc: Option<&Toc>);
}

impl<O: IsA<TocSetter>> TocSetterExt for O {
    fn get_toc(&self) -> Option<Toc> {
        unsafe { from_glib_full(ffi::gst_toc_setter_get_toc(self.as_ref().to_glib_none().0)) }
    }

    fn reset(&self) {
        unsafe {
            ffi::gst_toc_setter_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_toc(&self, toc: Option<&Toc>) {
        unsafe {
            ffi::gst_toc_setter_set_toc(self.as_ref().to_glib_none().0, toc.to_glib_none().0);
        }
    }
}
