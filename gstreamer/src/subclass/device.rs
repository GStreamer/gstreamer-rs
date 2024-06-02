// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::{ffi, Device, Element, LoggableError};

pub trait DeviceImpl: DeviceImplExt + GstObjectImpl + Send + Sync {
    fn create_element(&self, name: Option<&str>) -> Result<Element, LoggableError> {
        self.parent_create_element(name)
    }

    fn reconfigure_element(&self, element: &Element) -> Result<(), LoggableError> {
        self.parent_reconfigure_element(element)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::DeviceImplExt> Sealed for T {}
}

pub trait DeviceImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_create_element(&self, name: Option<&str>) -> Result<Element, LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstDeviceClass;
            if let Some(f) = (*parent_class).create_element {
                let ptr = f(
                    self.obj().unsafe_cast_ref::<Device>().to_glib_none().0,
                    name.to_glib_none().0,
                );

                // Don't steal floating reference here but pass it further to the caller
                Option::<_>::from_glib_full(ptr).ok_or_else(|| {
                    loggable_error!(
                        crate::CAT_RUST,
                        "Failed to create element using the parent function"
                    )
                })
            } else {
                Err(loggable_error!(
                    crate::CAT_RUST,
                    "Parent function `create_element` is not defined"
                ))
            }
        }
    }

    fn parent_reconfigure_element(&self, element: &Element) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstDeviceClass;
            let f = (*parent_class).reconfigure_element.ok_or_else(|| {
                loggable_error!(
                    crate::CAT_RUST,
                    "Parent function `reconfigure_element` is not defined"
                )
            })?;
            result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<Device>().to_glib_none().0,
                    element.to_glib_none().0
                ),
                crate::CAT_RUST,
                "Failed to reconfigure the element using the parent function"
            )
        }
    }
}

impl<T: DeviceImpl> DeviceImplExt for T {}

unsafe impl<T: DeviceImpl> IsSubclassable<T> for Device {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.create_element = Some(device_create_element::<T>);
        klass.reconfigure_element = Some(device_reconfigure_element::<T>);
    }
}

unsafe extern "C" fn device_create_element<T: DeviceImpl>(
    ptr: *mut ffi::GstDevice,
    name: *const libc::c_char,
) -> *mut ffi::GstElement {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.create_element(
        Option::<glib::GString>::from_glib_borrow(name)
            .as_ref()
            .as_ref()
            .map(|s| s.as_str()),
    ) {
        Ok(element) => {
            // The reference we're going to return, the initial reference is going to
            // be dropped here now
            let element = element.into_glib_ptr();
            // See https://gitlab.freedesktop.org/gstreamer/gstreamer/issues/444
            glib::gobject_ffi::g_object_force_floating(element as *mut glib::gobject_ffi::GObject);
            element
        }
        Err(err) => {
            err.log_with_imp(imp);
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn device_reconfigure_element<T: DeviceImpl>(
    ptr: *mut ffi::GstDevice,
    element: *mut ffi::GstElement,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    match imp.reconfigure_element(&from_glib_borrow(element)) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_imp(imp);
            false
        }
    }
    .into_glib()
}
