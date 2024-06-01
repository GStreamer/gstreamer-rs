// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Extractable, MetaContainer, Operation, TimelineElement, TrackElement, Transition,
};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESAudioTransition")]
    pub struct AudioTransition(Object<ffi::GESAudioTransition, ffi::GESAudioTransitionClass>) @extends Transition, Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_audio_transition_get_type(),
    }
}

impl AudioTransition {
    pub const NONE: Option<&'static AudioTransition> = None;

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "ges_audio_transition_new")]
    pub fn new() -> AudioTransition {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_audio_transition_new()) }
    }
}

impl Default for AudioTransition {
    fn default() -> Self {
        Self::new()
    }
}
