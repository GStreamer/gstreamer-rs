// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ops, ptr};

use crate::{ffi, prelude::*};
use glib::{prelude::*, translate::*};

#[derive(Debug)]
#[doc(alias = "GstVideoConverter")]
pub struct VideoConverter(ptr::NonNull<ffi::GstVideoConverter>);

impl Drop for VideoConverter {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_video_converter_free(self.0.as_ptr());
        }
    }
}

unsafe impl Send for VideoConverter {}
unsafe impl Sync for VideoConverter {}

impl VideoConverter {
    #[doc(alias = "gst_video_converter_new")]
    pub fn new(
        in_info: &crate::VideoInfo,
        out_info: &crate::VideoInfo,
        config: Option<VideoConverterConfig>,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();
        if in_info.fps() != out_info.fps() {
            return Err(glib::bool_error!("Can't do framerate conversion"));
        }

        if in_info.interlace_mode() != out_info.interlace_mode() {
            return Err(glib::bool_error!("Can't do interlacing conversion"));
        }

        unsafe {
            let ptr = ffi::gst_video_converter_new(
                in_info.to_glib_none().0 as *mut _,
                out_info.to_glib_none().0 as *mut _,
                config
                    .map(|s| s.0.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
            if ptr.is_null() {
                Err(glib::bool_error!("Failed to create video converter"))
            } else {
                Ok(Self(ptr::NonNull::new_unchecked(ptr)))
            }
        }
    }

    #[doc(alias = "get_config")]
    #[doc(alias = "gst_video_converter_get_config")]
    pub fn config(&self) -> VideoConverterConfig {
        unsafe {
            VideoConverterConfig(
                gst::StructureRef::from_glib_borrow(ffi::gst_video_converter_get_config(
                    self.0.as_ptr(),
                ))
                .to_owned(),
            )
        }
    }

    #[doc(alias = "gst_video_converter_set_config")]
    pub fn set_config(&mut self, config: VideoConverterConfig) {
        unsafe {
            ffi::gst_video_converter_set_config(self.0.as_ptr(), config.0.into_glib_ptr());
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "get_in_info")]
    #[doc(alias = "gst_video_converter_get_in_info")]
    pub fn in_info(&self) -> crate::VideoInfo {
        unsafe { from_glib_none(ffi::gst_video_converter_get_in_info(self.0.as_ptr())) }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "get_out_info")]
    #[doc(alias = "gst_video_converter_get_out_info")]
    pub fn out_info(&self) -> crate::VideoInfo {
        unsafe { from_glib_none(ffi::gst_video_converter_get_out_info(self.0.as_ptr())) }
    }

    #[doc(alias = "gst_video_converter_frame")]
    pub fn frame<T>(
        &self,
        src: &crate::VideoFrame<T>,
        dest: &mut crate::VideoFrame<crate::video_frame::Writable>,
    ) {
        unsafe {
            ffi::gst_video_converter_frame(self.0.as_ptr(), src.as_ptr(), dest.as_mut_ptr());
        }
    }

    pub fn frame_ref<T>(
        &self,
        src: &crate::VideoFrameRef<T>,
        dest: &mut crate::VideoFrameRef<&mut gst::BufferRef>,
    ) {
        unsafe {
            ffi::gst_video_converter_frame(self.0.as_ptr(), src.as_ptr(), dest.as_mut_ptr());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoConverterConfig(gst::Structure);

impl ops::Deref for VideoConverterConfig {
    type Target = gst::StructureRef;

    #[inline]
    fn deref(&self) -> &gst::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for VideoConverterConfig {
    #[inline]
    fn deref_mut(&mut self) -> &mut gst::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<gst::StructureRef> for VideoConverterConfig {
    #[inline]
    fn as_ref(&self) -> &gst::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<gst::StructureRef> for VideoConverterConfig {
    #[inline]
    fn as_mut(&mut self) -> &mut gst::StructureRef {
        self.0.as_mut()
    }
}

impl Default for VideoConverterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<gst::Structure> for VideoConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: gst::Structure) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v.name() == "GstVideoConverter" {
            Ok(Self(v))
        } else {
            Err(glib::bool_error!("Structure is no VideoConverterConfig"))
        }
    }
}

impl<'a> TryFrom<&'a gst::StructureRef> for VideoConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: &'a gst::StructureRef) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        Self::try_from(v.to_owned())
    }
}

impl From<VideoConverterConfig> for gst::Structure {
    fn from(v: VideoConverterConfig) -> Self {
        skip_assert_initialized!();
        v.0
    }
}

impl glib::value::ToValue for VideoConverterConfig {
    fn to_value(&self) -> glib::Value {
        self.0.to_value()
    }

    fn value_type(&self) -> glib::Type {
        self.0.value_type()
    }
}

impl glib::value::ToValueOptional for VideoConverterConfig {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        s.map(|s| &s.0).to_value()
    }
}

impl From<VideoConverterConfig> for glib::Value {
    fn from(s: VideoConverterConfig) -> glib::Value {
        skip_assert_initialized!();
        s.0.into()
    }
}

impl VideoConverterConfig {
    pub fn new() -> Self {
        Self(gst::Structure::new_empty("GstVideoConverter"))
    }

