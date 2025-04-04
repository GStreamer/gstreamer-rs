// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Edge, EditMode, Extractable, Layer, MetaContainer, TimelineElement};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESContainer")]
    pub struct Container(Object<ffi::GESContainer, ffi::GESContainerClass>) @extends TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_container_get_type(),
    }
}

impl Container {
    pub const NONE: Option<&'static Container> = None;

    #[doc(alias = "ges_container_group")]
    pub fn group(containers: &[Container]) -> Option<Container> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_container_group(containers.to_glib_none().0)) }
    }
}

pub trait GESContainerExt: IsA<Container> + 'static {
    #[doc(alias = "ges_container_add")]
    fn add(&self, child: &impl IsA<TimelineElement>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_add(
                    self.as_ref().to_glib_none().0,
                    child.as_ref().to_glib_none().0
                ),
                "Failed to add element"
            )
        }
    }

    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[allow(deprecated)]
    #[doc(alias = "ges_container_edit")]
    fn edit(
        &self,
        layers: &[Layer],
        new_layer_priority: i32,
        mode: EditMode,
        edge: Edge,
        position: u64,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_edit(
                    self.as_ref().to_glib_none().0,
                    layers.to_glib_none().0,
                    new_layer_priority,
                    mode.into_glib(),
                    edge.into_glib(),
                    position
                ),
                "Failed to edit container"
            )
        }
    }

    #[doc(alias = "ges_container_get_children")]
    #[doc(alias = "get_children")]
    fn children(&self, recursive: bool) -> Vec<TimelineElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_get_children(
                self.as_ref().to_glib_none().0,
                recursive.into_glib(),
            ))
        }
    }

    #[doc(alias = "ges_container_remove")]
    fn remove(&self, child: &impl IsA<TimelineElement>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_container_remove(
                    self.as_ref().to_glib_none().0,
                    child.as_ref().to_glib_none().0
                ),
                "Failed to remove element"
            )
        }
    }

    #[doc(alias = "ges_container_ungroup")]
    fn ungroup(self, recursive: bool) -> Vec<Container> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_ungroup(
                self.upcast().into_glib_ptr(),
                recursive.into_glib(),
            ))
        }
    }

    fn height(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "height")
    }

    #[doc(alias = "child-added")]
    fn connect_child_added<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_added_trampoline<
            P: IsA<Container>,
            F: Fn(&P, &TimelineElement) + 'static,
        >(
            this: *mut ffi::GESContainer,
            element: *mut ffi::GESTimelineElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"child-added".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    child_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child-removed")]
    fn connect_child_removed<F: Fn(&Self, &TimelineElement) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_removed_trampoline<
            P: IsA<Container>,
            F: Fn(&P, &TimelineElement) + 'static,
        >(
            this: *mut ffi::GESContainer,
            element: *mut ffi::GESTimelineElement,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(element),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"child-removed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    child_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<Container>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESContainer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Container::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Container>> GESContainerExt for O {}
