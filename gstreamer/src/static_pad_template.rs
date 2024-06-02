// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, marker::PhantomData, ptr};

use glib::{prelude::*, translate::*};

use crate::{ffi, Caps, PadTemplate};

#[doc(alias = "GstStaticPadTemplate")]
#[derive(Clone, Copy)]
pub struct StaticPadTemplate(ptr::NonNull<ffi::GstStaticPadTemplate>);

impl StaticPadTemplate {
    #[doc(alias = "gst_static_pad_template_get")]
    #[inline]
    pub fn get(&self) -> PadTemplate {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get(self.0.as_ptr())) }
    }

    #[doc(alias = "get_caps")]
    #[doc(alias = "gst_static_pad_template_get_caps")]
    #[inline]
    pub fn caps(&self) -> Caps {
        unsafe { from_glib_full(ffi::gst_static_pad_template_get_caps(self.0.as_ptr())) }
    }

    #[inline]
    pub fn name_template<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(self.0.as_ref().name_template)
                .to_str()
                .unwrap()
        }
    }

    #[inline]
    pub fn direction(&self) -> crate::PadDirection {
        unsafe { from_glib(self.0.as_ref().direction) }
    }

    #[inline]
    pub fn presence(&self) -> crate::PadPresence {
        unsafe { from_glib(self.0.as_ref().presence) }
    }
}

unsafe impl glib::translate::TransparentPtrType for StaticPadTemplate {}

unsafe impl Send for StaticPadTemplate {}
unsafe impl Sync for StaticPadTemplate {}

impl fmt::Debug for StaticPadTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StaticPadTemplate")
            .field("name_template", &unsafe {
                CStr::from_ptr(self.0.as_ref().name_template).to_str()
            })
            .field("direction", &unsafe {
                from_glib::<_, crate::PadDirection>(self.0.as_ref().direction)
            })
            .field("presence", &unsafe {
                from_glib::<_, crate::PadPresence>(self.0.as_ref().presence)
            })
            .field("static_caps", &unsafe {
                from_glib_none::<_, crate::StaticCaps>(&self.0.as_ref().static_caps as *const _)
            })
            .finish()
    }
}

impl glib::types::StaticType for StaticPadTemplate {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_static_pad_template_get_type()) }
    }
}

impl glib::value::ValueType for StaticPadTemplate {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for StaticPadTemplate {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstStaticPadTemplate)
    }
}

#[doc(hidden)]
impl glib::value::ToValue for StaticPadTemplate {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for StaticPadTemplate {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}

impl From<StaticPadTemplate> for glib::Value {
    #[inline]
    fn from(v: StaticPadTemplate) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for StaticPadTemplate {
    type GlibType = *mut ffi::GstStaticPadTemplate;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstStaticPadTemplate> for StaticPadTemplate {
    type Storage = PhantomData<&'a StaticPadTemplate>;

    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const ffi::GstStaticPadTemplate, Self> {
        glib::translate::Stash(self.0.as_ptr(), PhantomData)
    }

    fn to_glib_full(&self) -> *const ffi::GstStaticPadTemplate {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStaticPadTemplate) -> Self {
        debug_assert!(!ptr.is_null());
        StaticPadTemplate(ptr::NonNull::new_unchecked(ptr as *mut _))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        debug_assert!(!ptr.is_null());
        StaticPadTemplate(ptr::NonNull::new_unchecked(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrBorrow<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStaticPadTemplate) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        Borrowed::new(StaticPadTemplate(ptr::NonNull::new_unchecked(ptr)))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstStaticPadTemplate> for StaticPadTemplate {
    unsafe fn from_glib_full(_ptr: *mut ffi::GstStaticPadTemplate) -> Self {
        unimplemented!();
    }
}
