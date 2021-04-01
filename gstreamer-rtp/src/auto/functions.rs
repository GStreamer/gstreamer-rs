// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTCPSDESType;
use glib::translate::*;

//#[cfg(any(feature = "v1_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
//#[doc(alias = "gst_buffer_add_rtp_source_meta")]
//pub fn buffer_add_rtp_source_meta(buffer: &gst::Buffer, ssrc: u32, csrc: u32, csrc_count: u32) -> /*Ignored*/Option<RTPSourceMeta> {
//    unsafe { TODO: call ffi:gst_buffer_add_rtp_source_meta() }
//}

//#[cfg(any(feature = "v1_16", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
//#[doc(alias = "gst_buffer_get_rtp_source_meta")]
//pub fn buffer_get_rtp_source_meta(buffer: &gst::Buffer) -> /*Ignored*/Option<RTPSourceMeta> {
//    unsafe { TODO: call ffi:gst_buffer_get_rtp_source_meta() }
//}

#[doc(alias = "gst_rtcp_ntp_to_unix")]
pub fn rtcp_ntp_to_unix(ntptime: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_rtcp_ntp_to_unix(ntptime) }
}

#[doc(alias = "gst_rtcp_sdes_name_to_type")]
pub fn rtcp_sdes_name_to_type(name: &str) -> RTCPSDESType {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gst_rtcp_sdes_name_to_type(name.to_glib_none().0)) }
}

#[doc(alias = "gst_rtcp_sdes_type_to_name")]
pub fn rtcp_sdes_type_to_name(type_: RTCPSDESType) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gst_rtcp_sdes_type_to_name(type_.to_glib())) }
}

#[doc(alias = "gst_rtcp_unix_to_ntp")]
pub fn rtcp_unix_to_ntp(unixtime: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_rtcp_unix_to_ntp(unixtime) }
}

//#[doc(alias = "gst_rtp_hdrext_set_ntp_56")]
//pub fn rtp_hdrext_set_ntp_56(data: /*Unimplemented*/Option<Fundamental: Pointer>, size: u32, ntptime: u64) -> bool {
//    unsafe { TODO: call ffi:gst_rtp_hdrext_set_ntp_56() }
//}

//#[doc(alias = "gst_rtp_hdrext_set_ntp_64")]
//pub fn rtp_hdrext_set_ntp_64(data: /*Unimplemented*/Option<Fundamental: Pointer>, size: u32, ntptime: u64) -> bool {
//    unsafe { TODO: call ffi:gst_rtp_hdrext_set_ntp_64() }
//}
