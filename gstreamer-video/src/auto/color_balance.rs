// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, ColorBalanceChannel, ColorBalanceType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstColorBalance")]
    pub struct ColorBalance(Interface<ffi::GstColorBalance, ffi::GstColorBalanceInterface>);

    match fn {
        type_ => || ffi::gst_color_balance_get_type(),
    }
}

impl ColorBalance {
    pub const NONE: Option<&'static ColorBalance> = None;
}

unsafe impl Send for ColorBalance {}
unsafe impl Sync for ColorBalance {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ColorBalance>> Sealed for T {}
}

pub trait ColorBalanceExt: IsA<ColorBalance> + sealed::Sealed + 'static {
    #[doc(alias = "gst_color_balance_get_balance_type")]
    #[doc(alias = "get_balance_type")]
    fn balance_type(&self) -> ColorBalanceType {
        unsafe {
            from_glib(ffi::gst_color_balance_get_balance_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_color_balance_get_value")]
    #[doc(alias = "get_value")]
    fn value(&self, channel: &impl IsA<ColorBalanceChannel>) -> i32 {
        unsafe {
            ffi::gst_color_balance_get_value(
                self.as_ref().to_glib_none().0,
                channel.as_ref().to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gst_color_balance_list_channels")]
    fn list_channels(&self) -> Vec<ColorBalanceChannel> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_color_balance_list_channels(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_color_balance_set_value")]
    fn set_value(&self, channel: &impl IsA<ColorBalanceChannel>, value: i32) {
        unsafe {
            ffi::gst_color_balance_set_value(
                self.as_ref().to_glib_none().0,
                channel.as_ref().to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "gst_color_balance_value_changed")]
    fn value_changed(&self, channel: &impl IsA<ColorBalanceChannel>, value: i32) {
        unsafe {
            ffi::gst_color_balance_value_changed(
                self.as_ref().to_glib_none().0,
                channel.as_ref().to_glib_none().0,
                value,
            );
        }
    }

    #[doc(alias = "value-changed")]
    fn connect_value_changed<F: Fn(&Self, &ColorBalanceChannel, i32) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<
            P: IsA<ColorBalance>,
            F: Fn(&P, &ColorBalanceChannel, i32) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstColorBalance,
            channel: *mut ffi::GstColorBalanceChannel,
            value: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ColorBalance::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(channel),
                value,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ColorBalance>> ColorBalanceExt for O {}
