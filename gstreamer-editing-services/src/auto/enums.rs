// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::ffi::CStr;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GESEdge")]
pub enum Edge {
    #[doc(alias = "GES_EDGE_START")]
    Start,
    #[doc(alias = "GES_EDGE_END")]
    End,
    #[doc(alias = "GES_EDGE_NONE")]
    None,
    #[doc(hidden)]
    __Unknown(i32),
}

impl Edge {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::ges_edge_name(self.to_glib())
                    .as_ref()
                    .expect("ges_edge_name returned NULL"),
            )
            .to_str()
            .expect("ges_edge_name returned an invalid string")
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl fmt::Display for Edge {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl ToGlib for Edge {
    type GlibType = ffi::GESEdge;

    fn to_glib(&self) -> ffi::GESEdge {
        match *self {
            Edge::Start => ffi::GES_EDGE_START,
            Edge::End => ffi::GES_EDGE_END,
            Edge::None => ffi::GES_EDGE_NONE,
            Edge::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESEdge> for Edge {
    unsafe fn from_glib(value: ffi::GESEdge) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Edge::Start,
            1 => Edge::End,
            2 => Edge::None,
            value => Edge::__Unknown(value),
        }
    }
}

impl StaticType for Edge {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_edge_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Edge {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Edge {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Edge {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GESEditMode")]
pub enum EditMode {
    #[doc(alias = "GES_EDIT_MODE_NORMAL")]
    Normal,
    #[doc(alias = "GES_EDIT_MODE_RIPPLE")]
    Ripple,
    #[doc(alias = "GES_EDIT_MODE_ROLL")]
    Roll,
    #[doc(alias = "GES_EDIT_MODE_TRIM")]
    Trim,
    #[doc(alias = "GES_EDIT_MODE_SLIDE")]
    Slide,
    #[doc(hidden)]
    __Unknown(i32),
}

impl EditMode {
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn name<'a>(self) -> &'a str {
        unsafe {
            CStr::from_ptr(
                ffi::ges_edit_mode_name(self.to_glib())
                    .as_ref()
                    .expect("ges_edit_mode_name returned NULL"),
            )
            .to_str()
            .expect("ges_edit_mode_name returned an invalid string")
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl fmt::Display for EditMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name())
    }
}

#[doc(hidden)]
impl ToGlib for EditMode {
    type GlibType = ffi::GESEditMode;

    fn to_glib(&self) -> ffi::GESEditMode {
        match *self {
            EditMode::Normal => ffi::GES_EDIT_MODE_NORMAL,
            EditMode::Ripple => ffi::GES_EDIT_MODE_RIPPLE,
            EditMode::Roll => ffi::GES_EDIT_MODE_ROLL,
            EditMode::Trim => ffi::GES_EDIT_MODE_TRIM,
            EditMode::Slide => ffi::GES_EDIT_MODE_SLIDE,
            EditMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESEditMode> for EditMode {
    unsafe fn from_glib(value: ffi::GESEditMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => EditMode::Normal,
            1 => EditMode::Ripple,
            2 => EditMode::Roll,
            3 => EditMode::Trim,
            4 => EditMode::Slide,
            value => EditMode::__Unknown(value),
        }
    }
}

impl StaticType for EditMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_edit_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EditMode {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EditMode {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for EditMode {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
