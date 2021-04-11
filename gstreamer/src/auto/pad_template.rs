// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Caps;
use crate::Object;
use crate::Pad;
use crate::PadDirection;
use crate::PadPresence;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct PadTemplate(Object<ffi::GstPadTemplate, ffi::GstPadTemplateClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_pad_template_get_type(),
    }
}

impl PadTemplate {
    #[doc(alias = "gst_pad_template_new")]
    pub fn new(
        name_template: &str,
        direction: PadDirection,
        presence: PadPresence,
        caps: &Caps,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_pad_template_new(
                name_template.to_glib_none().0,
                direction.to_glib(),
                presence.to_glib(),
                caps.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create pad template"))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_pad_template_new_with_gtype")]
    pub fn with_gtype(
        name_template: &str,
        direction: PadDirection,
        presence: PadPresence,
        caps: &Caps,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_pad_template_new_with_gtype(
                name_template.to_glib_none().0,
                direction.to_glib(),
                presence.to_glib(),
                caps.to_glib_none().0,
                pad_type.to_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create pad template"))
        }
    }

    #[doc(alias = "gst_pad_template_get_caps")]
    pub fn caps(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_pad_template_get_caps(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_pad_template_get_documentation_caps")]
    pub fn documentation_caps(&self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_pad_template_get_documentation_caps(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_pad_template_pad_created")]
    pub fn pad_created<P: IsA<Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_pad_template_pad_created(self.to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_pad_template_set_documentation_caps")]
    pub fn set_documentation_caps(&self, caps: &Caps) {
        unsafe {
            ffi::gst_pad_template_set_documentation_caps(
                self.to_glib_none().0,
                caps.to_glib_full(),
            );
        }
    }

    #[doc(alias = "get_property_direction")]
    pub fn direction(&self) -> PadDirection {
        unsafe {
            let mut value = glib::Value::from_type(<PadDirection as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"direction\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `direction` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "get_property_gtype")]
    pub fn gtype(&self) -> glib::types::Type {
        unsafe {
            let mut value =
                glib::Value::from_type(<glib::types::Type as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"gtype\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gtype` getter")
                .unwrap()
        }
    }

    #[doc(alias = "get_property_name_template")]
    pub fn name_template(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"name-template\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name-template` getter")
        }
    }

    #[doc(alias = "get_property_presence")]
    pub fn presence(&self) -> PadPresence {
        unsafe {
            let mut value = glib::Value::from_type(<PadPresence as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"presence\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `presence` getter")
                .unwrap()
        }
    }

    pub fn connect_pad_created<F: Fn(&PadTemplate, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn pad_created_trampoline<
            F: Fn(&PadTemplate, &Pad) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPadTemplate,
            pad: *mut ffi::GstPad,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(pad))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pad-created\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pad_created_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for PadTemplate {}
unsafe impl Sync for PadTemplate {}
