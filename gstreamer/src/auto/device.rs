// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Caps, Element, Object, Structure};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstDevice")]
    pub struct Device(Object<ffi::GstDevice, ffi::GstDeviceClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_device_get_type(),
    }
}

impl Device {
    pub const NONE: Option<&'static Device> = None;
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Device>> Sealed for T {}
}

pub trait DeviceExt: IsA<Device> + sealed::Sealed + 'static {
    #[doc(alias = "gst_device_create_element")]
    fn create_element(&self, name: Option<&str>) -> Result<Element, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_device_create_element(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create element for device"))
        }
    }

    #[doc(alias = "gst_device_get_caps")]
    #[doc(alias = "get_caps")]
    fn caps(&self) -> Option<Caps> {
        unsafe { from_glib_full(ffi::gst_device_get_caps(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_device_get_device_class")]
    #[doc(alias = "get_device_class")]
    fn device_class(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_device_get_device_class(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_get_display_name")]
    #[doc(alias = "get_display_name")]
    fn display_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_device_get_display_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_get_properties")]
    #[doc(alias = "get_properties")]
    fn properties(&self) -> Option<Structure> {
        unsafe {
            from_glib_full(ffi::gst_device_get_properties(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_has_classes")]
    fn has_classes(&self, classes: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_device_has_classes(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_has_classesv")]
    fn has_classesv(&self, classes: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gst_device_has_classesv(
                self.as_ref().to_glib_none().0,
                classes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_device_reconfigure_element")]
    fn reconfigure_element(
        &self,
        element: &impl IsA<Element>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_device_reconfigure_element(
                    self.as_ref().to_glib_none().0,
                    element.as_ref().to_glib_none().0
                ),
                "Failed to reconfigure the element to use this device"
            )
        }
    }

    #[doc(alias = "removed")]
    fn connect_removed<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn removed_trampoline<
            P: IsA<Device>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Device::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Device>> DeviceExt for O {}
