// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPContext;
use crate::RTSPThread;
use crate::RTSPThreadType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstRTSPThreadPool")]
    pub struct RTSPThreadPool(Object<ffi::GstRTSPThreadPool, ffi::GstRTSPThreadPoolClass>);

    match fn {
        type_ => || ffi::gst_rtsp_thread_pool_get_type(),
    }
}

impl RTSPThreadPool {
    #[doc(alias = "gst_rtsp_thread_pool_new")]
    pub fn new() -> RTSPThreadPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_thread_pool_new()) }
    }

    #[doc(alias = "gst_rtsp_thread_pool_cleanup")]
    pub fn cleanup() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_rtsp_thread_pool_cleanup();
        }
    }
}

impl Default for RTSPThreadPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPThreadPool {}
unsafe impl Sync for RTSPThreadPool {}

impl RTSPThreadPool {
    pub const NONE: Option<&'static RTSPThreadPool> = None;
}

pub trait RTSPThreadPoolExt: 'static {
    #[doc(alias = "gst_rtsp_thread_pool_get_max_threads")]
    #[doc(alias = "get_max_threads")]
    fn max_threads(&self) -> i32;

    #[doc(alias = "gst_rtsp_thread_pool_get_thread")]
    #[doc(alias = "get_thread")]
    fn thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> Option<RTSPThread>;

    #[doc(alias = "gst_rtsp_thread_pool_set_max_threads")]
    fn set_max_threads(&self, max_threads: i32);

    #[doc(alias = "max-threads")]
    fn connect_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPThreadPool>> RTSPThreadPoolExt for O {
    fn max_threads(&self) -> i32 {
        unsafe { ffi::gst_rtsp_thread_pool_get_max_threads(self.as_ref().to_glib_none().0) }
    }

    fn thread(&self, type_: RTSPThreadType, ctx: &RTSPContext) -> Option<RTSPThread> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_thread_pool_get_thread(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
                ctx.to_glib_none().0,
            ))
        }
    }

    fn set_max_threads(&self, max_threads: i32) {
        unsafe {
            ffi::gst_rtsp_thread_pool_set_max_threads(self.as_ref().to_glib_none().0, max_threads);
        }
    }

    fn connect_max_threads_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_threads_trampoline<
            P: IsA<RTSPThreadPool>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPThreadPool,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPThreadPool::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-threads\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_threads_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
