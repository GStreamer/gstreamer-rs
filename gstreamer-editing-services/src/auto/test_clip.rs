// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{
    ffi, Clip, Container, Extractable, MetaContainer, SourceClip, TimelineElement, VideoTestPattern,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GESTestClip")]
    pub struct TestClip(Object<ffi::GESTestClip, ffi::GESTestClipClass>) @extends SourceClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_test_clip_get_type(),
    }
}

impl TestClip {
    pub const NONE: Option<&'static TestClip> = None;

    #[doc(alias = "ges_test_clip_new")]
    pub fn new() -> Option<TestClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_test_clip_new()) }
    }

    #[doc(alias = "ges_test_clip_new_for_nick")]
    #[doc(alias = "new_for_nick")]
    pub fn for_nick(nick: &str) -> Option<TestClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_test_clip_new_for_nick(nick.to_glib_none().0)) }
    }
}

pub trait TestClipExt: IsA<TestClip> + 'static {
    #[doc(alias = "ges_test_clip_get_frequency")]
    #[doc(alias = "get_frequency")]
    fn frequency(&self) -> f64 {
        unsafe { ffi::ges_test_clip_get_frequency(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_test_clip_get_volume")]
    #[doc(alias = "get_volume")]
    fn volume(&self) -> f64 {
        unsafe { ffi::ges_test_clip_get_volume(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_test_clip_get_vpattern")]
    #[doc(alias = "get_vpattern")]
    fn vpattern(&self) -> VideoTestPattern {
        unsafe {
            from_glib(ffi::ges_test_clip_get_vpattern(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_test_clip_is_muted")]
    fn is_muted(&self) -> bool {
        unsafe { from_glib(ffi::ges_test_clip_is_muted(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "ges_test_clip_set_frequency")]
    fn set_frequency(&self, freq: f64) {
        unsafe {
            ffi::ges_test_clip_set_frequency(self.as_ref().to_glib_none().0, freq);
        }
    }

    #[doc(alias = "ges_test_clip_set_mute")]
    #[doc(alias = "mute")]
    fn set_mute(&self, mute: bool) {
        unsafe {
            ffi::ges_test_clip_set_mute(self.as_ref().to_glib_none().0, mute.into_glib());
        }
    }

    #[doc(alias = "ges_test_clip_set_volume")]
    #[doc(alias = "volume")]
    fn set_volume(&self, volume: f64) {
        unsafe {
            ffi::ges_test_clip_set_volume(self.as_ref().to_glib_none().0, volume);
        }
    }

    #[doc(alias = "ges_test_clip_set_vpattern")]
    #[doc(alias = "vpattern")]
    fn set_vpattern(&self, vpattern: VideoTestPattern) {
        unsafe {
            ffi::ges_test_clip_set_vpattern(self.as_ref().to_glib_none().0, vpattern.into_glib());
        }
    }

    fn freq(&self) -> f64 {
        ObjectExt::property(self.as_ref(), "freq")
    }

    fn set_freq(&self, freq: f64) {
        ObjectExt::set_property(self.as_ref(), "freq", freq)
    }

    #[doc(alias = "freq")]
    fn connect_freq_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_freq_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::freq".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_freq_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mute")]
    fn connect_mute_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mute_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::mute".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mute_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume")]
    fn connect_volume_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_volume_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::volume".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_volume_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vpattern")]
    fn connect_vpattern_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vpattern_trampoline<P: IsA<TestClip>, F: Fn(&P) + 'static>(
            this: *mut ffi::GESTestClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TestClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::vpattern".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_vpattern_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TestClip>> TestClipExt for O {}