    pub fn set_resampler_method(&mut self, v: crate::VideoResamplerMethod) {
        self.0
            .set(glib::gstr!("GstVideoConverter.resampler-method"), v);
    }

    #[doc(alias = "get_resampler_method")]
    pub fn resampler_method(&self) -> crate::VideoResamplerMethod {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.resampler-method"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoResamplerMethod::Cubic)
    }

    pub fn set_chroma_resampler_method(&mut self, v: crate::VideoResamplerMethod) {
        self.0
            .set(glib::gstr!("GstVideoConverter.chroma-resampler-method"), v);
    }

    #[doc(alias = "get_chroma_resampler_method")]
    pub fn chroma_resampler_method(&self) -> crate::VideoResamplerMethod {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.chroma-resampler-method"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoResamplerMethod::Linear)
    }

    pub fn set_resampler_taps(&mut self, v: u32) {
        self.0
            .set(glib::gstr!("GstVideoConverter.resampler-taps"), v);
    }

    #[doc(alias = "get_resampler_taps")]
    pub fn resampler_taps(&self) -> u32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.resampler-taps"))
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dither_method(&mut self, v: crate::VideoDitherMethod) {
        self.0
            .set(glib::gstr!("GstVideoConverter.dither-method"), v);
    }

    #[doc(alias = "get_dither_method")]
    pub fn dither_method(&self) -> crate::VideoDitherMethod {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dither-method"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoDitherMethod::Bayer)
    }

    pub fn set_dither_quantization(&mut self, v: u32) {
        self.0
            .set(glib::gstr!("GstVideoConverter.dither-quantization"), v);
    }

    #[doc(alias = "get_dither_quantization")]
    pub fn dither_quantization(&self) -> u32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dither-quantization"))
            .expect("Wrong type")
            .unwrap_or(1)
    }

    pub fn set_src_x(&mut self, v: i32) {
        self.0.set(glib::gstr!("GstVideoConverter.src-x"), v);
    }

    #[doc(alias = "get_src_x")]
    pub fn src_x(&self) -> i32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.src-x"))
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_src_y(&mut self, v: i32) {
        self.0.set(glib::gstr!("GstVideoConverter.src-y"), v);
    }

