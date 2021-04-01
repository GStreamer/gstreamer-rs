// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioBaseSrc;

glib::wrapper! {
    pub struct AudioSrc(Object<ffi::GstAudioSrc, ffi::GstAudioSrcClass>) @extends AudioBaseSrc, gst_base::BaseSrc, gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_audio_src_get_type(),
    }
}

impl AudioSrc {}

unsafe impl Send for AudioSrc {}
unsafe impl Sync for AudioSrc {}

pub const NONE_AUDIO_SRC: Option<&AudioSrc> = None;
