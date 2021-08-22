// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use crate::{
    Bin, Buffer, BufferList, Element, Event, FlowReturn, Message, Object, Pad, PadLinkReturn,
    Query, StateChange, StateChangeReturn, Tracer,
};
use glib::{prelude::*, subclass::prelude::*, translate::*};

#[allow(unused_variables)]
pub trait TracerImpl: TracerImplExt + ObjectImpl + Send + Sync {
    fn bin_add_post(&self, ts: u64, bin: &Bin, element: &Element, success: bool) {}
    fn bin_add_pre(&self, ts: u64, bin: &Bin, element: &Element) {}
    fn bin_remove_post(&self, ts: u64, bin: &Bin, success: bool) {}
    fn bin_remove_pre(&self, ts: u64, bin: &Bin, element: &Element) {}
    fn element_add_pad(&self, ts: u64, element: &Element, pad: &Pad) {}
    fn element_change_state_post(
        &self,
        ts: u64,
        element: &Element,
        change: StateChange,
        result: StateChangeReturn,
    ) {
    }
    fn element_change_state_pre(&self, ts: u64, element: &Element, change: StateChange) {}
    fn element_new(&self, ts: u64, element: &Element) {}
    fn element_post_message_post(&self, ts: u64, element: &Element, success: bool) {}
    fn element_post_message_pre(&self, ts: u64, element: &Element, message: &Message) {}
    fn element_query_post(&self, ts: u64, element: &Element, query: &Query, success: bool) {}
    fn element_query_pre(&self, ts: u64, element: &Element, query: &Query) {}
    fn element_remove_pad(&self, ts: u64, element: &Element, pad: &Pad) {}
    fn object_created(&self, ts: u64, object: &Object) {}
    // rustdoc-stripper-ignore-next
    /// Hook to be called after the GstObject has been finalized.
    fn object_destroyed(&self, ts: u64, object: std::ptr::NonNull<ffi::GstObject>) {}
    fn object_reffed(&self, ts: u64, object: &Object, new_refcount: i32) {}
    fn object_unreffed(&self, ts: u64, object: &Object, new_refcount: i32) {}
    fn pad_link_post(&self, ts: u64, src: &Pad, sink: &Pad, result: PadLinkReturn) {}
    fn pad_link_pre(&self, ts: u64, src: &Pad, sink: &Pad) {}
    fn pad_pull_range_post(&self, ts: u64, pad: &Pad, buffer: &Buffer, result: FlowReturn) {}
    fn pad_pull_range_pre(&self, ts: u64, pad: &Pad, offset: u64, size: u32) {}
    fn pad_push_event_post(&self, ts: u64, pad: &Pad, success: bool) {}
    fn pad_push_event_pre(&self, ts: u64, pad: &Pad, event: &Event) {}
    fn pad_push_list_post(&self, ts: u64, pad: &Pad, result: FlowReturn) {}
    fn pad_push_list_pre(&self, ts: u64, pad: &Pad, buffer_list: &BufferList) {}
    fn pad_push_post(&self, ts: u64, pad: &Pad, result: FlowReturn) {}
    fn pad_push_pre(&self, ts: u64, pad: &Pad, buffer: &Buffer) {}
    fn pad_query_post(&self, ts: u64, pad: &Pad, query: &Query, success: bool) {}
    fn pad_query_pre(&self, ts: u64, pad: &Pad, query: &Query) {}
    fn pad_unlink_post(&self, ts: u64, src: &Pad, sink: &Pad, success: bool) {}
    fn pad_unlink_pre(&self, ts: u64, src: &Pad, sink: &Pad) {}
}

