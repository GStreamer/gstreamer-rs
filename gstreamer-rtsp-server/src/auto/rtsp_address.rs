// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RTSPAddress(Boxed<ffi::GstRTSPAddress>);

    match fn {
        copy => |ptr| ffi::gst_rtsp_address_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_rtsp_address_free(ptr),
        type_ => || ffi::gst_rtsp_address_get_type(),
    }
}

unsafe impl Send for RTSPAddress {}
