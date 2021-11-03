// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::BaseEffect;
use crate::Extractable;
use crate::MetaContainer;
use crate::Operation;
use crate::TimelineElement;
use crate::TrackElement;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    #[doc(alias = "GESEffect")]
    pub struct Effect(Object<ffi::GESEffect, ffi::GESEffectClass>) @extends BaseEffect, Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_effect_get_type(),
    }
}

impl Effect {
    #[doc(alias = "ges_effect_new")]
    pub fn new(bin_description: &str) -> Result<Effect, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_effect_new(bin_description.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to create effect from description"))
        }
    }
}

impl Effect {
    pub const NONE: Option<&'static Effect> = None;
}

pub trait EffectExt: 'static {
    #[doc(alias = "bin-description")]
    fn bin_description(&self) -> Option<glib::GString>;
}

impl<O: IsA<Effect>> EffectExt for O {
    fn bin_description(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"bin-description\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `bin-description` getter")
        }
    }
}
