// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    pub struct VideoOrientation(Interface<ffi::GstVideoOrientation, ffi::GstVideoOrientationInterface>);

    match fn {
        get_type => || ffi::gst_video_orientation_get_type(),
    }
}

unsafe impl Send for VideoOrientation {}
unsafe impl Sync for VideoOrientation {}

pub const NONE_VIDEO_ORIENTATION: Option<&VideoOrientation> = None;

pub trait VideoOrientationExt: 'static {
    #[doc(alias = "gst_video_orientation_get_hcenter")]
    fn get_hcenter(&self) -> Option<i32>;

    #[doc(alias = "gst_video_orientation_get_hflip")]
    fn get_hflip(&self) -> Option<bool>;

    #[doc(alias = "gst_video_orientation_get_vcenter")]
    fn get_vcenter(&self) -> Option<i32>;

    #[doc(alias = "gst_video_orientation_get_vflip")]
    fn get_vflip(&self) -> Option<bool>;

    #[doc(alias = "gst_video_orientation_set_hcenter")]
    fn set_hcenter(&self, center: i32) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_video_orientation_set_hflip")]
    fn set_hflip(&self, flip: bool) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_video_orientation_set_vcenter")]
    fn set_vcenter(&self, center: i32) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_video_orientation_set_vflip")]
    fn set_vflip(&self, flip: bool) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<VideoOrientation>> VideoOrientationExt for O {
    fn get_hcenter(&self) -> Option<i32> {
        unsafe {
            let mut center = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_video_orientation_get_hcenter(
                self.as_ref().to_glib_none().0,
                center.as_mut_ptr(),
            ));
            let center = center.assume_init();
            if ret {
                Some(center)
            } else {
                None
            }
        }
    }

    fn get_hflip(&self) -> Option<bool> {
        unsafe {
            let mut flip = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_video_orientation_get_hflip(
                self.as_ref().to_glib_none().0,
                flip.as_mut_ptr(),
            ));
            let flip = flip.assume_init();
            if ret {
                Some(from_glib(flip))
            } else {
                None
            }
        }
    }

    fn get_vcenter(&self) -> Option<i32> {
        unsafe {
            let mut center = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_video_orientation_get_vcenter(
                self.as_ref().to_glib_none().0,
                center.as_mut_ptr(),
            ));
            let center = center.assume_init();
            if ret {
                Some(center)
            } else {
                None
            }
        }
    }

    fn get_vflip(&self) -> Option<bool> {
        unsafe {
            let mut flip = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_video_orientation_get_vflip(
                self.as_ref().to_glib_none().0,
                flip.as_mut_ptr(),
            ));
            let flip = flip.assume_init();
            if ret {
                Some(from_glib(flip))
            } else {
                None
            }
        }
    }

    fn set_hcenter(&self, center: i32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_orientation_set_hcenter(self.as_ref().to_glib_none().0, center),
                "Failed to set horizontal centering"
            )
        }
    }

    fn set_hflip(&self, flip: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_orientation_set_hflip(
                    self.as_ref().to_glib_none().0,
                    flip.to_glib()
                ),
                "Failed to set horizontal flipping"
            )
        }
    }

    fn set_vcenter(&self, center: i32) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_orientation_set_vcenter(self.as_ref().to_glib_none().0, center),
                "Failed to set vertical centering"
            )
        }
    }

    fn set_vflip(&self, flip: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_orientation_set_vflip(
                    self.as_ref().to_glib_none().0,
                    flip.to_glib()
                ),
                "Failed to set vertical flipping"
            )
        }
    }
}
