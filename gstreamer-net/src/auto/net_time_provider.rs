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
    #[doc(alias = "GstNetTimeProvider")]
    pub struct NetTimeProvider(Object<ffi::GstNetTimeProvider, ffi::GstNetTimeProviderClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_net_time_provider_get_type(),
    }
}

impl NetTimeProvider {
    #[doc(alias = "gst_net_time_provider_new")]
    pub fn new(
        clock: &impl IsA<gst::Clock>,
        address: Option<&str>,
        port: i32,
    ) -> Result<NetTimeProvider, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_net_time_provider_new(
                clock.as_ref().to_glib_none().0,
                address.to_glib_none().0,
                port,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create NetTimeProvider"))
        }
    }

    pub fn is_active(&self) -> bool {
        ObjectExt::property(self, "active")
    }

    pub fn set_active(&self, active: bool) {
        ObjectExt::set_property(self, "active", active)
    }

    pub fn address(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "address")
    }

    pub fn clock(&self) -> Option<gst::Clock> {
        ObjectExt::property(self, "clock")
    }

    pub fn port(&self) -> i32 {
        ObjectExt::property(self, "port")
    }

    #[doc(alias = "qos-dscp")]
    pub fn qos_dscp(&self) -> i32 {
        ObjectExt::property(self, "qos-dscp")
    }

    #[doc(alias = "qos-dscp")]
    pub fn set_qos_dscp(&self, qos_dscp: i32) {
        ObjectExt::set_property(self, "qos-dscp", qos_dscp)
    }

    #[doc(alias = "active")]
    pub fn connect_active_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetTimeProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::active".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "qos-dscp")]
    pub fn connect_qos_dscp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_dscp_trampoline<
            F: Fn(&NetTimeProvider) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstNetTimeProvider,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::qos-dscp".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_qos_dscp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for NetTimeProvider {}
unsafe impl Sync for NetTimeProvider {}
