// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::{ffi, Bin, Element, LoggableError, Message};

pub trait BinImpl: BinImplExt + ElementImpl {
    fn add_element(&self, element: &Element) -> Result<(), LoggableError> {
        self.parent_add_element(element)
    }

    fn remove_element(&self, element: &Element) -> Result<(), LoggableError> {
        self.parent_remove_element(element)
    }

    fn do_latency(&self) -> Result<(), LoggableError> {
        self.parent_do_latency()
    }

    fn handle_message(&self, message: Message) {
        self.parent_handle_message(message)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::BinImplExt> Sealed for T {}
}

pub trait BinImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_add_element(&self, element: &Element) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBinClass;
            let f = (*parent_class).add_element.ok_or_else(|| {
                loggable_error!(
                    crate::CAT_RUST,
                    "Parent function `add_element` is not defined"
                )
            })?;
            result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<crate::Bin>().to_glib_none().0,
                    element.to_glib_none().0
                ),
                crate::CAT_RUST,
                "Failed to add the element using the parent function"
            )
        }
    }

    fn parent_remove_element(&self, element: &Element) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBinClass;
            let f = (*parent_class).remove_element.ok_or_else(|| {
                loggable_error!(
                    crate::CAT_RUST,
                    "Parent function `remove_element` is not defined"
                )
            })?;
            result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<crate::Bin>().to_glib_none().0,
                    element.to_glib_none().0
                ),
                crate::CAT_RUST,
                "Failed to remove the element using the parent function"
            )
        }
    }

    fn parent_do_latency(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBinClass;
            let f = (*parent_class).do_latency.ok_or_else(|| {
                loggable_error!(
                    crate::CAT_RUST,
                    "Parent function `do_latency` is not defined"
                )
            })?;
            result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<crate::Bin>().to_glib_none().0,),
                crate::CAT_RUST,
                "Failed to update latency using the parent function"
            )
        }
    }

    fn parent_handle_message(&self, message: Message) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBinClass;
            if let Some(ref f) = (*parent_class).handle_message {
                f(
                    self.obj().unsafe_cast_ref::<crate::Bin>().to_glib_none().0,
                    message.into_glib_ptr(),
                );
            }
        }
    }
}

impl<T: BinImpl> BinImplExt for T {}

unsafe impl<T: BinImpl> IsSubclassable<T> for Bin {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.add_element = Some(bin_add_element::<T>);
        klass.remove_element = Some(bin_remove_element::<T>);
        klass.do_latency = Some(bin_do_latency::<T>);
        klass.handle_message = Some(bin_handle_message::<T>);
    }
}

unsafe extern "C" fn bin_add_element<T: BinImpl>(
    ptr: *mut ffi::GstBin,
    element: *mut ffi::GstElement,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, false, {
        match imp.add_element(&from_glib_none(element)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn bin_remove_element<T: BinImpl>(
    ptr: *mut ffi::GstBin,
    element: *mut ffi::GstElement,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    // If we get a floating reference passed simply return FALSE here. It can't be
    // stored inside this bin, and if we continued to use it we would take ownership
    // of this floating reference.
    if glib::gobject_ffi::g_object_is_floating(element as *mut glib::gobject_ffi::GObject)
        != glib::ffi::GFALSE
    {
        return glib::ffi::GFALSE;
    }

    panic_to_error!(imp, false, {
        match imp.remove_element(&from_glib_none(element)) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn bin_do_latency<T: BinImpl>(ptr: *mut ffi::GstBin) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, false, {
        match imp.do_latency() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn bin_handle_message<T: BinImpl>(
    ptr: *mut ffi::GstBin,
    message: *mut ffi::GstMessage,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    panic_to_error!(imp, (), { imp.handle_message(from_glib_full(message)) });
}
