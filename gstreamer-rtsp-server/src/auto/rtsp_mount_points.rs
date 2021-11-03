// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPMediaFactory;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    #[doc(alias = "GstRTSPMountPoints")]
    pub struct RTSPMountPoints(Object<ffi::GstRTSPMountPoints, ffi::GstRTSPMountPointsClass>);

    match fn {
        type_ => || ffi::gst_rtsp_mount_points_get_type(),
    }
}

impl RTSPMountPoints {
    #[doc(alias = "gst_rtsp_mount_points_new")]
    pub fn new() -> RTSPMountPoints {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_mount_points_new()) }
    }
}

impl Default for RTSPMountPoints {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMountPoints {}
unsafe impl Sync for RTSPMountPoints {}

impl RTSPMountPoints {
    pub const NONE: Option<&'static RTSPMountPoints> = None;
}

pub trait RTSPMountPointsExt: 'static {
    #[doc(alias = "gst_rtsp_mount_points_add_factory")]
    fn add_factory(&self, path: &str, factory: &impl IsA<RTSPMediaFactory>);

    #[doc(alias = "gst_rtsp_mount_points_make_path")]
    fn make_path(&self, url: &gst_rtsp::RTSPUrl) -> Result<glib::GString, glib::BoolError>;

    #[doc(alias = "gst_rtsp_mount_points_match")]
    #[doc(alias = "match")]
    fn match_(&self, path: &str) -> (RTSPMediaFactory, i32);

    #[doc(alias = "gst_rtsp_mount_points_remove_factory")]
    fn remove_factory(&self, path: &str);
}

impl<O: IsA<RTSPMountPoints>> RTSPMountPointsExt for O {
    fn add_factory(&self, path: &str, factory: &impl IsA<RTSPMediaFactory>) {
        unsafe {
            ffi::gst_rtsp_mount_points_add_factory(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                factory.as_ref().to_glib_full(),
            );
        }
    }

    fn make_path(&self, url: &gst_rtsp::RTSPUrl) -> Result<glib::GString, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtsp_mount_points_make_path(
                self.as_ref().to_glib_none().0,
                url.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to make path"))
        }
    }

    fn match_(&self, path: &str) -> (RTSPMediaFactory, i32) {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::gst_rtsp_mount_points_match(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            let matched = matched.assume_init();
            (ret, matched)
        }
    }

    fn remove_factory(&self, path: &str) {
        unsafe {
            ffi::gst_rtsp_mount_points_remove_factory(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }
}
