// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::DiscovererInfo;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Discoverer(Object<ffi::GstDiscoverer, ffi::GstDiscovererClass>);

    match fn {
        get_type => || ffi::gst_discoverer_get_type(),
    }
}

impl Discoverer {
    #[doc(alias = "gst_discoverer_new")]
    pub fn new(timeout: gst::ClockTime) -> Result<Discoverer, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_discoverer_new(timeout.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_discoverer_discover_uri")]
    pub fn discover_uri(&self, uri: &str) -> Result<DiscovererInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gst_discoverer_discover_uri(
                self.to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gst_discoverer_discover_uri_async")]
    pub fn discover_uri_async(&self, uri: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_discoverer_discover_uri_async(self.to_glib_none().0, uri.to_glib_none().0),
                "Failed to add URI to list of discovers"
            )
        }
    }

    #[doc(alias = "gst_discoverer_start")]
    pub fn start(&self) {
        unsafe {
            ffi::gst_discoverer_start(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_discoverer_stop")]
    pub fn stop(&self) {
        unsafe {
            ffi::gst_discoverer_stop(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn get_property_use_cache(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"use-cache\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-cache` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn set_property_use_cache(&self, use_cache: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"use-cache\0".as_ptr() as *const _,
                glib::Value::from(&use_cache).to_glib_none().0,
            );
        }
    }

    pub fn connect_discovered<
        F: Fn(&Discoverer, &DiscovererInfo, Option<&glib::Error>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn discovered_trampoline<
            F: Fn(&Discoverer, &DiscovererInfo, Option<&glib::Error>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDiscoverer,
            info: *mut ffi::GstDiscovererInfo,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(info),
                Option::<glib::Error>::from_glib_borrow(error)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"discovered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    discovered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_finished<F: Fn(&Discoverer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn finished_trampoline<F: Fn(&Discoverer) + Send + Sync + 'static>(
            this: *mut ffi::GstDiscoverer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    finished_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_source_setup<F: Fn(&Discoverer, &gst::Element) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn source_setup_trampoline<
            F: Fn(&Discoverer, &gst::Element) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDiscoverer,
            source: *mut gst::ffi::GstElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(source))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"source-setup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    source_setup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_starting<F: Fn(&Discoverer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn starting_trampoline<F: Fn(&Discoverer) + Send + Sync + 'static>(
            this: *mut ffi::GstDiscoverer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"starting\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    starting_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn connect_property_use_cache_notify<F: Fn(&Discoverer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_cache_trampoline<
            F: Fn(&Discoverer) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstDiscoverer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-cache\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_cache_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for Discoverer {}
unsafe impl Sync for Discoverer {}
