// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::NetClientClock;

glib::wrapper! {
    pub struct NtpClock(Object<ffi::GstNtpClock, ffi::GstNtpClockClass>) @extends NetClientClock, gst::Clock, gst::Object;

    match fn {
        get_type => || ffi::gst_ntp_clock_get_type(),
    }
}

impl NtpClock {}

unsafe impl Send for NtpClock {}
unsafe impl Sync for NtpClock {}
