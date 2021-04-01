// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;

bitflags! {
    pub struct RTSPAddressFlags: u32 {
        const IPV4 = 1;
        const IPV6 = 2;
        const EVEN_PORT = 4;
        const MULTICAST = 8;
        const UNICAST = 16;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPAddressFlags {
    type GlibType = ffi::GstRTSPAddressFlags;

    fn to_glib(&self) -> ffi::GstRTSPAddressFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPAddressFlags> for RTSPAddressFlags {
    unsafe fn from_glib(value: ffi::GstRTSPAddressFlags) -> RTSPAddressFlags {
        skip_assert_initialized!();
        RTSPAddressFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct RTSPTransportMode: u32 {
        const PLAY = 1;
        const RECORD = 2;
    }
}

#[doc(hidden)]
impl ToGlib for RTSPTransportMode {
    type GlibType = ffi::GstRTSPTransportMode;

    fn to_glib(&self) -> ffi::GstRTSPTransportMode {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstRTSPTransportMode> for RTSPTransportMode {
    unsafe fn from_glib(value: ffi::GstRTSPTransportMode) -> RTSPTransportMode {
        skip_assert_initialized!();
        RTSPTransportMode::from_bits_truncate(value)
    }
}

impl StaticType for RTSPTransportMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_rtsp_transport_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RTSPTransportMode {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RTSPTransportMode {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for RTSPTransportMode {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
