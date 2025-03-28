// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, ClockTime, Object};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstControlBinding")]
    pub struct ControlBinding(Object<ffi::GstControlBinding, ffi::GstControlBindingClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_control_binding_get_type(),
    }
}

impl ControlBinding {
    pub const NONE: Option<&'static ControlBinding> = None;
}

unsafe impl Send for ControlBinding {}
unsafe impl Sync for ControlBinding {}

pub trait ControlBindingExt: IsA<ControlBinding> + 'static {
    #[doc(alias = "gst_control_binding_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self, timestamp: ClockTime) -> Option<glib::Value> {
        unsafe {
            from_glib_full(ffi::gst_control_binding_get_value(
                self.as_ref().to_glib_none().0,
                timestamp.into_glib(),
            ))
        }
    }

    //#[doc(alias = "gst_control_binding_get_value_array")]
    //#[doc(alias = "get_value_array")]
    //fn is_value_array(&self, timestamp: impl Into<Option<ClockTime>>, interval: impl Into<Option<ClockTime>>, values: /*Unimplemented*/&[&Basic: Pointer]) -> bool {
    //    unsafe { TODO: call ffi:gst_control_binding_get_value_array() }
    //}

    #[doc(alias = "gst_control_binding_is_disabled")]
    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_control_binding_is_disabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_control_binding_set_disabled")]
    fn set_disabled(&self, disabled: bool) {
        unsafe {
            ffi::gst_control_binding_set_disabled(
                self.as_ref().to_glib_none().0,
                disabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_control_binding_sync_values")]
    fn sync_values(
        &self,
        object: &impl IsA<Object>,
        timestamp: ClockTime,
        last_sync: impl Into<Option<ClockTime>>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_control_binding_sync_values(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                timestamp.into_glib(),
                last_sync.into().into_glib(),
            ))
        }
    }

    fn object(&self) -> Option<Object> {
        ObjectExt::property(self.as_ref(), "object")
    }
}

impl<O: IsA<ControlBinding>> ControlBindingExt for O {}
