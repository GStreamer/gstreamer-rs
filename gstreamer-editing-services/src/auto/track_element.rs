// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Edge, EditMode, Extractable, Layer, MetaContainer, TimelineElement, Track, TrackType,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESTrackElement")]
    pub struct TrackElement(Object<ffi::GESTrackElement, ffi::GESTrackElementClass>) @extends TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_track_element_get_type(),
    }
}

impl TrackElement {
    pub const NONE: Option<&'static TrackElement> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TrackElement>> Sealed for T {}
}

pub trait TrackElementExt: IsA<TrackElement> + sealed::Sealed + 'static {
    #[doc(alias = "ges_track_element_add_children_props")]
    fn add_children_props(
        &self,
        element: &impl IsA<gst::Element>,
        wanted_categories: &[&str],
        blacklist: &[&str],
        whitelist: &[&str],
    ) {
        unsafe {
            ffi::ges_track_element_add_children_props(
                self.as_ref().to_glib_none().0,
                element.as_ref().to_glib_none().0,
                wanted_categories.to_glib_none().0,
                blacklist.to_glib_none().0,
                whitelist.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_clamp_control_source")]
    fn clamp_control_source(&self, property_name: &str) {
        unsafe {
            ffi::ges_track_element_clamp_control_source(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "ges_track_element_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_element_edit(
                    self.as_ref().to_glib_none().0,
                    layers.to_glib_none().0,
                    mode.into_glib(),
                    edge.into_glib(),
                    position
                ),
                "Failed to edit"
            )
        }
    }

    //#[doc(alias = "ges_track_element_get_all_control_bindings")]
    //#[doc(alias = "get_all_control_bindings")]
    //fn all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 88 } {
    //    unsafe { TODO: call ffi:ges_track_element_get_all_control_bindings() }
    //}

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_get_auto_clamp_control_sources")]
    #[doc(alias = "get_auto_clamp_control_sources")]
    fn is_auto_clamp_control_sources(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_get_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "ges_track_element_get_child_properties")]
    //#[doc(alias = "get_child_properties")]
    //fn child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:ges_track_element_get_child_properties() }
    //}

    #[doc(alias = "ges_track_element_get_child_property")]
    #[doc(alias = "get_child_property")]
    fn child_property(&self, property_name: &str) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::ges_track_element_get_child_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "ges_track_element_get_child_property_by_pspec")]
    #[doc(alias = "get_child_property_by_pspec")]
    fn child_property_by_pspec(&self, pspec: impl AsRef<glib::ParamSpec>) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::ges_track_element_get_child_property_by_pspec(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    //#[doc(alias = "ges_track_element_get_child_property_valist")]
    //#[doc(alias = "get_child_property_valist")]
    //fn child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_track_element_get_child_property_valist() }
    //}

    #[doc(alias = "ges_track_element_get_control_binding")]
    #[doc(alias = "get_control_binding")]
    fn control_binding(&self, property_name: &str) -> Option<gst::ControlBinding> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_control_binding(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_get_element")]
    #[doc(alias = "get_element")]
    fn element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_get_gnlobject")]
    #[doc(alias = "get_gnlobject")]
    fn gnlobject(&self) -> gst::Element {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_gnlobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_get_nleobject")]
    #[doc(alias = "get_nleobject")]
    fn nleobject(&self) -> gst::Element {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_nleobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_get_track")]
    #[doc(alias = "get_track")]
    fn track(&self) -> Option<Track> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_track(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_get_track_type")]
    #[doc(alias = "get_track_type")]
    fn track_type(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_track_element_get_track_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_has_internal_source")]
    fn has_internal_source(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_has_internal_source(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_is_active")]
    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_is_core")]
    fn is_core(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_core(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_track_element_list_children_properties")]
    fn list_children_properties(&self) -> Vec<glib::ParamSpec> {
        unsafe {
            let mut n_properties = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::ges_track_element_list_children_properties(
                    self.as_ref().to_glib_none().0,
                    n_properties.as_mut_ptr(),
                ),
                n_properties.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "ges_track_element_lookup_child")]
    fn lookup_child(&self, prop_name: &str) -> Option<(gst::Element, glib::ParamSpec)> {
        unsafe {
            let mut element = std::ptr::null_mut();
            let mut pspec = std::ptr::null_mut();
            let ret = from_glib(ffi::ges_track_element_lookup_child(
                self.as_ref().to_glib_none().0,
                prop_name.to_glib_none().0,
                &mut element,
                &mut pspec,
            ));
            if ret {
                Some((from_glib_full(element), from_glib_full(pspec)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "ges_track_element_remove_control_binding")]
    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_element_remove_control_binding(
                    self.as_ref().to_glib_none().0,
                    property_name.to_glib_none().0
                ),
                "Failed to remove control binding"
            )
        }
    }

    #[doc(alias = "ges_track_element_set_active")]
    fn set_active(&self, active: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_active(
                self.as_ref().to_glib_none().0,
                active.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_set_auto_clamp_control_sources")]
    fn set_auto_clamp_control_sources(&self, auto_clamp: bool) {
        unsafe {
            ffi::ges_track_element_set_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
                auto_clamp.into_glib(),
            );
        }
    }

    //#[doc(alias = "ges_track_element_set_child_properties")]
    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:ges_track_element_set_child_properties() }
    //}

    #[doc(alias = "ges_track_element_set_child_property")]
    fn set_child_property(
        &self,
        property_name: &str,
        value: &glib::Value,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_track_element_set_child_property(
                    self.as_ref().to_glib_none().0,
                    property_name.to_glib_none().0,
                    mut_override(value.to_glib_none().0)
                ),
                "Failed to set child property"
            )
        }
    }

    #[doc(alias = "ges_track_element_set_child_property_by_pspec")]
    fn set_child_property_by_pspec(&self, pspec: impl AsRef<glib::ParamSpec>, value: &glib::Value) {
        unsafe {
            ffi::ges_track_element_set_child_property_by_pspec(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                mut_override(value.to_glib_none().0),
            );
        }
    }

    //#[doc(alias = "ges_track_element_set_child_property_valist")]
    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_track_element_set_child_property_valist() }
    //}

    #[doc(alias = "ges_track_element_set_control_source")]
    fn set_control_source(
        &self,
        source: &impl IsA<gst::ControlSource>,
        property_name: &str,
        binding_type: &str,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_control_source(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                binding_type.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_set_has_internal_source")]
    fn set_has_internal_source(&self, has_internal_source: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_has_internal_source(
                self.as_ref().to_glib_none().0,
                has_internal_source.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_track_element_set_track_type")]
    fn set_track_type(&self, type_: TrackType) {
        unsafe {
            ffi::ges_track_element_set_track_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

    #[doc(alias = "control-binding-added")]
    fn connect_control_binding_added<F: Fn(&Self, &gst::ControlBinding) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn control_binding_added_trampoline<
            P: IsA<TrackElement>,
            F: Fn(&P, &gst::ControlBinding) + 'static,
        >(
            this: *mut ffi::GESTrackElement,
            control_binding: *mut gst::ffi::GstControlBinding,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TrackElement::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(control_binding),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"control-binding-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    control_binding_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "control-binding-removed")]
    fn connect_control_binding_removed<F: Fn(&Self, &gst::ControlBinding) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn control_binding_removed_trampoline<
            P: IsA<TrackElement>,
            F: Fn(&P, &gst::ControlBinding) + 'static,
        >(
            this: *mut ffi::GESTrackElement,
            control_binding: *mut gst::ffi::GstControlBinding,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TrackElement::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(control_binding),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"control-binding-removed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    control_binding_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<TrackElement>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "auto-clamp-control-sources")]
    fn connect_auto_clamp_control_sources_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_clamp_control_sources_trampoline<
            P: IsA<TrackElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-clamp-control-sources\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_auto_clamp_control_sources_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "has-internal-source")]
    fn connect_has_internal_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_internal_source_trampoline<
            P: IsA<TrackElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-internal-source\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_internal_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "track")]
    fn connect_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_trampoline<P: IsA<TrackElement>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::track\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_track_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "track-type")]
    fn connect_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_track_type_trampoline<
            P: IsA<TrackElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTrackElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TrackElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::track-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_track_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TrackElement>> TrackElementExt for O {}
