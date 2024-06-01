// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, WebRTCDataChannelState, WebRTCPriorityType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstWebRTCDataChannel")]
    pub struct WebRTCDataChannel(Object<ffi::GstWebRTCDataChannel, ffi::GstWebRTCDataChannelClass>);

    match fn {
        type_ => || ffi::gst_webrtc_data_channel_get_type(),
    }
}

impl WebRTCDataChannel {
    #[doc(alias = "gst_webrtc_data_channel_close")]
    pub fn close(&self) {
        unsafe {
            ffi::gst_webrtc_data_channel_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_webrtc_data_channel_send_data")]
    pub fn send_data(&self, data: Option<&glib::Bytes>) {
        unsafe {
            ffi::gst_webrtc_data_channel_send_data(self.to_glib_none().0, data.to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_data_channel_send_data_full")]
    pub fn send_data_full(&self, data: Option<&glib::Bytes>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gst_webrtc_data_channel_send_data_full(
                self.to_glib_none().0,
                data.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_webrtc_data_channel_send_string")]
    pub fn send_string(&self, str: Option<&str>) {
        unsafe {
            ffi::gst_webrtc_data_channel_send_string(self.to_glib_none().0, str.to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_webrtc_data_channel_send_string_full")]
    pub fn send_string_full(&self, str: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gst_webrtc_data_channel_send_string_full(
                self.to_glib_none().0,
                str.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "buffered-amount")]
    pub fn buffered_amount(&self) -> u64 {
        ObjectExt::property(self, "buffered-amount")
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn buffered_amount_low_threshold(&self) -> u64 {
        ObjectExt::property(self, "buffered-amount-low-threshold")
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn set_buffered_amount_low_threshold(&self, buffered_amount_low_threshold: u64) {
        ObjectExt::set_property(
            self,
            "buffered-amount-low-threshold",
            buffered_amount_low_threshold,
        )
    }

    pub fn id(&self) -> i32 {
        ObjectExt::property(self, "id")
    }

    pub fn label(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "label")
    }

    #[doc(alias = "max-packet-lifetime")]
    pub fn max_packet_lifetime(&self) -> i32 {
        ObjectExt::property(self, "max-packet-lifetime")
    }

    #[doc(alias = "max-retransmits")]
    pub fn max_retransmits(&self) -> i32 {
        ObjectExt::property(self, "max-retransmits")
    }

    pub fn is_negotiated(&self) -> bool {
        ObjectExt::property(self, "negotiated")
    }

    pub fn is_ordered(&self) -> bool {
        ObjectExt::property(self, "ordered")
    }

    pub fn priority(&self) -> WebRTCPriorityType {
        ObjectExt::property(self, "priority")
    }

    pub fn protocol(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "protocol")
    }

    #[doc(alias = "ready-state")]
    pub fn ready_state(&self) -> WebRTCDataChannelState {
        ObjectExt::property(self, "ready-state")
    }

    #[doc(alias = "close")]
    pub fn connect_close<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn close_trampoline<F: Fn(&WebRTCDataChannel) + Send + Sync + 'static>(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"close\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_close(&self) {
        self.emit_by_name::<()>("close", &[]);
    }

    #[doc(alias = "on-buffered-amount-low")]
    pub fn connect_on_buffered_amount_low<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_buffered_amount_low_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-buffered-amount-low\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_buffered_amount_low_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-close")]
    pub fn connect_on_close<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn on_close_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-close\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-error")]
    pub fn connect_on_error<F: Fn(&Self, &glib::Error) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_error_trampoline<
            F: Fn(&WebRTCDataChannel, &glib::Error) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(error))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-error\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-message-data")]
    pub fn connect_on_message_data<F: Fn(&Self, Option<&glib::Bytes>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut glib::ffi::GBytes,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-data\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_message_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-message-string")]
    pub fn connect_on_message_string<F: Fn(&Self, Option<&str>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn on_message_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref()
                    .map(|s| s.as_str()),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-message-string\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_message_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "on-open")]
    pub fn connect_on_open<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn on_open_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"on-open\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    on_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "send-data")]
    pub fn connect_send_data<F: Fn(&Self, Option<&glib::Bytes>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_data_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&glib::Bytes>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut glib::ffi::GBytes,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Bytes>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-data\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    send_data_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_data(&self, data: Option<&glib::Bytes>) {
        self.emit_by_name::<()>("send-data", &[&data]);
    }

    #[doc(alias = "send-string")]
    pub fn connect_send_string<F: Fn(&Self, Option<&str>) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn send_string_trampoline<
            F: Fn(&WebRTCDataChannel, Option<&str>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
            data: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::GString>::from_glib_borrow(data)
                    .as_ref()
                    .as_ref()
                    .map(|s| s.as_str()),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"send-string\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    send_string_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn emit_send_string(&self, data: Option<&str>) {
        self.emit_by_name::<()>("send-string", &[&data]);
    }

    #[doc(alias = "buffered-amount")]
    pub fn connect_buffered_amount_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
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
                b"notify::buffered-amount\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_buffered_amount_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffered-amount-low-threshold")]
    pub fn connect_buffered_amount_low_threshold_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_buffered_amount_low_threshold_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
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
                b"notify::buffered-amount-low-threshold\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_buffered_amount_low_threshold_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ready-state")]
    pub fn connect_ready_state_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ready_state_trampoline<
            F: Fn(&WebRTCDataChannel) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstWebRTCDataChannel,
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
                b"notify::ready-state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ready_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for WebRTCDataChannel {}
unsafe impl Sync for WebRTCDataChannel {}