    #[doc(alias = "get_src_y")]
    pub fn src_y(&self) -> i32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.src-y"))
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_src_width(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set(glib::gstr!("GstVideoConverter.src-width"), v);
        } else {
            self.0
                .remove_field(glib::gstr!("GstVideoConverter.src-width"));
        }
    }

    #[doc(alias = "get_src_width")]
    pub fn src_width(&self) -> Option<i32> {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.src-width"))
            .expect("Wrong type")
    }

    pub fn set_src_height(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set(glib::gstr!("GstVideoConverter.src-height"), v);
        } else {
            self.0
                .remove_field(glib::gstr!("GstVideoConverter.src-height"));
        }
    }

    #[doc(alias = "get_src_height")]
    pub fn src_height(&self) -> Option<i32> {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.src-height"))
            .expect("Wrong type")
    }

    pub fn set_dest_x(&mut self, v: i32) {
        self.0.set(glib::gstr!("GstVideoConverter.dest-x"), v);
    }

    #[doc(alias = "get_dest_x")]
    pub fn dest_x(&self) -> i32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dest-x"))
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dest_y(&mut self, v: i32) {
        self.0.set(glib::gstr!("GstVideoConverter.dest-y"), v);
    }

    #[doc(alias = "get_dest_y")]
    pub fn dest_y(&self) -> i32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dest-y"))
            .expect("Wrong type")
            .unwrap_or(0)
    }

    pub fn set_dest_width(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set(glib::gstr!("GstVideoConverter.dest-width"), v);
        } else {
            self.0
                .remove_field(glib::gstr!("GstVideoConverter.dest-width"));
        }
    }

    #[doc(alias = "get_dest_width")]
    pub fn dest_width(&self) -> Option<i32> {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dest-width"))
            .expect("Wrong type")
    }

    pub fn set_dest_height(&mut self, v: Option<i32>) {
        if let Some(v) = v {
            self.0.set(glib::gstr!("GstVideoConverter.dest-height"), v);
        } else {
            self.0
                .remove_field(glib::gstr!("GstVideoConverter.dest-height"));
        }
    }

    #[doc(alias = "get_dest_height")]
    pub fn dest_height(&self) -> Option<i32> {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.dest-height"))
            .expect("Wrong type")
    }

    pub fn set_fill_border(&mut self, v: bool) {
        self.0.set(glib::gstr!("GstVideoConverter.fill-border"), v);
    }

    #[doc(alias = "get_fill_border")]
    pub fn fills_border(&self) -> bool {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.fill-border"))
            .expect("Wrong type")
            .unwrap_or(true)
    }

    pub fn set_alpha_value(&mut self, v: f64) {
        self.0.set(glib::gstr!("GstVideoConverter.alpha-value"), v);
    }

    #[doc(alias = "get_alpha_value")]
    pub fn alpha_value(&self) -> f64 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.alpha-value"))
            .expect("Wrong type")
            .unwrap_or(1.0)
    }

    pub fn set_alpha_mode(&mut self, v: crate::VideoAlphaMode) {
        self.0.set(glib::gstr!("GstVideoConverter.alpha-mode"), v);
    }

    #[doc(alias = "get_alpha_mode")]
    pub fn alpha_mode(&self) -> crate::VideoAlphaMode {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.alpha-mode"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoAlphaMode::Copy)
    }

    pub fn set_border_argb(&mut self, v: u32) {
        self.0.set(glib::gstr!("GstVideoConverter.border-argb"), v);
    }

    #[doc(alias = "get_border_argb")]
    pub fn border_argb(&self) -> u32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.border-argb"))
            .expect("Wrong type")
            .unwrap_or(0xff_00_00_00)
    }

    pub fn set_chroma_mode(&mut self, v: crate::VideoChromaMode) {
        self.0.set(glib::gstr!("GstVideoConverter.chroma-mode"), v);
    }

    #[doc(alias = "get_chroma_mode")]
    pub fn chroma_mode(&self) -> crate::VideoChromaMode {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.chroma-mode"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoChromaMode::Full)
    }

    pub fn set_matrix_mode(&mut self, v: crate::VideoMatrixMode) {
        self.0.set(glib::gstr!("GstVideoConverter.matrix-mode"), v);
    }

    #[doc(alias = "get_matrix_mode")]
    pub fn matrix_mode(&self) -> crate::VideoMatrixMode {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.matrix-mode"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoMatrixMode::Full)
    }

    pub fn set_gamma_mode(&mut self, v: crate::VideoGammaMode) {
        self.0.set(glib::gstr!("GstVideoConverter.gamma-mode"), v);
    }

    #[doc(alias = "get_gamma_mode")]
    pub fn gamma_mode(&self) -> crate::VideoGammaMode {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.gamma-mode"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoGammaMode::None)
    }

    pub fn set_primaries_mode(&mut self, v: crate::VideoPrimariesMode) {
        self.0
            .set(glib::gstr!("GstVideoConverter.primaries-mode"), v);
    }

    #[doc(alias = "get_primaries_mode")]
    pub fn primaries_mode(&self) -> crate::VideoPrimariesMode {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.primaries-mode"))
            .expect("Wrong type")
            .unwrap_or(crate::VideoPrimariesMode::None)
    }

    pub fn set_threads(&mut self, v: u32) {
        self.0.set(glib::gstr!("GstVideoConverter.threads"), v);
    }

    #[doc(alias = "get_threads")]
    pub fn threads(&self) -> u32 {
        self.0
            .get_optional(glib::gstr!("GstVideoConverter.threads"))
            .expect("Wrong type")
            .unwrap_or(1)
    }
}
