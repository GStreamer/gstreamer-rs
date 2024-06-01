// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstVideoAggregatorPad")]
    pub struct VideoAggregatorPad(Object<ffi::GstVideoAggregatorPad, ffi::GstVideoAggregatorPadClass>) @extends gst_base::AggregatorPad, gst::Pad, gst::Object;

    match fn {
        type_ => || ffi::gst_video_aggregator_pad_get_type(),
    }
}

impl VideoAggregatorPad {
    pub const NONE: Option<&'static VideoAggregatorPad> = None;
}

unsafe impl Send for VideoAggregatorPad {}
unsafe impl Sync for VideoAggregatorPad {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoAggregatorPad>> Sealed for T {}
}

pub trait VideoAggregatorPadExt: IsA<VideoAggregatorPad> + sealed::Sealed + 'static {
    #[doc(alias = "gst_video_aggregator_pad_set_needs_alpha")]
    fn set_needs_alpha(&self, needs_alpha: bool) {
        unsafe {
            ffi::gst_video_aggregator_pad_set_needs_alpha(
                self.as_ref().to_glib_none().0,
                needs_alpha.into_glib(),
            );
        }
    }

    #[doc(alias = "max-last-buffer-repeat")]
    fn max_last_buffer_repeat(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "max-last-buffer-repeat")
    }

    #[doc(alias = "max-last-buffer-repeat")]
    fn set_max_last_buffer_repeat(&self, max_last_buffer_repeat: u64) {
        ObjectExt::set_property(
            self.as_ref(),
            "max-last-buffer-repeat",
            max_last_buffer_repeat,
        )
    }

    #[doc(alias = "repeat-after-eos")]
    fn is_repeat_after_eos(&self) -> bool {
        ObjectExt::property(self.as_ref(), "repeat-after-eos")
    }

    #[doc(alias = "repeat-after-eos")]
    fn set_repeat_after_eos(&self, repeat_after_eos: bool) {
        ObjectExt::set_property(self.as_ref(), "repeat-after-eos", repeat_after_eos)
    }

    fn zorder(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "zorder")
    }

    fn set_zorder(&self, zorder: u32) {
        ObjectExt::set_property(self.as_ref(), "zorder", zorder)
    }

    #[doc(alias = "max-last-buffer-repeat")]
    fn connect_max_last_buffer_repeat_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_last_buffer_repeat_trampoline<
            P: IsA<VideoAggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoAggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-last-buffer-repeat\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_last_buffer_repeat_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "repeat-after-eos")]
    fn connect_repeat_after_eos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_repeat_after_eos_trampoline<
            P: IsA<VideoAggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoAggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::repeat-after-eos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_repeat_after_eos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "zorder")]
    fn connect_zorder_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_zorder_trampoline<
            P: IsA<VideoAggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoAggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::zorder\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_zorder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<VideoAggregatorPad>> VideoAggregatorPadExt for O {}
