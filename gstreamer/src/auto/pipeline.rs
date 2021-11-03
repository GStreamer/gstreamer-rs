// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Bin;
use crate::ChildProxy;
use crate::Clock;
use crate::ClockTime;
use crate::Element;
use crate::Object;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstPipeline")]
    pub struct Pipeline(Object<ffi::GstPipeline, ffi::GstPipelineClass>) @extends Bin, Element, Object, @implements ChildProxy;

    match fn {
        type_ => || ffi::gst_pipeline_get_type(),
    }
}

impl Pipeline {
    #[doc(alias = "gst_pipeline_new")]
    pub fn new(name: Option<&str>) -> Pipeline {
        assert_initialized_main_thread!();
        unsafe {
            Element::from_glib_none(ffi::gst_pipeline_new(name.to_glib_none().0)).unsafe_cast()
        }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

impl Pipeline {
    pub const NONE: Option<&'static Pipeline> = None;
}

pub trait PipelineExt: 'static {
    #[doc(alias = "gst_pipeline_auto_clock")]
    fn auto_clock(&self);

    #[doc(alias = "gst_pipeline_get_auto_flush_bus")]
    #[doc(alias = "get_auto_flush_bus")]
    fn is_auto_flush_bus(&self) -> bool;

    #[doc(alias = "gst_pipeline_get_delay")]
    #[doc(alias = "get_delay")]
    fn delay(&self) -> ClockTime;

    #[doc(alias = "gst_pipeline_get_latency")]
    #[doc(alias = "get_latency")]
    fn latency(&self) -> Option<ClockTime>;

    #[doc(alias = "gst_pipeline_get_pipeline_clock")]
    #[doc(alias = "get_pipeline_clock")]
    fn pipeline_clock(&self) -> Clock;

    #[doc(alias = "gst_pipeline_set_auto_flush_bus")]
    fn set_auto_flush_bus(&self, auto_flush: bool);

    #[doc(alias = "gst_pipeline_set_delay")]
    fn set_delay(&self, delay: ClockTime);

    #[doc(alias = "gst_pipeline_set_latency")]
    fn set_latency(&self, latency: impl Into<Option<ClockTime>>);

    #[doc(alias = "gst_pipeline_use_clock")]
    fn use_clock(&self, clock: Option<&impl IsA<Clock>>);

    #[doc(alias = "auto-flush-bus")]
    fn connect_auto_flush_bus_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "delay")]
    fn connect_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "latency")]
    fn connect_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<Pipeline>> PipelineExt for O {
    fn auto_clock(&self) {
        unsafe {
            ffi::gst_pipeline_auto_clock(self.as_ref().to_glib_none().0);
        }
    }

    fn is_auto_flush_bus(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pipeline_get_auto_flush_bus(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn delay(&self) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_pipeline_get_delay(self.as_ref().to_glib_none().0))
                .expect("mandatory glib value is None")
        }
    }

    fn latency(&self) -> Option<ClockTime> {
        unsafe {
            from_glib(ffi::gst_pipeline_get_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pipeline_clock(&self) -> Clock {
        unsafe {
            from_glib_full(ffi::gst_pipeline_get_pipeline_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_auto_flush_bus(&self, auto_flush: bool) {
        unsafe {
            ffi::gst_pipeline_set_auto_flush_bus(
                self.as_ref().to_glib_none().0,
                auto_flush.into_glib(),
            );
        }
    }

    fn set_delay(&self, delay: ClockTime) {
        unsafe {
            ffi::gst_pipeline_set_delay(self.as_ref().to_glib_none().0, delay.into_glib());
        }
    }

    fn set_latency(&self, latency: impl Into<Option<ClockTime>>) {
        unsafe {
            ffi::gst_pipeline_set_latency(
                self.as_ref().to_glib_none().0,
                latency.into().into_glib(),
            );
        }
    }

    fn use_clock(&self, clock: Option<&impl IsA<Clock>>) {
        unsafe {
            ffi::gst_pipeline_use_clock(
                self.as_ref().to_glib_none().0,
                clock.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

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
                b"notify::auto-flush-bus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_flush_bus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
