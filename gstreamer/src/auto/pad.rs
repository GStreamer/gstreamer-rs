// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Caps;
use crate::Element;
use crate::Event;
use crate::EventType;
use crate::Object;
use crate::PadDirection;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use crate::PadLinkCheck;
use crate::PadMode;
use crate::PadTemplate;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use crate::Stream;
#[cfg(any(feature = "v1_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
use crate::TaskState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct Pad(Object<ffi::GstPad, ffi::GstPadClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_pad_get_type(),
    }
}

unsafe impl Send for Pad {}
unsafe impl Sync for Pad {}

pub const NONE_PAD: Option<&Pad> = None;

pub trait PadExt: 'static {
    #[doc(alias = "gst_pad_activate_mode")]
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_pad_can_link")]
    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool;

    #[doc(alias = "gst_pad_check_reconfigure")]
    fn check_reconfigure(&self) -> bool;

    #[doc(alias = "gst_pad_create_stream_id")]
    fn create_stream_id<P: IsA<Element>>(
        &self,
        parent: &P,
        stream_id: Option<&str>,
    ) -> glib::GString;

    //#[doc(alias = "gst_pad_create_stream_id_printf")]
    //fn create_stream_id_printf<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> glib::GString;

    //#[doc(alias = "gst_pad_create_stream_id_printf_valist")]
    //fn create_stream_id_printf_valist<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> glib::GString;

    #[doc(alias = "gst_pad_forward")]
    fn forward<P: FnMut(&Pad) -> bool>(&self, forward: P) -> bool;

    #[doc(alias = "gst_pad_get_allowed_caps")]
    fn get_allowed_caps(&self) -> Option<Caps>;

    #[doc(alias = "gst_pad_get_current_caps")]
    fn get_current_caps(&self) -> Option<Caps>;

    #[doc(alias = "gst_pad_get_direction")]
    fn get_direction(&self) -> PadDirection;

    //#[doc(alias = "gst_pad_get_element_private")]
    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[doc(alias = "gst_pad_get_offset")]
    fn get_offset(&self) -> i64;

    #[doc(alias = "gst_pad_get_pad_template")]
    fn get_pad_template(&self) -> Option<PadTemplate>;

    #[doc(alias = "gst_pad_get_pad_template_caps")]
    fn get_pad_template_caps(&self) -> Caps;

    #[doc(alias = "gst_pad_get_parent_element")]
    fn get_parent_element(&self) -> Option<Element>;

    #[doc(alias = "gst_pad_get_peer")]
    fn get_peer(&self) -> Option<Pad>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_pad_get_single_internal_link")]
    fn get_single_internal_link(&self) -> Option<Pad>;

    #[doc(alias = "gst_pad_get_sticky_event")]
    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_pad_get_stream")]
    fn get_stream(&self) -> Option<Stream>;

    #[doc(alias = "gst_pad_get_stream_id")]
    fn get_stream_id(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "gst_pad_get_task_state")]
    fn get_task_state(&self) -> TaskState;

    #[doc(alias = "gst_pad_has_current_caps")]
    fn has_current_caps(&self) -> bool;

    #[doc(alias = "gst_pad_is_active")]
    fn is_active(&self) -> bool;

    #[doc(alias = "gst_pad_is_blocked")]
    fn is_blocked(&self) -> bool;

    #[doc(alias = "gst_pad_is_blocking")]
    fn is_blocking(&self) -> bool;

    #[doc(alias = "gst_pad_is_linked")]
    fn is_linked(&self) -> bool;

    //#[doc(alias = "gst_pad_iterate_internal_links")]
    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator>;

    //#[doc(alias = "gst_pad_iterate_internal_links_default")]
    //fn iterate_internal_links_default<P: IsA<Object>>(&self, parent: Option<&P>) -> /*Ignored*/Option<Iterator>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_pad_link_maybe_ghosting")]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_pad_link_maybe_ghosting_full")]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(
        &self,
        sink: &P,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_pad_mark_reconfigure")]
    fn mark_reconfigure(&self);

    #[doc(alias = "gst_pad_needs_reconfigure")]
    fn needs_reconfigure(&self) -> bool;

    #[doc(alias = "gst_pad_pause_task")]
    fn pause_task(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_pad_peer_query_accept_caps")]
    fn peer_query_accept_caps(&self, caps: &Caps) -> bool;

    #[doc(alias = "gst_pad_peer_query_caps")]
    fn peer_query_caps(&self, filter: Option<&Caps>) -> Caps;

    #[doc(alias = "gst_pad_query_accept_caps")]
    fn query_accept_caps(&self, caps: &Caps) -> bool;

    #[doc(alias = "gst_pad_query_caps")]
    fn query_caps(&self, filter: Option<&Caps>) -> Caps;

    #[doc(alias = "gst_pad_set_active")]
    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //#[doc(alias = "gst_pad_set_element_private")]
    //fn set_element_private(&self, priv_: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[doc(alias = "gst_pad_set_offset")]
    fn set_offset(&self, offset: i64);

    #[doc(alias = "gst_pad_stop_task")]
    fn stop_task(&self) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_pad_unlink")]
    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_pad_use_fixed_caps")]
    fn use_fixed_caps(&self);

    fn get_property_caps(&self) -> Option<Caps>;

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Pad>> PadExt for O {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_activate_mode(
                    self.as_ref().to_glib_none().0,
                    mode.to_glib(),
                    active.to_glib()
                ),
                "Failed to activate mode pad"
            )
        }
    }

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_can_link(
                self.as_ref().to_glib_none().0,
                sinkpad.as_ref().to_glib_none().0,
            ))
        }
    }

    fn check_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_check_reconfigure(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn create_stream_id<P: IsA<Element>>(
        &self,
        parent: &P,
        stream_id: Option<&str>,
    ) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_pad_create_stream_id(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
                stream_id.to_glib_none().0,
            ))
        }
    }

    //fn create_stream_id_printf<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> glib::GString {
    //    unsafe { TODO: call ffi:gst_pad_create_stream_id_printf() }
    //}

    //fn create_stream_id_printf_valist<P: IsA<Element>>(&self, parent: &P, stream_id: Option<&str>, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> glib::GString {
    //    unsafe { TODO: call ffi:gst_pad_create_stream_id_printf_valist() }
    //}

    fn forward<P: FnMut(&Pad) -> bool>(&self, forward: P) -> bool {
        let forward_data: P = forward;
        unsafe extern "C" fn forward_func<P: FnMut(&Pad) -> bool>(
            pad: *mut ffi::GstPad,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let pad = from_glib_borrow(pad);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(&pad);
            res.to_glib()
        }
        let forward = Some(forward_func::<P> as _);
        let super_callback0: &P = &forward_data;
        unsafe {
            from_glib(ffi::gst_pad_forward(
                self.as_ref().to_glib_none().0,
                forward,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    fn get_allowed_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_allowed_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_current_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_current_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_direction(&self) -> PadDirection {
        unsafe { from_glib(ffi::gst_pad_get_direction(self.as_ref().to_glib_none().0)) }
    }

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gst_pad_get_element_private() }
    //}

    fn get_offset(&self) -> i64 {
        unsafe { ffi::gst_pad_get_offset(self.as_ref().to_glib_none().0) }
    }

    fn get_pad_template(&self) -> Option<PadTemplate> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pad_template_caps(&self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_parent_element(&self) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_parent_element(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_peer(&self) -> Option<Pad> {
        unsafe { from_glib_full(ffi::gst_pad_get_peer(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn get_single_internal_link(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_single_internal_link(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_sticky_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib(),
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn get_stream(&self) -> Option<Stream> {
        unsafe { from_glib_full(ffi::gst_pad_get_stream(self.as_ref().to_glib_none().0)) }
    }

    fn get_stream_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_pad_get_stream_id(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    fn get_task_state(&self) -> TaskState {
        unsafe { from_glib(ffi::gst_pad_get_task_state(self.as_ref().to_glib_none().0)) }
    }

    fn has_current_caps(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_has_current_caps(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_active(&self) -> bool {
        unsafe { from_glib(ffi::gst_pad_is_active(self.as_ref().to_glib_none().0)) }
    }

    fn is_blocked(&self) -> bool {
        unsafe { from_glib(ffi::gst_pad_is_blocked(self.as_ref().to_glib_none().0)) }
    }

    fn is_blocking(&self) -> bool {
        unsafe { from_glib(ffi::gst_pad_is_blocking(self.as_ref().to_glib_none().0)) }
    }

    fn is_linked(&self) -> bool {
        unsafe { from_glib(ffi::gst_pad_is_linked(self.as_ref().to_glib_none().0)) }
    }

    //fn iterate_internal_links(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:gst_pad_iterate_internal_links() }
    //}

    //fn iterate_internal_links_default<P: IsA<Object>>(&self, parent: Option<&P>) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:gst_pad_iterate_internal_links_default() }
    //}

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_link_maybe_ghosting(
                    self.as_ref().to_glib_none().0,
                    sink.as_ref().to_glib_none().0
                ),
                "Failed to link pads, possibly ghosting"
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn link_maybe_ghosting_full<P: IsA<Pad>>(
        &self,
        sink: &P,
        flags: PadLinkCheck,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_link_maybe_ghosting_full(
                    self.as_ref().to_glib_none().0,
                    sink.as_ref().to_glib_none().0,
                    flags.to_glib()
                ),
                "Failed to link pads, possibly ghosting"
            )
        }
    }

    fn mark_reconfigure(&self) {
        unsafe {
            ffi::gst_pad_mark_reconfigure(self.as_ref().to_glib_none().0);
        }
    }

    fn needs_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_needs_reconfigure(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pause_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_pause_task(self.as_ref().to_glib_none().0),
                "Failed to pause pad task"
            )
        }
    }

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query_accept_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn peer_query_caps(&self, filter: Option<&Caps>) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_pad_peer_query_caps(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query_accept_caps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
            ))
        }
    }

    fn query_caps(&self, filter: Option<&Caps>) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_pad_query_caps(
                self.as_ref().to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_set_active(self.as_ref().to_glib_none().0, active.to_glib()),
                "Failed to activate pad"
            )
        }
    }

    //fn set_element_private(&self, priv_: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gst_pad_set_element_private() }
    //}

    fn set_offset(&self, offset: i64) {
        unsafe {
            ffi::gst_pad_set_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn stop_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_stop_task(self.as_ref().to_glib_none().0),
                "Failed to stop pad task"
            )
        }
    }

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_pad_unlink(
                    self.as_ref().to_glib_none().0,
                    sinkpad.as_ref().to_glib_none().0
                ),
                "Failed to unlink pad"
            )
        }
    }

    fn use_fixed_caps(&self) {
        unsafe {
            ffi::gst_pad_use_fixed_caps(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_caps(&self) -> Option<Caps> {
        unsafe {
            let mut value = glib::Value::from_type(<Caps as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"caps\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `caps` getter")
        }
    }

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn linked_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(
            this: *mut ffi::GstPad,
            peer: *mut ffi::GstPad,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Pad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"linked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    linked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn unlinked_trampoline<P, F: Fn(&P, &Pad) + Send + Sync + 'static>(
            this: *mut ffi::GstPad,
            peer: *mut ffi::GstPad,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Pad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(peer),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"unlinked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    unlinked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_caps_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(&Pad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Pad>,
        {
            let f: &F = &*(f as *const F);
            f(&Pad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
