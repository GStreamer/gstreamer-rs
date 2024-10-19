// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, LFOWaveform};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstLFOControlSource")]
    pub struct LFOControlSource(Object<ffi::GstLFOControlSource, ffi::GstLFOControlSourceClass>) @extends gst::ControlSource, gst::Object;

    match fn {
        type_ => || ffi::gst_lfo_control_source_get_type(),
    }
}

impl LFOControlSource {
    pub const NONE: Option<&'static LFOControlSource> = None;

    #[doc(alias = "gst_lfo_control_source_new")]
    pub fn new() -> LFOControlSource {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlSource::from_glib_full(ffi::gst_lfo_control_source_new()).unsafe_cast()
        }
    }
}

impl Default for LFOControlSource {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for LFOControlSource {}
unsafe impl Sync for LFOControlSource {}

pub trait LFOControlSourceExt: IsA<LFOControlSource> + 'static {
    fn amplitude(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "amplitude")
    }

    fn set_amplitude(&self, amplitude: f64) {
        ObjectExt::set_property(self.as_ref(), "amplitude", amplitude)
    }

    fn frequency(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "frequency")
    }

    fn set_frequency(&self, frequency: f64) {
        ObjectExt::set_property(self.as_ref(), "frequency", frequency)
    }

    fn offset(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "offset")
    }

    fn set_offset(&self, offset: f64) {
        ObjectExt::set_property(self.as_ref(), "offset", offset)
    }

    fn timeshift(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "timeshift")
    }

    fn set_timeshift(&self, timeshift: u64) {
        ObjectExt::set_property(self.as_ref(), "timeshift", timeshift)
    }

    fn waveform(&self) -> LFOWaveform {
        ObjectExt::property(self.as_ref(), "waveform")
    }

    fn set_waveform(&self, waveform: LFOWaveform) {
        ObjectExt::set_property(self.as_ref(), "waveform", waveform)
    }

    #[doc(alias = "amplitude")]
    fn connect_amplitude_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_amplitude_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::amplitude\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_amplitude_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "frequency")]
    fn connect_frequency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_frequency_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::frequency\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_frequency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "offset")]
    fn connect_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timeshift")]
    fn connect_timeshift_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeshift_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeshift\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timeshift_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "waveform")]
    fn connect_waveform_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_waveform_trampoline<
            P: IsA<LFOControlSource>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstLFOControlSource,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(LFOControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::waveform\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_waveform_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<LFOControlSource>> LFOControlSourceExt for O {}
