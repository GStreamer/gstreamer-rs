// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct BaseTransform(Object<ffi::GstBaseTransform, ffi::GstBaseTransformClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || ffi::gst_base_transform_get_type(),
    }
}

unsafe impl Send for BaseTransform {}
unsafe impl Sync for BaseTransform {}

pub const NONE_BASE_TRANSFORM: Option<&BaseTransform> = None;

pub trait BaseTransformExt: 'static {
    //#[doc(alias = "gst_base_transform_get_allocator")]
    //fn allocator(&self, allocator: /*Ignored*/Option<gst::Allocator>, params: /*Ignored*/gst::AllocationParams);

    #[doc(alias = "gst_base_transform_get_buffer_pool")]
    fn buffer_pool(&self) -> Option<gst::BufferPool>;

    #[doc(alias = "gst_base_transform_is_in_place")]
    fn is_in_place(&self) -> bool;

    #[doc(alias = "gst_base_transform_is_passthrough")]
    fn is_passthrough(&self) -> bool;

    #[doc(alias = "gst_base_transform_is_qos_enabled")]
    fn is_qos_enabled(&self) -> bool;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_base_transform_reconfigure")]
    fn reconfigure(&self) -> bool;

    #[doc(alias = "gst_base_transform_reconfigure_sink")]
    fn reconfigure_sink(&self);

    #[doc(alias = "gst_base_transform_reconfigure_src")]
    fn reconfigure_src(&self);

    #[doc(alias = "gst_base_transform_set_gap_aware")]
    fn set_gap_aware(&self, gap_aware: bool);

    #[doc(alias = "gst_base_transform_set_in_place")]
    fn set_in_place(&self, in_place: bool);

    #[doc(alias = "gst_base_transform_set_passthrough")]
    fn set_passthrough(&self, passthrough: bool);

    #[doc(alias = "gst_base_transform_set_prefer_passthrough")]
    fn set_prefer_passthrough(&self, prefer_passthrough: bool);

    #[doc(alias = "gst_base_transform_set_qos_enabled")]
    fn set_qos_enabled(&self, enabled: bool);

    #[doc(alias = "gst_base_transform_update_qos")]
    fn update_qos(&self, proportion: f64, diff: gst::ClockTimeDiff, timestamp: gst::ClockTime);

    #[doc(alias = "gst_base_transform_update_src_caps")]
    fn update_src_caps(&self, updated_caps: &gst::Caps) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "get_property_qos")]
    fn is_qos(&self) -> bool;

    #[doc(alias = "set_property_qos")]
    fn set_qos(&self, qos: bool);

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseTransform>> BaseTransformExt for O {
    //fn allocator(&self, allocator: /*Ignored*/Option<gst::Allocator>, params: /*Ignored*/gst::AllocationParams) {
    //    unsafe { TODO: call ffi:gst_base_transform_get_allocator() }
    //}

    fn buffer_pool(&self) -> Option<gst::BufferPool> {
        unsafe {
            from_glib_full(ffi::gst_base_transform_get_buffer_pool(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_in_place(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_in_place(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_passthrough(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_passthrough(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_is_qos_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_base_transform_reconfigure(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reconfigure_sink(&self) {
        unsafe {
            ffi::gst_base_transform_reconfigure_sink(self.as_ref().to_glib_none().0);
        }
    }

    fn reconfigure_src(&self) {
        unsafe {
            ffi::gst_base_transform_reconfigure_src(self.as_ref().to_glib_none().0);
        }
    }

    fn set_gap_aware(&self, gap_aware: bool) {
        unsafe {
            ffi::gst_base_transform_set_gap_aware(
                self.as_ref().to_glib_none().0,
                gap_aware.to_glib(),
            );
        }
    }

    fn set_in_place(&self, in_place: bool) {
        unsafe {
            ffi::gst_base_transform_set_in_place(
                self.as_ref().to_glib_none().0,
                in_place.to_glib(),
            );
        }
    }

    fn set_passthrough(&self, passthrough: bool) {
        unsafe {
            ffi::gst_base_transform_set_passthrough(
                self.as_ref().to_glib_none().0,
                passthrough.to_glib(),
            );
        }
    }

    fn set_prefer_passthrough(&self, prefer_passthrough: bool) {
        unsafe {
            ffi::gst_base_transform_set_prefer_passthrough(
                self.as_ref().to_glib_none().0,
                prefer_passthrough.to_glib(),
            );
        }
    }

    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_base_transform_set_qos_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn update_qos(&self, proportion: f64, diff: gst::ClockTimeDiff, timestamp: gst::ClockTime) {
        unsafe {
            ffi::gst_base_transform_update_qos(
                self.as_ref().to_glib_none().0,
                proportion,
                diff,
                timestamp.to_glib(),
            );
        }
    }

    fn update_src_caps(&self, updated_caps: &gst::Caps) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_base_transform_update_src_caps(
                    self.as_ref().to_glib_none().0,
                    updated_caps.to_glib_none().0
                ),
                "Failed to update src caps"
            )
        }
    }

    fn is_qos(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `qos` getter")
                .unwrap()
        }
    }

    fn set_qos(&self, qos: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"qos\0".as_ptr() as *const _,
                glib::Value::from(&qos).to_glib_none().0,
            );
        }
    }

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstBaseTransform,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<BaseTransform>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseTransform::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
