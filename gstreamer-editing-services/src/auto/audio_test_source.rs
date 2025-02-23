// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, AudioSource, Extractable, MetaContainer, Source, TimelineElement, TrackElement};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESAudioTestSource")]
    pub struct AudioTestSource(Object<ffi::GESAudioTestSource, ffi::GESAudioTestSourceClass>) @extends AudioSource, Source, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_audio_test_source_get_type(),
    }
}

impl AudioTestSource {
    pub const NONE: Option<&'static AudioTestSource> = None;
}

pub trait AudioTestSourceExt: IsA<AudioTestSource> + 'static {
    #[doc(alias = "ges_audio_test_source_get_freq")]
    #[doc(alias = "get_freq")]
    fn freq(&self) -> f64 {
        unsafe { ffi::ges_audio_test_source_get_freq(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_audio_test_source_get_volume")]
    #[doc(alias = "get_volume")]
    fn volume(&self) -> f64 {
        unsafe { ffi::ges_audio_test_source_get_volume(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_audio_test_source_set_freq")]
    fn set_freq(&self, freq: f64) {
        unsafe {
            ffi::ges_audio_test_source_set_freq(self.as_ref().to_glib_none().0, freq);
        }
    }

    #[doc(alias = "ges_audio_test_source_set_volume")]
    fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::ges_audio_test_source_set_volume(self.as_ref().to_glib_none().0, volume);
        }
    }
}

impl<O: IsA<AudioTestSource>> AudioTestSourceExt for O {}
