// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Object, Plugin, PluginFeature};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRegistry")]
    pub struct Registry(Object<ffi::GstRegistry, ffi::GstRegistryClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_registry_get_type(),
    }
}

impl Registry {
    #[doc(alias = "gst_registry_add_feature")]
    pub fn add_feature(
        &self,
        feature: &impl IsA<PluginFeature>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_registry_add_feature(
                    self.to_glib_none().0,
                    feature.as_ref().to_glib_none().0
                ),
                "Failed to add feature"
            )
        }
    }

    #[doc(alias = "gst_registry_add_plugin")]
    pub fn add_plugin(&self, plugin: &Plugin) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_registry_add_plugin(self.to_glib_none().0, plugin.to_glib_none().0),
                "Failed to add plugin"
            )
        }
    }

    #[doc(alias = "gst_registry_check_feature_version")]
    pub fn check_feature_version(
        &self,
        feature_name: &str,
        min_major: u32,
        min_minor: u32,
        min_micro: u32,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_registry_check_feature_version(
                self.to_glib_none().0,
                feature_name.to_glib_none().0,
                min_major,
                min_minor,
                min_micro,
            ))
        }
    }

    #[doc(alias = "gst_registry_find_feature")]
    pub fn find_feature(&self, name: &str, type_: glib::types::Type) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(ffi::gst_registry_find_feature(
                self.to_glib_none().0,
                name.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_registry_find_plugin")]
    pub fn find_plugin(&self, name: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_registry_find_plugin(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_get_feature_list_cookie")]
    #[doc(alias = "get_feature_list_cookie")]
    pub fn feature_list_cookie(&self) -> u32 {
        unsafe { ffi::gst_registry_get_feature_list_cookie(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_registry_lookup")]
    pub fn lookup(&self, filename: &str) -> Option<Plugin> {
        unsafe {
            from_glib_full(ffi::gst_registry_lookup(
                self.to_glib_none().0,
                filename.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_lookup_feature")]
    pub fn lookup_feature(&self, name: &str) -> Option<PluginFeature> {
        unsafe {
            from_glib_full(ffi::gst_registry_lookup_feature(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_remove_feature")]
    pub fn remove_feature(&self, feature: &impl IsA<PluginFeature>) {
        unsafe {
            ffi::gst_registry_remove_feature(
                self.to_glib_none().0,
                feature.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_registry_remove_plugin")]
    pub fn remove_plugin(&self, plugin: &Plugin) {
        unsafe {
            ffi::gst_registry_remove_plugin(self.to_glib_none().0, plugin.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_registry_scan_path")]
    pub fn scan_path(&self, path: impl AsRef<std::path::Path>) -> bool {
        unsafe {
            from_glib(ffi::gst_registry_scan_path(
                self.to_glib_none().0,
                path.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_get")]
    pub fn get() -> Registry {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_registry_get()) }
    }

    #[doc(alias = "feature-added")]
    pub fn connect_feature_added<F: Fn(&Self, &PluginFeature) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn feature_added_trampoline<
            F: Fn(&Registry, &PluginFeature) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRegistry,
            feature: *mut ffi::GstPluginFeature,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(feature))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"feature-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    feature_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "plugin-added")]
    pub fn connect_plugin_added<F: Fn(&Self, &Plugin) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn plugin_added_trampoline<
            F: Fn(&Registry, &Plugin) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRegistry,
            plugin: *mut ffi::GstPlugin,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(plugin))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"plugin-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    plugin_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for Registry {}
unsafe impl Sync for Registry {}
