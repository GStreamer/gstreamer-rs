// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, DiscovererInfo};
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstEncodingProfile")]
    pub struct EncodingProfile(Object<ffi::GstEncodingProfile, ffi::GstEncodingProfileClass>);

    match fn {
        type_ => || ffi::gst_encoding_profile_get_type(),
    }
}

impl EncodingProfile {
    pub const NONE: Option<&'static EncodingProfile> = None;

    #[doc(alias = "gst_encoding_profile_find")]
    pub fn find(
        targetname: &str,
        profilename: Option<&str>,
        category: Option<&str>,
    ) -> Option<EncodingProfile> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_find(
                targetname.to_glib_none().0,
                profilename.to_glib_none().0,
                category.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_from_discoverer")]
    pub fn from_discoverer(info: &DiscovererInfo) -> Result<EncodingProfile, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_encoding_profile_from_discoverer(
                info.to_glib_none().0,
            ))
            .ok_or_else(|| {
                glib::bool_error!("Failed to create EncodingProfile from DiscovererInfo")
            })
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_encoding_profile_from_string")]
    pub fn from_string(string: &str) -> Result<EncodingProfile, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_encoding_profile_from_string(
                string.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create EncodingProfile from string"))
        }
    }
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
impl std::fmt::Display for EncodingProfile {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&EncodingProfileExt::to_str(self))
    }
}

unsafe impl Send for EncodingProfile {}
unsafe impl Sync for EncodingProfile {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::EncodingProfile>> Sealed for T {}
}

pub trait EncodingProfileExt: IsA<EncodingProfile> + sealed::Sealed + 'static {
    #[doc(alias = "gst_encoding_profile_copy")]
    #[must_use]
    fn copy(&self) -> EncodingProfile {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_copy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_allow_dynamic_output")]
    #[doc(alias = "get_allow_dynamic_output")]
    fn allows_dynamic_output(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_get_allow_dynamic_output(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_file_extension")]
    #[doc(alias = "get_file_extension")]
    fn file_extension(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_file_extension(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_format")]
    #[doc(alias = "get_format")]
    fn format(&self) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_input_caps")]
    #[doc(alias = "get_input_caps")]
    fn input_caps(&self) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_get_input_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_presence")]
    #[doc(alias = "get_presence")]
    fn presence(&self) -> u32 {
        unsafe { ffi::gst_encoding_profile_get_presence(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_encoding_profile_get_preset")]
    #[doc(alias = "get_preset")]
    fn preset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_preset_name")]
    #[doc(alias = "get_preset_name")]
    fn preset_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_preset_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_encoding_profile_get_single_segment")]
    #[doc(alias = "get_single_segment")]
    fn is_single_segment(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_get_single_segment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_get_type_nick")]
    #[doc(alias = "get_type_nick")]
    fn type_nick(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gst_encoding_profile_get_type_nick(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_is_enabled")]
    fn is_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_is_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_encoding_profile_is_equal")]
    fn is_equal(&self, b: &impl IsA<EncodingProfile>) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_profile_is_equal(
                self.as_ref().to_glib_none().0,
                b.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_encoding_profile_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_encoding_profile_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "element-properties")]
    fn connect_element_properties_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_element_properties_trampoline<
            P: IsA<EncodingProfile>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstEncodingProfile,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(EncodingProfile::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::element-properties\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_element_properties_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<EncodingProfile>> EncodingProfileExt for O {}
