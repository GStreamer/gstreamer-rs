// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Object, PluginFeature, URIType};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GstElementFactory")]
    pub struct ElementFactory(Object<ffi::GstElementFactory, ffi::GstElementFactoryClass>) @extends PluginFeature, Object;

    match fn {
        type_ => || ffi::gst_element_factory_get_type(),
    }
}

impl ElementFactory {
    #[doc(alias = "gst_element_factory_get_element_type")]
    #[doc(alias = "get_element_type")]
    pub fn element_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_element_factory_get_element_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_metadata_keys")]
    #[doc(alias = "get_metadata_keys")]
    pub fn metadata_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_get_metadata_keys(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_num_pad_templates")]
    #[doc(alias = "get_num_pad_templates")]
    pub fn num_pad_templates(&self) -> u32 {
        unsafe { ffi::gst_element_factory_get_num_pad_templates(self.to_glib_none().0) }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_element_factory_get_skip_documentation")]
    #[doc(alias = "get_skip_documentation")]
    pub fn skips_documentation(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_get_skip_documentation(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_uri_protocols")]
    #[doc(alias = "get_uri_protocols")]
    pub fn uri_protocols(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_element_factory_get_uri_protocols(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_uri_type")]
    #[doc(alias = "get_uri_type")]
    pub fn uri_type(&self) -> URIType {
        unsafe { from_glib(ffi::gst_element_factory_get_uri_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gst_element_factory_has_interface")]
    pub fn has_interface(&self, interfacename: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_has_interface(
                self.to_glib_none().0,
                interfacename.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_element_factory_find")]
    pub fn find(name: &str) -> Option<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_element_factory_find(name.to_glib_none().0)) }
    }
}

unsafe impl Send for ElementFactory {}
unsafe impl Sync for ElementFactory {}
