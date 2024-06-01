// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, PlayerStreamInfo};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstPlayerVideoInfo")]
    pub struct PlayerVideoInfo(Object<ffi::GstPlayerVideoInfo, ffi::GstPlayerVideoInfoClass>) @extends PlayerStreamInfo;

    match fn {
        type_ => || ffi::gst_player_video_info_get_type(),
    }
}

impl PlayerVideoInfo {
    #[doc(alias = "gst_player_video_info_get_bitrate")]
    #[doc(alias = "get_bitrate")]
    pub fn bitrate(&self) -> i32 {
        unsafe { ffi::gst_player_video_info_get_bitrate(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_video_info_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gst_player_video_info_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_video_info_get_max_bitrate")]
    #[doc(alias = "get_max_bitrate")]
    pub fn max_bitrate(&self) -> i32 {
        unsafe { ffi::gst_player_video_info_get_max_bitrate(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_player_video_info_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gst_player_video_info_get_width(self.to_glib_none().0) }
    }
}

unsafe impl Send for PlayerVideoInfo {}
unsafe impl Sync for PlayerVideoInfo {}
