// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Bin;
use crate::BinFlags;
use crate::Element;
use crate::LoggableError;

use glib::prelude::*;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;

use std::boxed::Box as Box_;
use std::mem::transmute;
use std::path;

pub trait GstBinExtManual: 'static {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;

    fn connect_do_latency<F: Fn(&Self) -> Result<(), LoggableError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_bin_iterate_all_by_element_factory_name")]
    fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_all_by_interface")]
    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_elements")]
    fn iterate_elements(&self) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_recurse")]
    fn iterate_recurse(&self) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_sinks")]
    fn iterate_sinks(&self) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_sorted")]
    fn iterate_sorted(&self) -> crate::Iterator<Element>;
    #[doc(alias = "gst_bin_iterate_sources")]
    fn iterate_sources(&self) -> crate::Iterator<Element>;
    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<Element>;

    fn debug_to_dot_data(&self, details: crate::DebugGraphDetails) -> GString;
    fn debug_to_dot_file<Q: AsRef<path::Path>>(
        &self,
        details: crate::DebugGraphDetails,
        file_name: Q,
    );
    fn debug_to_dot_file_with_ts<Q: AsRef<path::Path>>(
        &self,
        details: crate::DebugGraphDetails,
        file_name: Q,
    );

    fn set_bin_flags(&self, flags: BinFlags);

    fn unset_bin_flags(&self, flags: BinFlags);

    #[doc(alias = "get_bin_flags")]
    fn bin_flags(&self) -> BinFlags;
}

impl<O: IsA<Bin>> GstBinExtManual for O {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                glib::result_from_gboolean!(
                    ffi::gst_bin_add(self.as_ref().to_glib_none().0, e.as_ref().to_glib_none().0),
                    "Failed to add elements"
                )?;
            }
        }

        Ok(())
    }

    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                glib::result_from_gboolean!(
                    ffi::gst_bin_remove(
                        self.as_ref().to_glib_none().0,
                        e.as_ref().to_glib_none().0,
                    ),
                    "Failed to remove elements"
                )?;
            }
        }

        Ok(())
    }

    fn connect_do_latency<F: Fn(&Self) -> Result<(), LoggableError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"do-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    do_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_element_factory_name(
                self.as_ref().to_glib_none().0,
                factory_name.to_glib_none().0,
            ))
        }
    }

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_interface(
                self.as_ref().to_glib_none().0,
                iface.into_glib(),
            ))
        }
    }

    fn iterate_elements(&self) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_recurse(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_recurse(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sinks(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sinks(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sorted(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sorted(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sources(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sources(self.as_ref().to_glib_none().0)) }
    }

    fn children(&self) -> Vec<Element> {
        unsafe {
            let bin: &ffi::GstBin = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&bin.element.object.lock);
            FromGlibPtrContainer::from_glib_none(bin.children)
        }
    }

    fn debug_to_dot_data(&self, details: crate::DebugGraphDetails) -> GString {
        crate::debug_bin_to_dot_data(self, details)
    }

    fn debug_to_dot_file<Q: AsRef<path::Path>>(
        &self,
        details: crate::DebugGraphDetails,
        file_name: Q,
    ) {
        crate::debug_bin_to_dot_file(self, details, file_name)
    }

    fn debug_to_dot_file_with_ts<Q: AsRef<path::Path>>(
        &self,
        details: crate::DebugGraphDetails,
        file_name: Q,
    ) {
        crate::debug_bin_to_dot_file_with_ts(self, details, file_name)
    }

    fn set_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.into_glib();
        }
    }

    fn bin_flags(&self) -> BinFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

unsafe extern "C" fn do_latency_trampoline<
    P,
    F: Fn(&P) -> Result<(), LoggableError> + Send + Sync + 'static,
>(
    this: *mut ffi::GstBin,
    f: glib::ffi::gpointer,
) -> glib::ffi::gboolean
where
    P: IsA<Bin>,
{
    let f: &F = &*(f as *const F);
    match f(Bin::from_glib_borrow(this).unsafe_cast_ref()) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*Bin::from_glib_borrow(this));
            false
        }
    }
    .into_glib()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_get_children() {
        crate::init().unwrap();

        let bin = crate::Bin::new(None);
        bin.add(&crate::ElementFactory::make("identity", Some("identity0")).unwrap())
            .unwrap();
        bin.add(&crate::ElementFactory::make("identity", Some("identity1")).unwrap())
            .unwrap();

        let mut child_names = bin
            .children()
            .iter()
            .map(|c| c.name())
            .collect::<Vec<GString>>();
        child_names.sort();
        assert_eq!(
            child_names,
            vec![String::from("identity0"), String::from("identity1")]
        );
    }
}
