// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute, path};

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString,
};

use crate::{ffi, prelude::*, Bin, BinFlags, Element, LoggableError};

impl Bin {
    // rustdoc-stripper-ignore-next
    /// Creates a new [`Bin`] object with a default name.
    ///
    /// Use [`Bin::with_name()`] to create a [`Bin`] with a specific name.
    /// Use [`Bin::builder()`] for additional configuration.
    #[doc(alias = "gst_bin_new")]
    pub fn new() -> Bin {
        assert_initialized_main_thread!();
        unsafe { Element::from_glib_none(ffi::gst_bin_new(std::ptr::null())).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Bin`] object with the specified name.
    ///
    /// Use [`Bin::builder()`] for additional configuration.
    #[doc(alias = "gst_bin_new")]
    pub fn with_name(name: &str) -> Bin {
        assert_initialized_main_thread!();
        unsafe { Element::from_glib_none(ffi::gst_bin_new(name.to_glib_none().0)).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Bin`] objects.
    ///
    /// This method returns an instance of [`BinBuilder`] which can be used to create [`Bin`] objects.
    pub fn builder() -> BinBuilder {
        BinBuilder::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Bin>> Sealed for T {}
}

pub trait GstBinExtManual: sealed::Sealed + IsA<Bin> + 'static {
    #[doc(alias = "gst_bin_add_many")]
    fn add_many(
        &self,
        elements: impl IntoIterator<Item = impl AsRef<Element>>,
    ) -> Result<(), glib::BoolError> {
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

    #[doc(alias = "gst_bin_remove_many")]
    fn remove_many(
        &self,
        elements: impl IntoIterator<Item = impl AsRef<Element>>,
    ) -> Result<(), glib::BoolError> {
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

    #[doc(alias = "do-latency")]
    fn connect_do_latency<F: Fn(&Self) -> Result<(), LoggableError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"do-latency\0".as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    do_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_bin_iterate_all_by_element_factory_name")]
    fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_element_factory_name(
                self.as_ref().to_glib_none().0,
                factory_name.to_glib_none().0,
            ))
        }
    }
    #[doc(alias = "gst_bin_iterate_all_by_interface")]
    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_interface(
                self.as_ref().to_glib_none().0,
                iface.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_bin_iterate_elements")]
    fn iterate_elements(&self) -> crate::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_bin_iterate_recurse")]
    fn iterate_recurse(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_recurse(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_bin_iterate_sinks")]
    fn iterate_sinks(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sinks(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_bin_iterate_sorted")]
    fn iterate_sorted(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sorted(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_bin_iterate_sources")]
    fn iterate_sources(&self) -> crate::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sources(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "get_children")]
    fn children(&self) -> Vec<Element> {
        unsafe {
            let bin: &ffi::GstBin = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            FromGlibPtrContainer::from_glib_none(bin.children)
        }
    }

    #[doc(alias = "gst_debug_bin_to_dot_data")]
    fn debug_to_dot_data(&self, details: crate::DebugGraphDetails) -> GString {
        crate::auto::functions::debug_bin_to_dot_data(self, details)
    }

    #[doc(alias = "GST_DEBUG_BIN_TO_DOT_FILE")]
    #[doc(alias = "gst_debug_bin_to_dot_file")]
    fn debug_to_dot_file(
        &self,
        details: crate::DebugGraphDetails,
        file_name: impl AsRef<path::Path>,
    ) {
        crate::auto::functions::debug_bin_to_dot_file(self, details, file_name)
    }

    #[doc(alias = "GST_DEBUG_BIN_TO_DOT_FILE_WITH_TS")]
    #[doc(alias = "gst_debug_bin_to_dot_file_with_ts")]
    fn debug_to_dot_file_with_ts(
        &self,
        details: crate::DebugGraphDetails,
        file_name: impl AsRef<path::Path>,
    ) {
        crate::auto::functions::debug_bin_to_dot_file_with_ts(self, details, file_name)
    }

    fn set_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags &= !flags.into_glib();
        }
    }

    #[doc(alias = "get_bin_flags")]
    fn bin_flags(&self) -> BinFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            from_glib((*ptr).flags)
        }
    }
}

impl<O: IsA<Bin>> GstBinExtManual for O {}

impl Default for Bin {
    fn default() -> Self {
        glib::object::Object::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Bin`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BinBuilder {
    builder: glib::object::ObjectBuilder<'static, Bin>,
}

impl BinBuilder {
    fn new() -> Self {
        Self {
            builder: glib::Object::builder(),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Bin`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Bin {
        self.builder.build()
    }

    pub fn async_handling(self, async_handling: bool) -> Self {
        Self {
            builder: self.builder.property("async-handling", async_handling),
        }
    }

    pub fn async_handling_if_some(self, async_handling: Option<bool>) -> Self {
        if let Some(async_handling) = async_handling {
            self.async_handling(async_handling)
        } else {
            self
        }
    }

    pub fn message_forward(self, message_forward: bool) -> Self {
        Self {
            builder: self.builder.property("message-forward", message_forward),
        }
    }

    pub fn message_forward_if_some(self, message_forward: Option<bool>) -> Self {
        if let Some(message_forward) = message_forward {
            self.message_forward(message_forward)
        } else {
            self
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn name_if(self, name: impl Into<glib::GString>, predicate: bool) -> Self {
        if predicate {
            self.name(name)
        } else {
            self
        }
    }

    pub fn name_if_some(self, name: Option<impl Into<glib::GString>>) -> Self {
        if let Some(name) = name {
            self.name(name)
        } else {
            self
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

    #[test]
    fn test_get_children() {
        crate::init().unwrap();

        let bin = crate::Bin::new();
        bin.add(
            &crate::ElementFactory::make("identity")
                .name("identity0")
                .build()
                .unwrap(),
        )
        .unwrap();
        bin.add(
            &crate::ElementFactory::make("identity")
                .name("identity1")
                .build()
                .unwrap(),
        )
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
