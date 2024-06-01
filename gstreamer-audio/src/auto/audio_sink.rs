// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, AudioBaseSink};

glib::wrapper! {
    #[doc(alias = "GstAudioSink")]
    pub struct AudioSink(Object<ffi::GstAudioSink, ffi::GstAudioSinkClass>) @extends AudioBaseSink, gst_base::BaseSink, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_sink_get_type(),
    }
}

impl AudioSink {
    pub const NONE: Option<&'static AudioSink> = None;
}

unsafe impl Send for AudioSink {}
unsafe impl Sync for AudioSink {}
