// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{bitflags::bitflags, prelude::*, translate::*};

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstDiscovererSerializeFlags")]
    pub struct DiscovererSerializeFlags: u32 {
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_CAPS")]
        const CAPS = ffi::GST_DISCOVERER_SERIALIZE_CAPS as _;
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_TAGS")]
        const TAGS = ffi::GST_DISCOVERER_SERIALIZE_TAGS as _;
        #[doc(alias = "GST_DISCOVERER_SERIALIZE_MISC")]
        const MISC = ffi::GST_DISCOVERER_SERIALIZE_MISC as _;
    }
}

#[doc(hidden)]
impl IntoGlib for DiscovererSerializeFlags {
    type GlibType = ffi::GstDiscovererSerializeFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstDiscovererSerializeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDiscovererSerializeFlags> for DiscovererSerializeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstDiscovererSerializeFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for DiscovererSerializeFlags {
    #[inline]
    #[doc(alias = "gst_discoverer_serialize_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_discoverer_serialize_flags_get_type()) }
    }
}

impl glib::HasParamSpec for DiscovererSerializeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for DiscovererSerializeFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for DiscovererSerializeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for DiscovererSerializeFlags {
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

impl From<DiscovererSerializeFlags> for glib::Value {
    #[inline]
    fn from(v: DiscovererSerializeFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_20")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstPbUtilsCapsDescriptionFlags")]
    pub struct PbUtilsCapsDescriptionFlags: u32 {
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_CONTAINER")]
        const CONTAINER = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_CONTAINER as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_AUDIO")]
        const AUDIO = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_AUDIO as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_VIDEO")]
        const VIDEO = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_VIDEO as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_IMAGE")]
        const IMAGE = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_IMAGE as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_SUBTITLE")]
        const SUBTITLE = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_SUBTITLE as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_TAG")]
        const TAG = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_TAG as _;
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_GENERIC")]
        const GENERIC = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_GENERIC as _;
        #[cfg(feature = "v1_22")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
        #[doc(alias = "GST_PBUTILS_CAPS_DESCRIPTION_FLAG_METADATA")]
        const METADATA = ffi::GST_PBUTILS_CAPS_DESCRIPTION_FLAG_METADATA as _;
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for PbUtilsCapsDescriptionFlags {
    type GlibType = ffi::GstPbUtilsCapsDescriptionFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstPbUtilsCapsDescriptionFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstPbUtilsCapsDescriptionFlags> for PbUtilsCapsDescriptionFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstPbUtilsCapsDescriptionFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for PbUtilsCapsDescriptionFlags {
    #[inline]
    #[doc(alias = "gst_pb_utils_caps_description_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_pb_utils_caps_description_flags_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for PbUtilsCapsDescriptionFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for PbUtilsCapsDescriptionFlags {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for PbUtilsCapsDescriptionFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for PbUtilsCapsDescriptionFlags {
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
impl From<PbUtilsCapsDescriptionFlags> for glib::Value {
    #[inline]
    fn from(v: PbUtilsCapsDescriptionFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
