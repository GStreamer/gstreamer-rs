// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem::transmute;

use glib::{
    ffi::{gboolean, gpointer},
    prelude::*,
    source::Priority,
    translate::*,
    ControlFlow,
};

use crate::{ffi, RTSPSessionPool};

unsafe extern "C" fn trampoline_watch<
    F: FnMut(&RTSPSessionPool) -> ControlFlow + Send + 'static,
>(
    pool: *mut ffi::GstRTSPSessionPool,
    func: gpointer,
) -> gboolean {
    let func: &mut F = &mut *(func as *mut F);
    func(&from_glib_borrow(pool)).into_glib()
}

unsafe extern "C" fn destroy_closure_watch<
    F: FnMut(&RTSPSessionPool) -> ControlFlow + Send + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_watch<F: FnMut(&RTSPSessionPool) -> ControlFlow + Send + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPSessionPool>> Sealed for T {}
}

pub trait RTSPSessionPoolExtManual: sealed::Sealed + IsA<RTSPSessionPool> + 'static {
    #[doc(alias = "gst_rtsp_session_pool_create_watch")]
    fn create_watch<F>(&self, name: Option<&str>, priority: Priority, func: F) -> glib::Source
    where
        F: FnMut(&RTSPSessionPool) -> ControlFlow + Send + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let source = ffi::gst_rtsp_session_pool_create_watch(self.as_ref().to_glib_none().0);
            glib::ffi::g_source_set_callback(
                source,
                Some(transmute::<
                    *mut (),
                    unsafe extern "C" fn(glib::ffi::gpointer) -> i32,
                >(trampoline_watch::<F> as *mut ())),
                into_raw_watch(func),
                Some(destroy_closure_watch::<F>),
            );
            glib::ffi::g_source_set_priority(source, priority.into_glib());

            if let Some(name) = name {
                glib::ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }
}

impl<O: IsA<RTSPSessionPool>> RTSPSessionPoolExtManual for O {}
