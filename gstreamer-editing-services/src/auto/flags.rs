// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{bitflags::bitflags, prelude::*, translate::*, GStr};
use std::fmt;

#[cfg(feature = "v1_20")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GESMarkerFlags")]
    pub struct MarkerFlags: u32 {
        #[doc(alias = "GES_MARKER_FLAG_NONE")]
        const NONE = ffi::GES_MARKER_FLAG_NONE as _;
        #[doc(alias = "GES_MARKER_FLAG_SNAPPABLE")]
        const SNAPPABLE = ffi::GES_MARKER_FLAG_SNAPPABLE as _;
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for MarkerFlags {
    type GlibType = ffi::GESMarkerFlags;

    #[inline]
    fn into_glib(self) -> ffi::GESMarkerFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GESMarkerFlags> for MarkerFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GESMarkerFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for MarkerFlags {
    #[inline]
    #[doc(alias = "ges_marker_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::ges_marker_flags_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for MarkerFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for MarkerFlags {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for MarkerFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for MarkerFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl From<MarkerFlags> for glib::Value {
    #[inline]
    fn from(v: MarkerFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GESMetaFlag")]
    pub struct MetaFlag: u32 {
        #[doc(alias = "GES_META_READABLE")]
        const READABLE = ffi::GES_META_READABLE as _;
        #[doc(alias = "GES_META_WRITABLE")]
        const WRITABLE = ffi::GES_META_WRITABLE as _;
        #[doc(alias = "GES_META_READ_WRITE")]
        const READWRITE = ffi::GES_META_READ_WRITE as _;
    }
}

#[doc(hidden)]
impl IntoGlib for MetaFlag {
    type GlibType = ffi::GESMetaFlag;

    #[inline]
    fn into_glib(self) -> ffi::GESMetaFlag {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESMetaFlag> for MetaFlag {
    #[inline]
    unsafe fn from_glib(value: ffi::GESMetaFlag) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for MetaFlag {
    #[inline]
    #[doc(alias = "ges_meta_flag_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::ges_meta_flag_get_type()) }
    }
}

impl glib::HasParamSpec for MetaFlag {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for MetaFlag {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for MetaFlag {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for MetaFlag {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<MetaFlag> for glib::Value {
    #[inline]
    fn from(v: MetaFlag) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GESPipelineFlags")]
    pub struct PipelineFlags: u32 {
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW_AUDIO")]
        const AUDIO_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW_AUDIO as _;
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW_VIDEO")]
        const VIDEO_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW_VIDEO as _;
        #[doc(alias = "GES_PIPELINE_MODE_PREVIEW")]
        const FULL_PREVIEW = ffi::GES_PIPELINE_MODE_PREVIEW as _;
        #[doc(alias = "GES_PIPELINE_MODE_RENDER")]
        const RENDER = ffi::GES_PIPELINE_MODE_RENDER as _;
        #[doc(alias = "GES_PIPELINE_MODE_SMART_RENDER")]
        const SMART_RENDER = ffi::GES_PIPELINE_MODE_SMART_RENDER as _;
    }
}

#[doc(hidden)]
impl IntoGlib for PipelineFlags {
    type GlibType = ffi::GESPipelineFlags;

    #[inline]
    fn into_glib(self) -> ffi::GESPipelineFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESPipelineFlags> for PipelineFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GESPipelineFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for PipelineFlags {
    #[inline]
    #[doc(alias = "ges_pipeline_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::ges_pipeline_flags_get_type()) }
    }
}

impl glib::HasParamSpec for PipelineFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for PipelineFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for PipelineFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for PipelineFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PipelineFlags> for glib::Value {
    #[inline]
    fn from(v: PipelineFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GESTrackType")]
    pub struct TrackType: u32 {
        #[doc(alias = "GES_TRACK_TYPE_UNKNOWN")]
        const UNKNOWN = ffi::GES_TRACK_TYPE_UNKNOWN as _;
        #[doc(alias = "GES_TRACK_TYPE_AUDIO")]
        const AUDIO = ffi::GES_TRACK_TYPE_AUDIO as _;
        #[doc(alias = "GES_TRACK_TYPE_VIDEO")]
        const VIDEO = ffi::GES_TRACK_TYPE_VIDEO as _;
        #[doc(alias = "GES_TRACK_TYPE_TEXT")]
        const TEXT = ffi::GES_TRACK_TYPE_TEXT as _;
        #[doc(alias = "GES_TRACK_TYPE_CUSTOM")]
        const CUSTOM = ffi::GES_TRACK_TYPE_CUSTOM as _;
    }
}

impl TrackType {
    pub fn name<'a>(self) -> &'a GStr {
        unsafe {
            GStr::from_ptr(
                ffi::ges_track_type_name(self.into_glib())
                    .as_ref()
                    .expect("ges_track_type_name returned NULL"),
            )
        }
    }
}

impl fmt::Display for TrackType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl IntoGlib for TrackType {
    type GlibType = ffi::GESTrackType;

    #[inline]
    fn into_glib(self) -> ffi::GESTrackType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESTrackType> for TrackType {
    #[inline]
    unsafe fn from_glib(value: ffi::GESTrackType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for TrackType {
    #[inline]
    #[doc(alias = "ges_track_type_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::ges_track_type_get_type()) }
    }
}

impl glib::HasParamSpec for TrackType {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for TrackType {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for TrackType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for TrackType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<TrackType> for glib::Value {
    #[inline]
    fn from(v: TrackType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
