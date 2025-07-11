// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::{prelude::*, translate::*};

use crate::{
    ffi, CapsRef, Element, ElementFactory, Rank, StaticPadTemplate, ELEMENT_METADATA_AUTHOR,
    ELEMENT_METADATA_DESCRIPTION, ELEMENT_METADATA_DOC_URI, ELEMENT_METADATA_ICON_NAME,
    ELEMENT_METADATA_KLASS, ELEMENT_METADATA_LONGNAME,
};

impl ElementFactory {
    #[doc(alias = "gst_element_factory_create")]
    #[doc(alias = "gst_element_factory_create_with_properties")]
    #[track_caller]
    pub fn create(&self) -> ElementBuilder<'_> {
        assert_initialized_main_thread!();
        ElementBuilder {
            name_or_factory: NameOrFactory::Factory(self),
            builder: crate::Object::builder_for_deferred_type(),
        }
    }

    #[doc(alias = "gst_element_factory_make")]
    #[doc(alias = "gst_element_factory_make_with_properties")]
    #[track_caller]
    pub fn make(factoryname: &str) -> ElementBuilder<'_> {
        assert_initialized_main_thread!();
        ElementBuilder {
            name_or_factory: NameOrFactory::Name(factoryname),
            builder: crate::Object::builder_for_deferred_type(),
        }
    }

    #[doc(alias = "gst_element_factory_create")]
    #[track_caller]
    pub fn create_with_name(&self, name: Option<&str>) -> Result<Element, glib::BoolError> {
        let mut builder = self.create();
        if let Some(name) = name {
            builder = builder.name(name);
        }
        builder.build()
    }

    #[doc(alias = "gst_element_factory_make")]
    #[track_caller]
    pub fn make_with_name(
        factoryname: &str,
        name: Option<&str>,
    ) -> Result<Element, glib::BoolError> {
        skip_assert_initialized!();
        let mut builder = Self::make(factoryname);
        if let Some(name) = name {
            builder = builder.name(name);
        }
        builder.build()
    }

    #[doc(alias = "gst_element_factory_get_static_pad_templates")]
    #[doc(alias = "get_static_pad_templates")]
    pub fn static_pad_templates(&self) -> glib::List<StaticPadTemplate> {
        unsafe {
            glib::List::from_glib_none(ffi::gst_element_factory_get_static_pad_templates(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_list_is_type")]
    pub fn has_type(&self, type_: crate::ElementFactoryType) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_list_is_type(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_list_get_elements")]
    pub fn factories_with_type(
        type_: crate::ElementFactoryType,
        minrank: Rank,
    ) -> glib::List<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_list_get_elements(
                type_.into_glib(),
                minrank.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr =
                ffi::gst_element_factory_get_metadata(self.to_glib_none().0, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[doc(alias = "get_longname")]
    #[doc(alias = "gst_element_factory_get_longname")]
    pub fn longname(&self) -> &str {
        self.metadata(ELEMENT_METADATA_LONGNAME).unwrap()
    }

    #[doc(alias = "get_klass")]
    #[doc(alias = "gst_element_factory_get_klass")]
    pub fn klass(&self) -> &str {
        self.metadata(ELEMENT_METADATA_KLASS).unwrap()
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_element_factory_get_description")]
    pub fn description(&self) -> &str {
        self.metadata(ELEMENT_METADATA_DESCRIPTION).unwrap()
    }

    #[doc(alias = "get_author")]
    #[doc(alias = "gst_element_factory_get_author")]
    pub fn author(&self) -> &str {
        self.metadata(ELEMENT_METADATA_AUTHOR).unwrap()
    }

    #[doc(alias = "get_documentation_uri")]
    #[doc(alias = "gst_element_factory_get_documentation_uri")]
    pub fn documentation_uri(&self) -> Option<&str> {
        self.metadata(ELEMENT_METADATA_DOC_URI)
    }

    #[doc(alias = "get_icon_name")]
    #[doc(alias = "gst_element_factory_get_icon_name")]
    pub fn icon_name(&self) -> Option<&str> {
        self.metadata(ELEMENT_METADATA_ICON_NAME)
    }

    #[doc(alias = "gst_element_factory_can_sink_all_caps")]
    pub fn can_sink_all_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_all_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_sink_any_caps")]
    pub fn can_sink_any_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_any_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_all_caps")]
    pub fn can_src_all_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_all_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_any_caps")]
    pub fn can_src_any_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_any_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Builder for `Element`s.
#[must_use = "The builder must be built to be used"]
pub struct ElementBuilder<'a> {
    name_or_factory: NameOrFactory<'a>,
    builder: crate::gobject::GObjectBuilder<'a, Element>,
}

#[derive(Copy, Clone)]
enum NameOrFactory<'a> {
    Name(&'a str),
    Factory(&'a ElementFactory),
}

impl<'a> ElementBuilder<'a> {
    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn property(self, name: &'a str, value: impl Into<glib::Value> + 'a) -> Self {
        Self {
            builder: self.builder.property(name, value),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given string value `value`.
    #[inline]
    pub fn property_from_str(self, name: &'a str, value: &'a str) -> Self {
        Self {
            builder: self.builder.property_from_str(name, value),
            ..self
        }
    }

    impl_builder_gvalue_extra_setters!(property_and_name);

    // rustdoc-stripper-ignore-next
    /// Builds the [`Element`] with the provided properties.
    ///
    /// This fails if there is no such [`ElementFactory`] or the [`ElementFactory`] can't be loaded.
    ///
    /// # Panics
    ///
    /// This panics if the [`Element`] is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    ///
    /// [`Element`]: crate::Element
    #[track_caller]
    #[must_use = "Building the element without using it has no effect"]
    pub fn build(self) -> Result<Element, glib::BoolError> {
        let mut _factory_found = None;
        let factory = match self.name_or_factory {
            NameOrFactory::Name(name) => {
                let factory = ElementFactory::find(name).ok_or_else(|| {
                    crate::warning!(crate::CAT_RUST, "element factory '{}' not found", name);
                    glib::bool_error!(
                        "Failed to find element factory with name '{}' for creating element",
                        name
                    )
                })?;
                _factory_found = Some(factory);
                _factory_found.as_ref().unwrap()
            }
            NameOrFactory::Factory(factory) => factory,
        };

        // The below is basically a reimplementation of the C function. We want to call
        // glib::Object::with_type() ourselves here for checking properties and their values
        // correctly and to provide consistent behaviour.
        use crate::prelude::{
            ElementExtManual, GstObjectExt, GstObjectExtManual, PluginFeatureExtManual,
        };

        let factory = factory.load().map_err(|_| {
            crate::warning!(
                crate::CAT_RUST,
                obj = factory,
                "loading element factory '{}' failed",
                factory.name(),
            );
            glib::bool_error!(
                "Failed to load element factory '{}' for creating element",
                factory.name()
            )
        })?;

        let element_type = factory.element_type();
        if !element_type.is_valid() {
            crate::warning!(
                crate::CAT_RUST,
                obj = &factory,
                "element factory '{}' has no type",
                factory.name()
            );
            return Err(glib::bool_error!(
                "Failed to create element from factory '{}'",
                factory.name()
            ));
        }

        let element = self
            .builder
            .type_(element_type)
            .build()
            .map_err(|err| {
                use crate::gobject::GObjectError::*;
                match err {
                    PropertyNotFound { property, .. } => {
                        format!("property '{property}' of element factory '{}' not found", factory.name())
                    },
                    PropertyFromStr { property, value, .. } => {
                        format!("property '{property}' of element factory '{}' can't be set from string '{value}'", factory.name())
                    },
                }
            }).unwrap();

        unsafe {
            use std::sync::atomic;

            let klass = element.element_class();
            let factory_ptr: &atomic::AtomicPtr<ffi::GstElementFactory> =
                &*(&klass.as_ref().elementfactory as *const *mut ffi::GstElementFactory
                    as *const atomic::AtomicPtr<ffi::GstElementFactory>);
            if factory_ptr
                .compare_exchange(
                    std::ptr::null_mut(),
                    factory.as_ptr(),
                    atomic::Ordering::SeqCst,
                    atomic::Ordering::SeqCst,
                )
                .is_ok()
            {
                factory.set_object_flags(crate::ObjectFlags::MAY_BE_LEAKED);
            }

            if glib::gobject_ffi::g_object_is_floating(factory.as_ptr() as *mut _)
                != glib::ffi::GFALSE
            {
                glib::g_critical!(
                    "GStreamer",
                    "The created element should be floating, this is probably caused by faulty bindings",
                );
            }
        }

        crate::log!(
            crate::CAT_RUST,
            obj = &factory,
            "created element \"{}\"",
            factory.name()
        );

        Ok(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn builder() {
        crate::init().unwrap();

        let fakesink = ElementFactory::make("fakesink")
            .name("test-fakesink")
            .property("can-activate-pull", true)
            .property_from_str("state-error", "ready-to-paused")
            .build()
            .unwrap();

        assert_eq!(fakesink.name(), "test-fakesink");
        assert!(fakesink.property::<bool>("can-activate-pull"));
        let v = fakesink.property_value("state-error");
        let (_klass, e) = glib::EnumValue::from_value(&v).unwrap();
        assert_eq!(e.nick(), "ready-to-paused");
    }
}
