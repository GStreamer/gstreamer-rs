// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, VideoCodecFrame};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstVideoEncoder")]
    pub struct VideoEncoder(Object<ffi::GstVideoEncoder, ffi::GstVideoEncoderClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_video_encoder_get_type(),
    }
}

impl VideoEncoder {
    pub const NONE: Option<&'static VideoEncoder> = None;
}

unsafe impl Send for VideoEncoder {}
unsafe impl Sync for VideoEncoder {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoEncoder>> Sealed for T {}
}

pub trait VideoEncoderExt: IsA<VideoEncoder> + sealed::Sealed + 'static {
    #[doc(alias = "gst_video_encoder_allocate_output_buffer")]
    fn allocate_output_buffer(&self, size: usize) -> gst::Buffer {
        unsafe {
            from_glib_full(ffi::gst_video_encoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
        }
    }

    #[doc(alias = "gst_video_encoder_finish_frame")]
    fn finish_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_video_encoder_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_video_encoder_get_max_encode_time")]
    #[doc(alias = "get_max_encode_time")]
    fn max_encode_time(&self, frame: &VideoCodecFrame) -> gst::ClockTimeDiff {
        unsafe {
            ffi::gst_video_encoder_get_max_encode_time(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_video_encoder_get_min_force_key_unit_interval")]
    #[doc(alias = "get_min_force_key_unit_interval")]
    fn min_force_key_unit_interval(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_video_encoder_get_min_force_key_unit_interval(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_video_encoder_is_qos_enabled")]
    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_encoder_is_qos_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_video_encoder_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_video_encoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_video_encoder_proxy_getcaps")]
    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_video_encoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_video_encoder_set_min_force_key_unit_interval")]
    fn set_min_force_key_unit_interval(&self, interval: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_video_encoder_set_min_force_key_unit_interval(
                self.as_ref().to_glib_none().0,
                interval.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_video_encoder_set_min_pts")]
    fn set_min_pts(&self, min_pts: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            ffi::gst_video_encoder_set_min_pts(
                self.as_ref().to_glib_none().0,
                min_pts.into().into_glib(),
            );
        }
    }

    #[doc(alias = "gst_video_encoder_set_qos_enabled")]
    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_video_encoder_set_qos_enabled(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn is_qos(&self) -> bool {
        ObjectExt::property(self.as_ref(), "qos")
    }

    fn set_qos(&self, qos: bool) {
        ObjectExt::set_property(self.as_ref(), "qos", qos)
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "min-force-key-unit-interval")]
    fn connect_min_force_key_unit_interval_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_force_key_unit_interval_trampoline<
            P: IsA<VideoEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-force-key-unit-interval\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_min_force_key_unit_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "qos")]
    fn connect_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<
            P: IsA<VideoEncoder>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoEncoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoEncoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<VideoEncoder>> VideoEncoderExt for O {}
