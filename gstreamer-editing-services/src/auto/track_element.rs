// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Edge;
use crate::EditMode;
use crate::Extractable;
use crate::Layer;
use crate::MetaContainer;
use crate::TimelineElement;
use crate::Track;
use crate::TrackType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

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

pub trait TrackElementExt: 'static {
    #[doc(alias = "ges_track_element_add_children_props")]
    fn add_children_props(
        &self,
        element: &impl IsA<gst::Element>,
        wanted_categories: &[&str],
        blacklist: &[&str],
        whitelist: &[&str],
    );

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_clamp_control_source")]
    fn clamp_control_source(&self, property_name: &str);

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[doc(alias = "ges_track_element_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "ges_track_element_get_all_control_bindings")]
    //#[doc(alias = "get_all_control_bindings")]
    //fn all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 85 };

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_get_auto_clamp_control_sources")]
    #[doc(alias = "get_auto_clamp_control_sources")]
    fn is_auto_clamp_control_sources(&self) -> bool;

    #[doc(alias = "ges_track_element_get_control_binding")]
    #[doc(alias = "get_control_binding")]
    fn control_binding(&self, property_name: &str) -> Option<gst::ControlBinding>;

    #[doc(alias = "ges_track_element_get_element")]
    #[doc(alias = "get_element")]
    fn element(&self) -> Option<gst::Element>;

    #[doc(alias = "ges_track_element_get_gnlobject")]
    #[doc(alias = "get_gnlobject")]
    fn gnlobject(&self) -> Option<gst::Element>;

    #[doc(alias = "ges_track_element_get_nleobject")]
    #[doc(alias = "get_nleobject")]
    fn nleobject(&self) -> Option<gst::Element>;

    #[doc(alias = "ges_track_element_get_track")]
    #[doc(alias = "get_track")]
    fn track(&self) -> Option<Track>;

    #[doc(alias = "ges_track_element_get_track_type")]
    #[doc(alias = "get_track_type")]
    fn track_type(&self) -> TrackType;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_has_internal_source")]
    fn has_internal_source(&self) -> bool;

    #[doc(alias = "ges_track_element_is_active")]
    fn is_active(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_is_core")]
    fn is_core(&self) -> bool;

    //#[doc(alias = "ges_track_element_lookup_child")]
    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element>;

    #[doc(alias = "ges_track_element_remove_control_binding")]
    fn remove_control_binding(&self, property_name: &str) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "ges_track_element_set_active")]
    fn set_active(&self, active: bool) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_set_auto_clamp_control_sources")]
    fn set_auto_clamp_control_sources(&self, auto_clamp: bool);

    #[doc(alias = "ges_track_element_set_control_source")]
    fn set_control_source(
        &self,
        source: &impl IsA<gst::ControlSource>,
        property_name: &str,
        binding_type: &str,
    ) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_track_element_set_has_internal_source")]
    fn set_has_internal_source(&self, has_internal_source: bool) -> bool;

    #[doc(alias = "ges_track_element_set_track_type")]
    fn set_track_type(&self, type_: TrackType);

    #[doc(alias = "control-binding-added")]
    fn connect_control_binding_added<F: Fn(&Self, &gst::ControlBinding) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "control-binding-removed")]
    fn connect_control_binding_removed<F: Fn(&Self, &gst::ControlBinding) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "auto-clamp-control-sources")]
    fn connect_auto_clamp_control_sources_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "has-internal-source")]
    fn connect_has_internal_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "track")]
    fn connect_track_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "track-type")]
    fn connect_track_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TrackElement>> TrackElementExt for O {
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

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn clamp_control_source(&self, property_name: &str) {
        unsafe {
            ffi::ges_track_element_clamp_control_source(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

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

    //fn all_control_bindings(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 6, id: 85 } {
    //    unsafe { TODO: call ffi:ges_track_element_get_all_control_bindings() }
    //}

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_auto_clamp_control_sources(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_get_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn control_binding(&self, property_name: &str) -> Option<gst::ControlBinding> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_control_binding(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn element(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn gnlobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_gnlobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn nleobject(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_nleobject(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn track(&self) -> Option<Track> {
        unsafe {
            from_glib_none(ffi::ges_track_element_get_track(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn track_type(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_track_element_get_track_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn has_internal_source(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_has_internal_source(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_active(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn is_core(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_is_core(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn lookup_child(&self, prop_name: &str, pspec: /*Ignored*/glib::ParamSpec) -> Option<gst::Element> {
    //    unsafe { TODO: call ffi:ges_track_element_lookup_child() }
    //}

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

    fn set_active(&self, active: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_active(
                self.as_ref().to_glib_none().0,
                active.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_auto_clamp_control_sources(&self, auto_clamp: bool) {
        unsafe {
            ffi::ges_track_element_set_auto_clamp_control_sources(
                self.as_ref().to_glib_none().0,
                auto_clamp.into_glib(),
            );
        }
    }

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

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn set_has_internal_source(&self, has_internal_source: bool) -> bool {
        unsafe {
            from_glib(ffi::ges_track_element_set_has_internal_source(
                self.as_ref().to_glib_none().0,
                has_internal_source.into_glib(),
            ))
        }
    }

    fn set_track_type(&self, type_: TrackType) {
        unsafe {
            ffi::ges_track_element_set_track_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    control_binding_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    control_binding_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_clamp_control_sources_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_internal_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_track_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
