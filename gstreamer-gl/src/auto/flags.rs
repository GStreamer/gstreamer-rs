// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use bitflags::bitflags;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

bitflags! {
    pub struct GLAPI: u32 {
        const OPENGL = 1;
        const OPENGL3 = 2;
        const GLES1 = 32768;
        const GLES2 = 65536;
    }
}

impl GLAPI {
    #[doc(alias = "gst_gl_api_from_string")]
    pub fn from_string(api_s: &str) -> GLAPI {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_gl_api_from_string(api_s.to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_api_to_string")]
    pub fn to_str(self) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_api_to_string(self.into_glib())) }
    }
}

impl fmt::Display for GLAPI {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for GLAPI {
    type GlibType = ffi::GstGLAPI;

    fn into_glib(self) -> ffi::GstGLAPI {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLAPI> for GLAPI {
    unsafe fn from_glib(value: ffi::GstGLAPI) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLAPI {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_api_get_type()) }
    }
}

impl glib::value::ValueType for GLAPI {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLAPI {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLAPI {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct GLDisplayType: u32 {
        const X11 = 1;
        const WAYLAND = 2;
        const COCOA = 4;
        const WIN32 = 8;
        const DISPMANX = 16;
        const EGL = 32;
        const VIV_FB = 64;
        const GBM = 128;
        #[cfg(any(feature = "v1_18", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
        const EGL_DEVICE = 256;
    }
}

#[doc(hidden)]
impl IntoGlib for GLDisplayType {
    type GlibType = ffi::GstGLDisplayType;

    fn into_glib(self) -> ffi::GstGLDisplayType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLDisplayType> for GLDisplayType {
    unsafe fn from_glib(value: ffi::GstGLDisplayType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLDisplayType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_display_type_get_type()) }
    }
}

impl glib::value::ValueType for GLDisplayType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLDisplayType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLDisplayType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct GLPlatform: u32 {
        const EGL = 1;
        const GLX = 2;
        const WGL = 4;
        const CGL = 8;
        const EAGL = 16;
    }
}

impl GLPlatform {
    #[doc(alias = "gst_gl_platform_from_string")]
    pub fn from_string(platform_s: &str) -> GLPlatform {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_gl_platform_from_string(
                platform_s.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_platform_to_string")]
    pub fn to_str(self) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_platform_to_string(self.into_glib())) }
    }
}

impl fmt::Display for GLPlatform {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for GLPlatform {
    type GlibType = ffi::GstGLPlatform;

    fn into_glib(self) -> ffi::GstGLPlatform {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLPlatform> for GLPlatform {
    unsafe fn from_glib(value: ffi::GstGLPlatform) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLPlatform {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_gl_platform_get_type()) }
    }
}

impl glib::value::ValueType for GLPlatform {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLPlatform {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLPlatform {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

bitflags! {
    pub struct GLSLProfile: u32 {
        const ES = 1;
        const CORE = 2;
        const COMPATIBILITY = 4;
    }
}

impl GLSLProfile {
    #[doc(alias = "gst_glsl_profile_from_string")]
    pub fn from_string(string: &str) -> GLSLProfile {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_glsl_profile_from_string(string.to_glib_none().0)) }
    }

    #[doc(alias = "gst_glsl_profile_to_string")]
    pub fn to_str(self) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_glsl_profile_to_string(self.into_glib())) }
    }
}

#[doc(hidden)]
impl IntoGlib for GLSLProfile {
    type GlibType = ffi::GstGLSLProfile;

    fn into_glib(self) -> ffi::GstGLSLProfile {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstGLSLProfile> for GLSLProfile {
    unsafe fn from_glib(value: ffi::GstGLSLProfile) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for GLSLProfile {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_glsl_profile_get_type()) }
    }
}

impl glib::value::ValueType for GLSLProfile {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for GLSLProfile {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for GLSLProfile {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
