// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Extractable, MetaContainer, Timeline, TrackType};
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::{Edge, EditMode, Layer};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESTimelineElement")]
    pub struct TimelineElement(Object<ffi::GESTimelineElement, ffi::GESTimelineElementClass>) @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_timeline_element_get_type(),
    }
}

impl TimelineElement {
    pub const NONE: Option<&'static TimelineElement> = None;
}

pub trait TimelineElementExt: IsA<TimelineElement> + 'static {
    #[doc(alias = "ges_timeline_element_add_child_property")]
    fn add_child_property(
        &self,
        pspec: impl AsRef<glib::ParamSpec>,
        child: &impl IsA<glib::Object>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_add_child_property(
                    self.as_ref().to_glib_none().0,
                    pspec.as_ref().to_glib_none().0,
                    child.as_ref().to_glib_none().0
                ),
                "Failed to add child property"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_copy")]
    #[must_use]
    fn copy(&self, deep: bool) -> TimelineElement {
        unsafe {
            from_glib_none(ffi::ges_timeline_element_copy(
                self.as_ref().to_glib_none().0,
                deep.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_edit(
                self.as_ref().to_glib_none().0,
                layers.to_glib_none().0,
                new_layer_priority,
                mode.into_glib(),
                edge.into_glib(),
                position,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_edit_full")]
    fn edit_full(
        &self,
        new_layer_priority: i64,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_timeline_element_edit_full(
                self.as_ref().to_glib_none().0,
                new_layer_priority,
                mode.into_glib(),
                edge.into_glib(),
                position,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[doc(alias = "ges_timeline_element_get_child_properties")]
    //#[doc(alias = "get_child_properties")]
    //fn child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_properties() }
    //}

    #[doc(alias = "ges_timeline_element_get_child_property")]
    #[doc(alias = "get_child_property")]
    fn child_property(&self, property_name: &str) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::ges_timeline_element_get_child_property(
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

    #[doc(alias = "ges_timeline_element_get_child_property_by_pspec")]
    #[doc(alias = "get_child_property_by_pspec")]
    fn child_property_by_pspec(&self, pspec: impl AsRef<glib::ParamSpec>) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::ges_timeline_element_get_child_property_by_pspec(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    //#[doc(alias = "ges_timeline_element_get_child_property_valist")]
    //#[doc(alias = "get_child_property_valist")]
    //fn child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_get_child_property_valist() }
    //}

    #[doc(alias = "ges_timeline_element_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_timeline_element_get_duration(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[doc(alias = "ges_timeline_element_get_inpoint")]
    #[doc(alias = "get_inpoint")]
    fn inpoint(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_timeline_element_get_inpoint(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "ges_timeline_element_get_layer_priority")]
    #[doc(alias = "get_layer_priority")]
    fn layer_priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_layer_priority(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_timeline_element_get_max_duration")]
    #[doc(alias = "get_max_duration")]
    #[doc(alias = "max-duration")]
    fn max_duration(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_get_natural_framerate")]
    #[doc(alias = "get_natural_framerate")]
    fn natural_framerate(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut framerate_n = std::mem::MaybeUninit::uninit();
            let mut framerate_d = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::ges_timeline_element_get_natural_framerate(
                self.as_ref().to_glib_none().0,
                framerate_n.as_mut_ptr(),
                framerate_d.as_mut_ptr(),
            ));
            if ret {
                Some((framerate_n.assume_init(), framerate_d.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "ges_timeline_element_get_parent")]
    #[doc(alias = "get_parent")]
    #[must_use]
    fn parent(&self) -> Option<TimelineElement> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_get_priority")]
    #[doc(alias = "get_priority")]
    fn priority(&self) -> u32 {
        unsafe { ffi::ges_timeline_element_get_priority(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_timeline_element_get_start")]
    #[doc(alias = "get_start")]
    fn start(&self) -> gst::ClockTime {
        unsafe {
            try_from_glib(ffi::ges_timeline_element_get_start(
                self.as_ref().to_glib_none().0,
            ))
            .expect("mandatory glib value is None")
        }
    }

    #[doc(alias = "ges_timeline_element_get_timeline")]
    #[doc(alias = "get_timeline")]
    fn timeline(&self) -> Option<Timeline> {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_timeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_get_toplevel_parent")]
    #[doc(alias = "get_toplevel_parent")]
    #[must_use]
    fn toplevel_parent(&self) -> TimelineElement {
        unsafe {
            from_glib_full(ffi::ges_timeline_element_get_toplevel_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_get_track_types")]
    #[doc(alias = "get_track_types")]
    fn track_types(&self) -> TrackType {
        unsafe {
            from_glib(ffi::ges_timeline_element_get_track_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_list_children_properties")]
    fn list_children_properties(&self) -> Vec<glib::ParamSpec> {
        unsafe {
            let mut n_properties = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::ges_timeline_element_list_children_properties(
                    self.as_ref().to_glib_none().0,
                    n_properties.as_mut_ptr(),
                ),
                n_properties.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "ges_timeline_element_lookup_child")]
    fn lookup_child(&self, prop_name: &str) -> Option<(glib::Object, glib::ParamSpec)> {
        unsafe {
            let mut child = std::ptr::null_mut();
            let mut pspec = std::ptr::null_mut();
            let ret = from_glib(ffi::ges_timeline_element_lookup_child(
                self.as_ref().to_glib_none().0,
                prop_name.to_glib_none().0,
                &mut child,
                &mut pspec,
            ));
            if ret {
                Some((from_glib_full(child), from_glib_full(pspec)))
            } else {
                None
            }
        }
    }

    #[doc(alias = "ges_timeline_element_paste")]
    fn paste(&self, paste_position: gst::ClockTime) -> Result<TimelineElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::ges_timeline_element_paste(
                self.as_ref().to_glib_none().0,
                paste_position.into_glib(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to paste timeline element"))
        }
    }

    #[doc(alias = "ges_timeline_element_remove_child_property")]
    fn remove_child_property(
        &self,
        pspec: impl AsRef<glib::ParamSpec>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_remove_child_property(
                    self.as_ref().to_glib_none().0,
                    pspec.as_ref().to_glib_none().0
                ),
                "Failed to remove child property"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_ripple")]
    fn ripple(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_ripple(self.as_ref().to_glib_none().0, start.into_glib()),
                "Failed to ripple"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_ripple_end")]
    fn ripple_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_ripple_end(
                    self.as_ref().to_glib_none().0,
                    end.into_glib()
                ),
                "Failed to ripple"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_roll_end")]
    fn roll_end(&self, end: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_roll_end(self.as_ref().to_glib_none().0, end.into_glib()),
                "Failed to roll"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_roll_start")]
    fn roll_start(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_roll_start(
                    self.as_ref().to_glib_none().0,
                    start.into_glib()
                ),
                "Failed to roll"
            )
        }
    }

    //#[doc(alias = "ges_timeline_element_set_child_properties")]
    //fn set_child_properties(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_properties() }
    //}

    #[doc(alias = "ges_timeline_element_set_child_property")]
    fn set_child_property(
        &self,
        property_name: &str,
        value: &glib::Value,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_child_property(
                    self.as_ref().to_glib_none().0,
                    property_name.to_glib_none().0,
                    value.to_glib_none().0
                ),
                "Failed to set child property"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_set_child_property_by_pspec")]
    fn set_child_property_by_pspec(&self, pspec: impl AsRef<glib::ParamSpec>, value: &glib::Value) {
        unsafe {
            ffi::ges_timeline_element_set_child_property_by_pspec(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_timeline_element_set_child_property_full")]
    fn set_child_property_full(
        &self,
        property_name: &str,
        value: &glib::Value,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::ges_timeline_element_set_child_property_full(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[doc(alias = "ges_timeline_element_set_child_property_valist")]
    //fn set_child_property_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:ges_timeline_element_set_child_property_valist() }
    //}

    #[doc(alias = "ges_timeline_element_set_duration")]
    #[doc(alias = "duration")]
    fn set_duration(&self, duration: impl Into<Option<gst::ClockTime>>) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_duration(
                self.as_ref().to_glib_none().0,
                duration.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_set_inpoint")]
    fn set_inpoint(&self, inpoint: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_inpoint(
                self.as_ref().to_glib_none().0,
                inpoint.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_set_max_duration")]
    #[doc(alias = "max-duration")]
    fn set_max_duration(&self, maxduration: impl Into<Option<gst::ClockTime>>) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_max_duration(
                self.as_ref().to_glib_none().0,
                maxduration.into().into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_set_name")]
    #[doc(alias = "name")]
    fn set_name(&self, name: Option<&str>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_name(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0
                ),
                "Failed to set name"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_set_parent")]
    #[doc(alias = "parent")]
    fn set_parent(&self, parent: &impl IsA<TimelineElement>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "`TimelineElement` already had a parent or its parent was the same as specified"
            )
        }
    }

    #[deprecated = "Since 1.10"]
    #[allow(deprecated)]
    #[doc(alias = "ges_timeline_element_set_priority")]
    #[doc(alias = "priority")]
    fn set_priority(&self, priority: u32) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_priority(
                self.as_ref().to_glib_none().0,
                priority,
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_set_start")]
    #[doc(alias = "start")]
    fn set_start(&self, start: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::ges_timeline_element_set_start(
                self.as_ref().to_glib_none().0,
                start.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_timeline_element_set_timeline")]
    #[doc(alias = "timeline")]
    fn set_timeline(&self, timeline: &impl IsA<Timeline>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_set_timeline(
                    self.as_ref().to_glib_none().0,
                    timeline.as_ref().to_glib_none().0
                ),
                "`Failed to set timeline"
            )
        }
    }

    #[doc(alias = "ges_timeline_element_trim")]
    fn trim(&self, start: gst::ClockTime) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_timeline_element_trim(self.as_ref().to_glib_none().0, start.into_glib()),
                "Failed to trim"
            )
        }
    }

    #[doc(alias = "in-point")]
    fn in_point(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "in-point")
    }

    #[doc(alias = "in-point")]
    fn set_in_point(&self, in_point: u64) {
        ObjectExt::set_property(self.as_ref(), "in-point", in_point)
    }

    fn is_serialize(&self) -> bool {
        ObjectExt::property(self.as_ref(), "serialize")
    }

    fn set_serialize(&self, serialize: bool) {
        ObjectExt::set_property(self.as_ref(), "serialize", serialize)
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "child-property-added")]
    fn connect_child_property_added<F: Fn(&Self, &glib::Object, &glib::ParamSpec) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_property_added_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P, &glib::Object, &glib::ParamSpec) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            prop_object: *mut glib::gobject_ffi::GObject,
            prop: *mut glib::gobject_ffi::GParamSpec,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TimelineElement::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(prop_object),
                &from_glib_borrow(prop),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"child-property-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    child_property_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "child-property-removed")]
    fn connect_child_property_removed<F: Fn(&Self, &glib::Object, &glib::ParamSpec) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_property_removed_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P, &glib::Object, &glib::ParamSpec) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            prop_object: *mut glib::gobject_ffi::GObject,
            prop: *mut glib::gobject_ffi::GParamSpec,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TimelineElement::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(prop_object),
                &from_glib_borrow(prop),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"child-property-removed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    child_property_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "deep-notify")]
    fn connect_deep_notify<F: Fn(&Self, &glib::Object, &glib::ParamSpec) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn deep_notify_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P, &glib::Object, &glib::ParamSpec) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            prop_object: *mut glib::gobject_ffi::GObject,
            prop: *mut glib::gobject_ffi::GParamSpec,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                TimelineElement::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(prop_object),
                &from_glib_borrow(prop),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("deep-notify::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"deep-notify\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    deep_notify_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::duration".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "in-point")]
    fn connect_in_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_in_point_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::in-point".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_in_point_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-duration")]
    fn connect_max_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_duration_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::max-duration".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_max_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::parent".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[deprecated = "Since 1.10"]
    #[doc(alias = "priority")]
    fn connect_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_priority_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::priority".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_priority_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "serialize")]
    fn connect_serialize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_serialize_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::serialize".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_serialize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "start")]
    fn connect_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::start".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "timeline")]
    fn connect_timeline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeline_trampoline<
            P: IsA<TimelineElement>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTimelineElement,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TimelineElement::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::timeline".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TimelineElement>> TimelineElementExt for O {}
