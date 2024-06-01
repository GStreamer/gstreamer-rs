// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use crate::RTPHeaderExtension;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTPBasePayload")]
    pub struct RTPBasePayload(Object<ffi::GstRTPBasePayload, ffi::GstRTPBasePayloadClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_rtp_base_payload_get_type(),
    }
}

impl RTPBasePayload {
    pub const NONE: Option<&'static RTPBasePayload> = None;
}

unsafe impl Send for RTPBasePayload {}
unsafe impl Sync for RTPBasePayload {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTPBasePayload>> Sealed for T {}
}

pub trait RTPBasePayloadExt: IsA<RTPBasePayload> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_payload_allocate_output_buffer")]
    fn allocate_output_buffer(&self, payload_len: u32, pad_len: u8, csrc_count: u8) -> gst::Buffer {
        unsafe {
            from_glib_full(ffi::gst_rtp_base_payload_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                payload_len,
                pad_len,
                csrc_count,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_payload_get_source_count")]
    #[doc(alias = "get_source_count")]
    fn source_count(&self, buffer: &gst::Buffer) -> u32 {
        unsafe {
            ffi::gst_rtp_base_payload_get_source_count(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gst_rtp_base_payload_is_filled")]
    fn is_filled(&self, size: u32, duration: impl Into<Option<gst::ClockTime>>) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_base_payload_is_filled(
                self.as_ref().to_glib_none().0,
                size,
                duration.into().into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_payload_is_source_info_enabled")]
    fn is_source_info_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_base_payload_is_source_info_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_base_payload_push")]
    fn push(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_payload_push(
                self.as_ref().to_glib_none().0,
                buffer.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_rtp_base_payload_push_list")]
    fn push_list(&self, list: gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_payload_push_list(
                self.as_ref().to_glib_none().0,
                list.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_rtp_base_payload_set_options")]
    fn set_options(&self, media: &str, dynamic: bool, encoding_name: &str, clock_rate: u32) {
        unsafe {
            ffi::gst_rtp_base_payload_set_options(
                self.as_ref().to_glib_none().0,
                media.to_glib_none().0,
                dynamic.into_glib(),
                encoding_name.to_glib_none().0,
                clock_rate,
            );
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtp_base_payload_set_source_info_enabled")]
    fn set_source_info_enabled(&self, enable: bool) {
        unsafe {
            ffi::gst_rtp_base_payload_set_source_info_enabled(
                self.as_ref().to_glib_none().0,
                enable.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn is_auto_header_extension(&self) -> bool {
        ObjectExt::property(self.as_ref(), "auto-header-extension")
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn set_auto_header_extension(&self, auto_header_extension: bool) {
        ObjectExt::set_property(
            self.as_ref(),
            "auto-header-extension",
            auto_header_extension,
        )
    }

    #[doc(alias = "max-ptime")]
    fn max_ptime(&self) -> i64 {
        ObjectExt::property(self.as_ref(), "max-ptime")
    }

    #[doc(alias = "max-ptime")]
    fn set_max_ptime(&self, max_ptime: i64) {
        ObjectExt::set_property(self.as_ref(), "max-ptime", max_ptime)
    }

    #[doc(alias = "min-ptime")]
    fn min_ptime(&self) -> i64 {
        ObjectExt::property(self.as_ref(), "min-ptime")
    }

    #[doc(alias = "min-ptime")]
    fn set_min_ptime(&self, min_ptime: i64) {
        ObjectExt::set_property(self.as_ref(), "min-ptime", min_ptime)
    }

    fn mtu(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "mtu")
    }

    fn set_mtu(&self, mtu: u32) {
        ObjectExt::set_property(self.as_ref(), "mtu", mtu)
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "onvif-no-rate-control")]
    fn is_onvif_no_rate_control(&self) -> bool {
        ObjectExt::property(self.as_ref(), "onvif-no-rate-control")
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "onvif-no-rate-control")]
    fn set_onvif_no_rate_control(&self, onvif_no_rate_control: bool) {
        ObjectExt::set_property(
            self.as_ref(),
            "onvif-no-rate-control",
            onvif_no_rate_control,
        )
    }

    #[doc(alias = "perfect-rtptime")]
    fn is_perfect_rtptime(&self) -> bool {
        ObjectExt::property(self.as_ref(), "perfect-rtptime")
    }

    #[doc(alias = "perfect-rtptime")]
    fn set_perfect_rtptime(&self, perfect_rtptime: bool) {
        ObjectExt::set_property(self.as_ref(), "perfect-rtptime", perfect_rtptime)
    }

    fn pt(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "pt")
    }

    fn set_pt(&self, pt: u32) {
        ObjectExt::set_property(self.as_ref(), "pt", pt)
    }

    #[doc(alias = "ptime-multiple")]
    fn ptime_multiple(&self) -> i64 {
        ObjectExt::property(self.as_ref(), "ptime-multiple")
    }

    #[doc(alias = "ptime-multiple")]
    fn set_ptime_multiple(&self, ptime_multiple: i64) {
        ObjectExt::set_property(self.as_ref(), "ptime-multiple", ptime_multiple)
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scale-rtptime")]
    fn is_scale_rtptime(&self) -> bool {
        ObjectExt::property(self.as_ref(), "scale-rtptime")
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scale-rtptime")]
    fn set_scale_rtptime(&self, scale_rtptime: bool) {
        ObjectExt::set_property(self.as_ref(), "scale-rtptime", scale_rtptime)
    }

    fn seqnum(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "seqnum")
    }

    #[doc(alias = "seqnum-offset")]
    fn seqnum_offset(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "seqnum-offset")
    }

    #[doc(alias = "seqnum-offset")]
    fn set_seqnum_offset(&self, seqnum_offset: i32) {
        ObjectExt::set_property(self.as_ref(), "seqnum-offset", seqnum_offset)
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn is_source_info(&self) -> bool {
        ObjectExt::property(self.as_ref(), "source-info")
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn set_source_info(&self, source_info: bool) {
        ObjectExt::set_property(self.as_ref(), "source-info", source_info)
    }

    fn ssrc(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "ssrc")
    }

    fn set_ssrc(&self, ssrc: u32) {
        ObjectExt::set_property(self.as_ref(), "ssrc", ssrc)
    }

    fn stats(&self) -> Option<gst::Structure> {
        ObjectExt::property(self.as_ref(), "stats")
    }

    fn timestamp(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "timestamp")
    }

    #[doc(alias = "timestamp-offset")]
    fn timestamp_offset(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "timestamp-offset")
    }

    #[doc(alias = "timestamp-offset")]
    fn set_timestamp_offset(&self, timestamp_offset: u32) {
        ObjectExt::set_property(self.as_ref(), "timestamp-offset", timestamp_offset)
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "add-extension")]
    fn connect_add_extension<F: Fn(&Self, &RTPHeaderExtension) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_extension_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P, &RTPHeaderExtension) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            ext: *mut ffi::GstRTPHeaderExtension,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_full(ext),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-extension\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    add_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn emit_add_extension(&self, ext: &RTPHeaderExtension) {
        self.emit_by_name::<()>("add-extension", &[&ext]);
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "clear-extensions")]
    fn connect_clear_extensions<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn clear_extensions_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clear-extensions\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    clear_extensions_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn emit_clear_extensions(&self) {
        self.emit_by_name::<()>("clear-extensions", &[]);
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "request-extension")]
    fn connect_request_extension<
        F: Fn(&Self, u32, &str) -> Option<RTPHeaderExtension> + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn request_extension_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P, u32, &str) -> Option<RTPHeaderExtension> + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            ext_id: libc::c_uint,
            ext_uri: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::GstRTPHeaderExtension {
            let f: &F = &*(f as *const F);
            f(
                RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref(),
                ext_id,
                &glib::GString::from_glib_borrow(ext_uri),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"request-extension\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    request_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "auto-header-extension")]
    fn connect_auto_header_extension_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_header_extension_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-header-extension\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_auto_header_extension_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-ptime")]
    fn connect_max_ptime_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_ptime_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-ptime\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_ptime_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-ptime")]
    fn connect_min_ptime_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_ptime_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-ptime\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_ptime_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mtu")]
    fn connect_mtu_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mtu\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mtu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "onvif-no-rate-control")]
    fn connect_onvif_no_rate_control_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_onvif_no_rate_control_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::onvif-no-rate-control\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_onvif_no_rate_control_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "perfect-rtptime")]
    fn connect_perfect_rtptime_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_perfect_rtptime_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::perfect-rtptime\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_perfect_rtptime_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pt")]
    fn connect_pt_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pt_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pt\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pt_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ptime-multiple")]
    fn connect_ptime_multiple_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ptime_multiple_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ptime-multiple\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ptime_multiple_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scale-rtptime")]
    fn connect_scale_rtptime_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_rtptime_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-rtptime\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scale_rtptime_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "seqnum")]
    fn connect_seqnum_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seqnum_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seqnum\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_seqnum_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "seqnum-offset")]
    fn connect_seqnum_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_seqnum_offset_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seqnum-offset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_seqnum_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "source-info")]
    fn connect_source_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_info_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::source-info\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_source_info_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ssrc")]
    fn connect_ssrc_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ssrc_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ssrc\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_ssrc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stats")]
    fn connect_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stats_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stats\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_stats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timestamp")]
    fn connect_timestamp_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timestamp\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timestamp_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timestamp-offset")]
    fn connect_timestamp_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_timestamp_offset_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timestamp-offset\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timestamp_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTPBasePayload>> RTPBasePayloadExt for O {}
