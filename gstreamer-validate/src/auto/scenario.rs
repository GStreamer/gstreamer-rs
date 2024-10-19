// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Action, Reporter, Runner};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstValidateScenario")]
    pub struct Scenario(Object<ffi::GstValidateScenario, ffi::GstValidateScenarioClass>) @extends gst::Object, @implements Reporter;

    match fn {
        type_ => || ffi::gst_validate_scenario_get_type(),
    }
}

impl Scenario {
    pub const NONE: Option<&'static Scenario> = None;

    #[doc(alias = "gst_validate_scenario_deinit")]
    pub fn deinit() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_validate_scenario_deinit();
        }
    }

    #[doc(alias = "gst_validate_scenario_factory_create")]
    pub fn factory_create(
        runner: &impl IsA<Runner>,
        pipeline: &impl IsA<gst::Element>,
        scenario_name: &str,
    ) -> Option<Scenario> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_validate_scenario_factory_create(
                runner.as_ref().to_glib_none().0,
                pipeline.as_ref().to_glib_none().0,
                scenario_name.to_glib_none().0,
            ))
        }
    }
}

unsafe impl Send for Scenario {}
unsafe impl Sync for Scenario {}

pub trait ScenarioExt: IsA<Scenario> + 'static {
    //#[doc(alias = "gst_validate_scenario_execute_seek")]
    //fn execute_seek(&self, action: &Action, rate: f64, format: gst::Format, flags: gst::SeekFlags, start_type: gst::SeekType, start: /*Ignored*/gst::ClockTime, stop_type: gst::SeekType, stop: /*Ignored*/gst::ClockTime) -> i32 {
    //    unsafe { TODO: call ffi:gst_validate_scenario_execute_seek() }
    //}

    #[doc(alias = "gst_validate_scenario_get_actions")]
    #[doc(alias = "get_actions")]
    fn actions(&self) -> Vec<Action> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_validate_scenario_get_actions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_scenario_get_pipeline")]
    #[doc(alias = "get_pipeline")]
    fn pipeline(&self) -> Option<gst::Element> {
        unsafe {
            from_glib_full(ffi::gst_validate_scenario_get_pipeline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_scenario_get_target_state")]
    #[doc(alias = "get_target_state")]
    fn target_state(&self) -> gst::State {
        unsafe {
            from_glib(ffi::gst_validate_scenario_get_target_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "execute-on-idle")]
    fn is_execute_on_idle(&self) -> bool {
        ObjectExt::property(self.as_ref(), "execute-on-idle")
    }

    #[doc(alias = "execute-on-idle")]
    fn set_execute_on_idle(&self, execute_on_idle: bool) {
        ObjectExt::set_property(self.as_ref(), "execute-on-idle", execute_on_idle)
    }

    #[doc(alias = "handles-states")]
    fn is_handles_states(&self) -> bool {
        ObjectExt::property(self.as_ref(), "handles-states")
    }

    #[doc(alias = "action-done")]
    fn connect_action_done<F: Fn(&Self, &Action) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn action_done_trampoline<
            P: IsA<Scenario>,
            F: Fn(&P, &Action) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateScenario,
            action: *mut ffi::GstValidateAction,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Scenario::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(action),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    action_done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "done")]
    fn connect_done<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn done_trampoline<
            P: IsA<Scenario>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateScenario,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scenario::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"done\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "execute-on-idle")]
    fn connect_execute_on_idle_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_execute_on_idle_trampoline<
            P: IsA<Scenario>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateScenario,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scenario::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::execute-on-idle\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_execute_on_idle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "handles-states")]
    fn connect_handles_states_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_handles_states_trampoline<
            P: IsA<Scenario>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateScenario,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Scenario::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::handles-states\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_handles_states_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Scenario>> ScenarioExt for O {}
