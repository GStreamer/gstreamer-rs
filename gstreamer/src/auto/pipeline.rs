// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Bin, ChildProxy, Clock, ClockTime, Element, Object};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstPipeline")]
    pub struct Pipeline(Object<ffi::GstPipeline, ffi::GstPipelineClass>) @extends Bin, Element, Object, @implements ChildProxy;

    match fn {
        type_ => || ffi::gst_pipeline_get_type(),
    }
}

impl Pipeline {
    pub const NONE: Option<&'static Pipeline> = None;
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

pub trait PipelineExt: IsA<Pipeline> + 'static {
    #[doc(alias = "gst_pipeline_auto_clock")]
    fn auto_clock(&self) {
        unsafe {
            ffi::gst_pipeline_auto_clock(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_pipeline_get_auto_flush_bus")]
    #[doc(alias = "get_auto_flush_bus")]
    #[doc(alias = "auto-flush-bus")]
    fn is_auto_flush_bus(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pipeline_get_auto_flush_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_pipeline_get_configured_latency")]
    #[doc(alias = "get_configured_latency")]
    fn configured_latency(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_pipeline_get_configured_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_pipeline_get_delay")]
    #[doc(alias = "get_delay")]
    fn delay(&self) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_pipeline_get_delay(self.as_ref().to_glib_none().0))
                .expect("mandatory glib value is None")
        }
    }

    #[doc(alias = "gst_pipeline_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_pipeline_get_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_pipeline_get_pipeline_clock")]
    #[doc(alias = "get_pipeline_clock")]
    fn pipeline_clock(&self) -> Clock {
        unsafe {
            from_glib_full(ffi::gst_pipeline_get_pipeline_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_pipeline_is_live")]
    fn is_live(&self) -> bool {
        unsafe { from_glib(ffi::gst_pipeline_is_live(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_pipeline_set_auto_flush_bus")]
    #[doc(alias = "auto-flush-bus")]
    fn set_auto_flush_bus(&self, auto_flush: bool) {
        unsafe {
            ffi::gst_pipeline_set_auto_flush_bus(
                self.as_ref().to_glib_none().0,
                auto_flush.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_pipeline_set_delay")]
    #[doc(alias = "delay")]
    fn set_delay(&self, delay: ClockTime) {
        unsafe {
            ffi::gst_pipeline_set_delay(self.as_ref().to_glib_none().0, delay.into_glib());
        }
    }

    #[doc(alias = "gst_pipeline_set_latency")]
    #[doc(alias = "latency")]
    fn set_latency(&self, latency: impl Into<Option<ClockTime>>) {
        unsafe {
            ffi::gst_pipeline_set_latency(
                self.as_ref().to_glib_none().0,
                latency.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_pipeline_use_clock")]
    fn use_clock(&self, clock: Option<&impl IsA<Clock>>) {
        unsafe {
            ffi::gst_pipeline_use_clock(
                self.as_ref().to_glib_none().0,
                clock.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "auto-flush-bus")]
    fn connect_auto_flush_bus_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_flush_bus_trampoline<
            P: IsA<Pipeline>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::auto-flush-bus".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_auto_flush_bus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "delay")]
    fn connect_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_delay_trampoline<
            P: IsA<Pipeline>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::delay".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "latency")]
    fn connect_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_latency_trampoline<
            P: IsA<Pipeline>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPipeline,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Pipeline::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::latency".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Pipeline>> PipelineExt for O {}