unsafe impl<T: TracerImpl> IsSubclassable<T> for Tracer {
    fn class_init(klass: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(klass);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

pub trait TracerImplExt: ObjectSubclass {
    // rustdoc-stripper-ignore-next
    /// Register a corresponding hook to be called for this tracer when certain events occur.
    ///
    /// Upon an event a corresponding method in `TracerImpl` will be called.
    fn register_hook(&self, hook: TracerHook);
}

macro_rules! define_tracer_hooks {
    ($($name: ident($quark: literal) = |$this: ident, $ts: ident, $($cb_arg: ident: $cb_arg_ty: ty),*| $impl: block;)*) => {
        pub enum TracerHook {
            $($name),*
        }
        impl<T: TracerImpl> TracerImplExt for T {
            fn register_hook(&self, hook: TracerHook) {
                use TracerHook::*;
                let (hook_type, callback) = match hook {
                    $($name => {
                        #[allow(non_snake_case)]
                        unsafe extern "C" fn callback<T: TracerImpl>(
                            $this: *mut ffi::GstTracer,
                            $ts: u64,
                            $($cb_arg: $cb_arg_ty),*
                        ) {
                            let $this = Tracer::from_glib_borrow($this);
                            let $this = T::from_instance($this.unsafe_cast_ref());
                            $impl
                        }
                        (
                            concat!($quark, "\0"),
                            callback::<T> as unsafe extern "C" fn(_, _, $($cb_arg_ty),*) as *const ()
                        )
                    },)*
                };
                unsafe {
                    let instance = self.instance();
                    ffi::gst_tracing_register_hook(
                        instance.to_glib_none().0 as *mut ffi::GstTracer,
                        hook_type.as_ptr() as *const _,
                        Some(std::mem::transmute::<_, extern "C" fn()>(callback)),
                    );
                }
            }
        }
    };
}

define_tracer_hooks! {
    BinAddPost("bin-add-post") = |this, ts, b: *mut ffi::GstBin, e: *mut ffi::GstElement, r: glib::ffi::gboolean| {
        let b = Bin::from_glib_borrow(b);
        let e = Element::from_glib_borrow(e);
        this.bin_add_post(ts, &b, &e, bool::from_glib(r))
    };
    BinAddPre("bin-add-pre") = |this, ts, b: *mut ffi::GstBin, e: *mut ffi::GstElement| {
        let b = Bin::from_glib_borrow(b);
        let e = Element::from_glib_borrow(e);
        this.bin_add_pre(ts, &b, &e)
    };
    BinRemovePost("bin-remove-post") = |this, ts, b: *mut ffi::GstBin, r: glib::ffi::gboolean| {
        let b = Bin::from_glib_borrow(b);
        this.bin_remove_post(ts, &b, bool::from_glib(r))
    };
    BinRemovePre("bin-remove-pre") = |this, ts, b: *mut ffi::GstBin, e: *mut ffi::GstElement| {
        let b = Bin::from_glib_borrow(b);
        let e = Element::from_glib_borrow(e);
        this.bin_remove_pre(ts, &b, &e)
    };
    ElementAddPad("element-add-pad") = |this, ts, e: *mut ffi::GstElement, p: *mut ffi::GstPad| {
        let e = Element::from_glib_borrow(e);
        let p = Pad::from_glib_borrow(p);
        this.element_add_pad(ts, &e, &p)
    };
    ElementChangeStatePost("element-change-state-post") = |this, ts, e: *mut ffi::GstElement, c: ffi::GstStateChange, r: ffi::GstStateChangeReturn| {
        let e = Element::from_glib_borrow(e);
        this.element_change_state_post(ts, &e, StateChange::from_glib(c), StateChangeReturn::from_glib(r))
    };
    ElementChangeStatePre("element-change-state-pre") = |this, ts, e: *mut ffi::GstElement, c: ffi::GstStateChange| {
        let e = Element::from_glib_borrow(e);
        this.element_change_state_pre(ts, &e, StateChange::from_glib(c))
    };
    ElementNew("element-new") = |this, ts, e: *mut ffi::GstElement| {
        let e = Element::from_glib_borrow(e);
        this.element_new(ts, &e)
    };
    ElementPostMessagePost("element-post-message-post") = |this, ts, e: *mut ffi::GstElement, r: glib::ffi::gboolean| {
        let e = Element::from_glib_borrow(e);
        this.element_post_message_post(ts, &e, bool::from_glib(r))
    };
    ElementPostMessagePre("element-post-message-pre") = |this, ts, e: *mut ffi::GstElement, m: *mut ffi::GstMessage| {
        let e = Element::from_glib_borrow(e);
        let m = Message::from_glib_borrow(m);
        this.element_post_message_pre(ts, &e, &m)
    };
    ElementQueryPost("element-query-post") = |this, ts, e: *mut ffi::GstElement, q: *mut ffi::GstQuery, r: glib::ffi::gboolean| {
        let e = Element::from_glib_borrow(e);
        let q = Query::from_glib_borrow(q);
        this.element_query_post(ts, &e, &q, bool::from_glib(r))
    };
    ElementQueryPre("element-query-pre") = |this, ts, e: *mut ffi::GstElement, q: *mut ffi::GstQuery| {
        let e = Element::from_glib_borrow(e);
        let q = Query::from_glib_borrow(q);
        this.element_query_pre(ts, &e, &q)
    };
    ElementRemovePad("element-remove-pad") = |this, ts, e: *mut ffi::GstElement, p: *mut ffi::GstPad| {
        let e = Element::from_glib_borrow(e);
        let p = Pad::from_glib_borrow(p);
        this.element_remove_pad(ts, &e, &p)
    };
    // TODO(#353): Missing bindings to `GstMiniObject`.
    // MiniObjectCreated,
    // MiniObjectDestroyed,
    // MiniObjectReffed,
    // MiniObjectUnreffed,
    ObjectCreated("object-created") = |this, ts, o: *mut ffi::GstObject| {
        let o = Object::from_glib_borrow(o);
        this.object_created(ts, &o)
    };
    // TODO: unclear what to do here as the `GstObject` here is no longer valid…
    ObjectDestroyed("object-destroyed") = |this, ts, o: *mut ffi::GstObject| {
        this.object_destroyed(ts, std::ptr::NonNull::new_unchecked(o))
    };
    ObjectReffed("object-reffed") = |this, ts, o: *mut ffi::GstObject, rc: libc::c_int| {
        let o = Object::from_glib_borrow(o);
        this.object_reffed(ts, &o, rc as i32)
    };
    ObjectUnreffed("object-unreffed") = |this, ts, o: *mut ffi::GstObject, rc: libc::c_int| {
        let o = Object::from_glib_borrow(o);
        this.object_unreffed(ts, &o, rc as i32)
    };
    PadLinkPost("pad-link-post") = |this, ts, src: *mut ffi::GstPad, sink: *mut ffi::GstPad, r: ffi::GstPadLinkReturn| {
        let src = Pad::from_glib_borrow(src);
        let sink = Pad::from_glib_borrow(sink);
        this.pad_link_post(ts, &src, &sink, PadLinkReturn::from_glib(r))
    };
    PadLinkPre("pad-link-pre") = |this, ts, src: *mut ffi::GstPad, sink: *mut ffi::GstPad| {
        let src = Pad::from_glib_borrow(src);
        let sink = Pad::from_glib_borrow(sink);
        this.pad_link_pre(ts, &src, &sink)
    };
    PadPullRangePost("pad-pull-range-post") = |this, ts, p: *mut ffi::GstPad, b: *mut ffi::GstBuffer, r: ffi::GstFlowReturn| {
        let p = Pad::from_glib_borrow(p);
        let b = Buffer::from_glib_borrow(b);
        this.pad_pull_range_post(ts, &p, &b, FlowReturn::from_glib(r))
    };
    PadPullRangePre("pad-pull-range-pre") = |this, ts, p: *mut ffi::GstPad, o: u64, s: libc::c_uint| {
        let p = Pad::from_glib_borrow(p);
        this.pad_pull_range_pre(ts, &p, o, s as u32)
    };
    PadPushEventPost("pad-push-event-post") = |this, ts, p: *mut ffi::GstPad, r: glib::ffi::gboolean| {
        let p = Pad::from_glib_borrow(p);
        this.pad_push_event_post(ts, &p, bool::from_glib(r))
    };
    PadPushEventPre("pad-push-event-pre") = |this, ts, p: *mut ffi::GstPad, e: *mut ffi::GstEvent| {
        let p = Pad::from_glib_borrow(p);
        let e = Event::from_glib_borrow(e);
        this.pad_push_event_pre(ts, &p, &e)
    };
    PadPushListPost("pad-push-list-post") = |this, ts, p: *mut ffi::GstPad, r: ffi::GstFlowReturn| {
        let p = Pad::from_glib_borrow(p);
        this.pad_push_list_post(ts, &p, FlowReturn::from_glib(r))
    };
    PadPushListPre("pad-push-list-pre") = |this, ts, p: *mut ffi::GstPad, bl: *mut ffi::GstBufferList| {
        let p = Pad::from_glib_borrow(p);
        let bl = BufferList::from_glib_borrow(bl);
        this.pad_push_list_pre(ts, &p, &bl)
    };
    PadPushPost("pad-push-post") = |this, ts, p: *mut ffi::GstPad, r: ffi::GstFlowReturn| {
        let p = Pad::from_glib_borrow(p);
        this.pad_push_post(ts, &p, FlowReturn::from_glib(r))
    };
    PadPushPre("pad-push-pre") = |this, ts, p: *mut ffi::GstPad, b: *mut ffi::GstBuffer| {
        let p = Pad::from_glib_borrow(p);
        let b = Buffer::from_glib_borrow(b);
        this.pad_push_pre(ts, &p, &b)
    };
    PadQueryPost("pad-query-post") = |this, ts, p: *mut ffi::GstPad, q: *mut ffi::GstQuery, r: glib::ffi::gboolean| {
        let p = Pad::from_glib_borrow(p);
        let q = Query::from_glib_borrow(q);
        this.pad_query_post(ts, &p, &q, bool::from_glib(r))
    };
    PadQueryPre("pad-query-pre") = |this, ts, p: *mut ffi::GstPad, q: *mut ffi::GstQuery| {
        let p = Pad::from_glib_borrow(p);
        let q = Query::from_glib_borrow(q);
        this.pad_query_pre(ts, &p, &q)
    };
    PadUnlinkPost("pad-unlink-post") = |this, ts, src: *mut ffi::GstPad, sink: *mut ffi::GstPad, r: glib::ffi::gboolean| {
        let src = Pad::from_glib_borrow(src);
        let sink = Pad::from_glib_borrow(sink);
        this.pad_unlink_post(ts, &src, &sink, bool::from_glib(r))
    };
    PadUnlinkPre("pad-unlink-pre") = |this, ts, src: *mut ffi::GstPad, sink: *mut ffi::GstPad| {
        let src = Pad::from_glib_borrow(src);
        let sink = Pad::from_glib_borrow(sink);
        this.pad_unlink_pre(ts, &src, &sink)
    };
}
