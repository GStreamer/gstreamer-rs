// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, RTPHeaderExtensionDirection, RTPHeaderExtensionFlags};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstRTPHeaderExtension")]
    pub struct RTPHeaderExtension(Object<ffi::GstRTPHeaderExtension, ffi::GstRTPHeaderExtensionClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_rtp_header_extension_get_type(),
    }
}

impl RTPHeaderExtension {
    pub const NONE: Option<&'static RTPHeaderExtension> = None;

    #[doc(alias = "gst_rtp_header_extension_create_from_uri")]
    pub fn create_from_uri(uri: &str) -> Option<RTPHeaderExtension> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtp_header_extension_create_from_uri(
                uri.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for RTPHeaderExtension {}
unsafe impl Sync for RTPHeaderExtension {}

pub trait RTPHeaderExtensionExt: IsA<RTPHeaderExtension> + 'static {
    #[doc(alias = "gst_rtp_header_extension_get_direction")]
    #[doc(alias = "get_direction")]
    fn direction(&self) -> RTPHeaderExtensionDirection {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> u32 {
        unsafe { ffi::gst_rtp_header_extension_get_id(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtp_header_extension_get_max_size")]
    #[doc(alias = "get_max_size")]
    fn max_size(&self, input_meta: &gst::Buffer) -> usize {
        unsafe {
            ffi::gst_rtp_header_extension_get_max_size(
                self.as_ref().to_glib_none().0,
                input_meta.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gst_rtp_header_extension_get_sdp_caps_field_name")]
    #[doc(alias = "get_sdp_caps_field_name")]
    fn sdp_caps_field_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_rtp_header_extension_get_sdp_caps_field_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_get_supported_flags")]
    #[doc(alias = "get_supported_flags")]
    fn supported_flags(&self) -> RTPHeaderExtensionFlags {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_get_supported_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_rtp_header_extension_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_set_attributes_from_caps")]
    fn set_attributes_from_caps(&self, caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_set_attributes_from_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_set_direction")]
    fn set_direction(&self, direction: RTPHeaderExtensionDirection) {
        unsafe {
            ffi::gst_rtp_header_extension_set_direction(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtp_header_extension_set_id")]
    fn set_id(&self, ext_id: u32) {
        unsafe {
            ffi::gst_rtp_header_extension_set_id(self.as_ref().to_glib_none().0, ext_id);
        }
    }

    #[doc(alias = "gst_rtp_header_extension_set_non_rtp_sink_caps")]
    fn set_non_rtp_sink_caps(&self, caps: &gst::Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_set_non_rtp_sink_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtp_header_extension_set_wants_update_non_rtp_src_caps")]
    fn set_wants_update_non_rtp_src_caps(&self, state: bool) {
        unsafe {
            ffi::gst_rtp_header_extension_set_wants_update_non_rtp_src_caps(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtp_header_extension_wants_update_non_rtp_src_caps")]
    fn wants_update_non_rtp_src_caps(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtp_header_extension_wants_update_non_rtp_src_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<RTPHeaderExtension>> RTPHeaderExtensionExt for O {}
