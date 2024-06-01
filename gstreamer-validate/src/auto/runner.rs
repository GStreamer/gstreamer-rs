// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Report, ReportingDetails};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstValidateRunner")]
    pub struct Runner(Object<ffi::GstValidateRunner, ffi::GstValidateRunnerClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_validate_runner_get_type(),
    }
}

impl Runner {
    pub const NONE: Option<&'static Runner> = None;

    #[doc(alias = "gst_validate_runner_new")]
    pub fn new() -> Runner {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_validate_runner_new()) }
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Runner {}
unsafe impl Sync for Runner {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Runner>> Sealed for T {}
}

pub trait RunnerExt: IsA<Runner> + sealed::Sealed + 'static {
    #[doc(alias = "gst_validate_runner_add_report")]
    fn add_report(&self, report: &Report) {
        unsafe {
            ffi::gst_validate_runner_add_report(
                self.as_ref().to_glib_none().0,
                report.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_validate_runner_exit")]
    fn exit(&self, print_result: bool) -> i32 {
        unsafe {
            ffi::gst_validate_runner_exit(self.as_ref().to_glib_none().0, print_result.into_glib())
        }
    }

    #[doc(alias = "gst_validate_runner_get_default_reporting_level")]
    #[doc(alias = "get_default_reporting_level")]
    fn default_reporting_level(&self) -> ReportingDetails {
        unsafe {
            from_glib(ffi::gst_validate_runner_get_default_reporting_level(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_runner_get_reporting_level_for_name")]
    #[doc(alias = "get_reporting_level_for_name")]
    fn reporting_level_for_name(&self, name: &str) -> ReportingDetails {
        unsafe {
            from_glib(ffi::gst_validate_runner_get_reporting_level_for_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_runner_get_reports")]
    #[doc(alias = "get_reports")]
    fn reports(&self) -> Vec<Report> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_validate_runner_get_reports(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_validate_runner_get_reports_count")]
    #[doc(alias = "get_reports_count")]
    fn reports_count(&self) -> u32 {
        unsafe { ffi::gst_validate_runner_get_reports_count(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_validate_runner_printf")]
    fn printf(&self) -> i32 {
        unsafe { ffi::gst_validate_runner_printf(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "report-added")]
    fn connect_report_added<F: Fn(&Self, &Report) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn report_added_trampoline<
            P: IsA<Runner>,
            F: Fn(&P, &Report) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateRunner,
            object: *mut ffi::GstValidateReport,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Runner::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"report-added\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    report_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "stopping")]
    fn connect_stopping<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stopping_trampoline<
            P: IsA<Runner>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstValidateRunner,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Runner::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stopping\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    stopping_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Runner>> RunnerExt for O {}
