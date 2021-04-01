// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::DiscovererStreamInfo;
use glib::translate::*;

glib::wrapper! {
    pub struct DiscovererAudioInfo(Object<ffi::GstDiscovererAudioInfo>) @extends DiscovererStreamInfo;

    match fn {
        get_type => || ffi::gst_discoverer_audio_info_get_type(),
    }
}

impl DiscovererAudioInfo {
    #[doc(alias = "gst_discoverer_audio_info_get_bitrate")]
    pub fn get_bitrate(&self) -> u32 {
        unsafe { ffi::gst_discoverer_audio_info_get_bitrate(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_discoverer_audio_info_get_channel_mask")]
    pub fn get_channel_mask(&self) -> u64 {
        unsafe { ffi::gst_discoverer_audio_info_get_channel_mask(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_discoverer_audio_info_get_channels")]
    pub fn get_channels(&self) -> u32 {
        unsafe { ffi::gst_discoverer_audio_info_get_channels(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_discoverer_audio_info_get_depth")]
    pub fn get_depth(&self) -> u32 {
        unsafe { ffi::gst_discoverer_audio_info_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_discoverer_audio_info_get_language")]
    pub fn get_language(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_audio_info_get_language(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_discoverer_audio_info_get_max_bitrate")]
    pub fn get_max_bitrate(&self) -> u32 {
        unsafe { ffi::gst_discoverer_audio_info_get_max_bitrate(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_discoverer_audio_info_get_sample_rate")]
    pub fn get_sample_rate(&self) -> u32 {
        unsafe { ffi::gst_discoverer_audio_info_get_sample_rate(self.to_glib_none().0) }
    }
}

unsafe impl Send for DiscovererAudioInfo {}
unsafe impl Sync for DiscovererAudioInfo {}
