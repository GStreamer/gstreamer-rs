// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::{ffi, prelude::*};
use glib::translate::*;

pub trait IsMiniObject:
    AsRef<Self::RefType> + FromGlibPtrFull<*mut Self::FfiType> + Send + Sync + 'static
{
    type RefType;
    type FfiType;
}

#[macro_export]
macro_rules! mini_object_wrapper (
    ($name:ident, $ref_name:ident, $ffi_name:path) => {
        #[repr(transparent)]
        pub struct $name {
            obj: std::ptr::NonNull<$ffi_name>,
        }

        #[repr(transparent)]
        pub struct $ref_name($ffi_name);

        impl $crate::miniobject::IsMiniObject for $name {
            type RefType = $ref_name;
            type FfiType = $ffi_name;
        }

        impl $name {
            #[inline]
            pub unsafe fn from_glib_ptr_borrow(
                ptr: &*mut $ffi_name,
            ) -> &Self {
                debug_assert_eq!(std::mem::size_of::<$name>(), std::mem::size_of::<$crate::glib::ffi::gpointer>());
                debug_assert!(!ptr.is_null());
                &*(ptr as *const *mut $ffi_name as *const $name)
            }

            #[inline]
            pub unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                skip_assert_initialized!();
                debug_assert!(!ptr.is_null());

                $crate::ffi::gst_mini_object_ref(ptr as *mut $crate::ffi::GstMiniObject);

                $name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                }
            }

            #[inline]
            pub unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                skip_assert_initialized!();
                debug_assert!(!ptr.is_null());

                $name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                }
            }

            #[inline]
            pub unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                skip_assert_initialized!();
                debug_assert!(!ptr.is_null());

                $crate::glib::translate::Borrowed::new($name {
                    obj: std::ptr::NonNull::new_unchecked(ptr as *mut $ffi_name),
                })
            }

            #[inline]
            pub unsafe fn replace_ptr(&mut self, ptr: *mut $ffi_name) {
                debug_assert!(!ptr.is_null());
                self.obj = std::ptr::NonNull::new_unchecked(ptr);
            }

            #[inline]
            #[doc(alias = "gst_mini_object_make_writable")]
            pub fn make_mut(&mut self) -> &mut $ref_name {
                unsafe {
                    if self.is_writable() {
                        return &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name);
                    }

                    let ptr = $crate::ffi::gst_mini_object_make_writable(
                        self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject
                    );
                    self.replace_ptr(ptr as *mut $ffi_name);
                    debug_assert!(self.is_writable());

                    &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name)
                }
            }

            #[inline]
            pub fn get_mut(&mut self) -> Option<&mut $ref_name> {
                if self.is_writable() {
                    Some(unsafe { &mut *(self.obj.as_mut() as *mut $ffi_name as *mut $ref_name) })
                } else {
                    None
                }
            }

            #[doc(alias = "gst_mini_object_is_writable")]
            #[inline]
            pub fn is_writable(&self) -> bool {
                unsafe {
                    $crate::glib::translate::from_glib($crate::ffi::gst_mini_object_is_writable(
                        self.as_ptr() as *const $crate::ffi::GstMiniObject
                    ))
                }
            }

            #[must_use]
            #[inline]
            pub fn upcast(self) -> $crate::miniobject::MiniObject {
                use $crate::glib::translate::IntoGlibPtr;

                unsafe {
                    from_glib_full(self.into_glib_ptr() as *mut $crate::ffi::GstMiniObject)
                }
            }
        }

        impl $crate::glib::translate::IntoGlibPtr<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn into_glib_ptr(self) -> *mut $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
                s.as_mut_ptr()
            }
        }

        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                unsafe { $name::from_glib_none(self.as_ptr()) }
            }
        }

        impl Drop for $name {
            #[inline]
            fn drop(&mut self) {
                unsafe {
                    $crate::ffi::gst_mini_object_unref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                }
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ref_name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.obj.as_ref() as *const $ffi_name as *const $ref_name) }
            }
        }

        impl AsRef<$ref_name> for $name {
            #[inline]
            fn as_ref(&self) -> &$ref_name {
                &*self
            }
        }

        impl std::borrow::Borrow<$ref_name> for $name {
            #[inline]
            fn borrow(&self) -> &$ref_name {
                &*self
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *const $ffi_name> for $name {
            type Storage = std::marker::PhantomData<&'a Self>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *const $ffi_name, Self> {
                $crate::glib::translate::Stash(self.as_ptr(), std::marker::PhantomData)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                unsafe {
                    $crate::ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                    self.as_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtr<'a, *mut $ffi_name> for $name {
            type Storage = std::marker::PhantomData<&'a Self>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::glib::translate::Stash<'a, *mut $ffi_name, Self> {
                $crate::glib::translate::Stash(self.as_mut_ptr(), std::marker::PhantomData)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                unsafe {
                    $crate::ffi::gst_mini_object_ref(self.as_mut_ptr() as *mut $crate::ffi::GstMiniObject);
                    self.as_mut_ptr()
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name {
            type Storage = std::marker::PhantomData<&'a mut Self>;

            #[inline]
            fn to_glib_none_mut(&'_ mut self) -> $crate::glib::translate::StashMut<*mut $ffi_name, Self> {
                self.make_mut();
                $crate::glib::translate::StashMut(self.as_mut_ptr(), std::marker::PhantomData)
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name {
            #[allow(clippy::type_complexity)]
            type Storage = (
                std::marker::PhantomData<&'a [$name]>,
                Option<Vec<*mut $ffi_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                let mut v_ptr = Vec::with_capacity(t.len() + 1);
                unsafe {
                    let ptr = v_ptr.as_mut_ptr();
                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *mut $ffi_name, ptr, t.len());
                    std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
                    v_ptr.set_len(t.len() + 1);
                }

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (std::marker::PhantomData, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();

                let v_ptr = unsafe {
                    let v_ptr = $crate::glib::ffi::g_malloc(std::mem::size_of::<*mut $ffi_name>() * t.len() + 1)
                        as *mut *mut $ffi_name;

                    std::ptr::copy_nonoverlapping(t.as_ptr() as *mut *mut $ffi_name, v_ptr, t.len());
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                };

                (v_ptr, (std::marker::PhantomData, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut *mut $ffi_name {
                skip_assert_initialized!();
                unsafe {
                    let v_ptr = $crate::glib::ffi::g_malloc(std::mem::size_of::<*mut $ffi_name>() * t.len() + 1)
                        as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::glib::translate::ToGlibPtr::to_glib_full(s));
                    }
                    std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());

                    v_ptr
                }
            }
        }

        impl<'a> $crate::glib::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name>
            for $name
        {
            #[allow(clippy::type_complexity)]
            type Storage = (
                std::marker::PhantomData<&'a [$name]>,
                Option<Vec<*mut $ffi_name>>,
            );

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                let (ptr, stash) =
                    $crate::glib::translate::ToGlibContainerFromSlice::<'a, *mut *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name]) -> (*const *mut $ffi_name, Self::Storage) {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name]) -> *const *mut $ffi_name {
                skip_assert_initialized!();
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrNone<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                Self::from_glib_none(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrFull<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                Self::from_glib_full(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*const $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::glib::translate::Borrowed<Self> {
                Self::from_glib_borrow(ptr)
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::<Self>::with_capacity(num);
                let res_ptr = res.as_mut_ptr();
                for i in 0..num {
                    ::std::ptr::write(res_ptr.add(i), $crate::glib::translate::from_glib_none(std::ptr::read(ptr.add(i))));
                }
                res.set_len(num);
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::glib::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                let res_ptr = res.as_mut_ptr();
                ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
                res.set_len(num);
                $crate::glib::ffi::g_free(ptr as *mut _);
                res
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, glib::translate::c_ptr_array_len(ptr))
            }
        }

        impl $crate::glib::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name>
            for $name
        {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::glib::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name
        {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::glib::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $name {
            type GlibType = *mut $ffi_name;
        }

        unsafe impl $crate::glib::translate::TransparentPtrType for $name {}

        impl $ref_name {
            #[inline]
            pub fn as_ptr(&self) -> *const $ffi_name {
                self as *const Self as *const $ffi_name
            }

            #[inline]
            pub fn as_mut_ptr(&self) -> *mut $ffi_name {
                self as *const Self as *mut $ffi_name
            }

            #[inline]
            pub unsafe fn from_ptr<'a>(ptr: *const $ffi_name) -> &'a Self {
                debug_assert!(!ptr.is_null());
                &*(ptr as *const Self)
            }

            #[inline]
            pub unsafe fn from_mut_ptr<'a>(ptr: *mut $ffi_name) -> &'a mut Self {
                debug_assert!(!ptr.is_null());
                debug_assert_ne!(
                    $crate::ffi::gst_mini_object_is_writable(ptr as *mut $crate::ffi::GstMiniObject),
                    $crate::glib::ffi::GFALSE
                );
                &mut *(ptr as *mut Self)
            }

            #[doc(alias = "gst_mini_object_copy")]
            #[inline]
            pub fn copy(&self) -> $name {
                unsafe {
                    $name::from_glib_full($crate::ffi::gst_mini_object_copy(
                        self.as_ptr() as *const $crate::ffi::GstMiniObject
                    ) as *const $ffi_name)
                }
            }

            #[inline]
            pub fn upcast_ref(&self) -> &$crate::miniobject::MiniObjectRef {
                unsafe {
                    &*(self.as_ptr() as *const $crate::miniobject::MiniObjectRef)
                }
            }

            #[inline]
            pub fn upcast_mut(&mut self) -> &mut $crate::miniobject::MiniObjectRef {
                unsafe {
                    &mut *(self.as_mut_ptr() as *mut $crate::miniobject::MiniObjectRef)
                }
            }

            #[inline]
            pub fn ptr_eq(this: &$ref_name, other: &$ref_name) -> bool {
                skip_assert_initialized!();
                this.as_ptr() == other.as_ptr()
            }
        }

        impl $crate::glib::translate::GlibPtrDefault for $ref_name {
            type GlibType = *mut $ffi_name;
        }

        impl ToOwned for $ref_name {
            type Owned = $name;

            #[inline]
            fn to_owned(&self) -> $name {
                self.copy()
            }
        }

        unsafe impl Sync for $ref_name {}
        unsafe impl Send for $ref_name {}
        unsafe impl Sync for $name {}
        unsafe impl Send for $name {}
    };
    ($name:ident, $ref_name:ident, $ffi_name:path, $get_type:expr) => {
        $crate::mini_object_wrapper!($name, $ref_name, $ffi_name);

        impl $crate::glib::types::StaticType for $name {
            #[inline]
            fn static_type() -> $crate::glib::types::Type {
                $ref_name::static_type()
            }
        }

        #[allow(clippy::redundant_closure_call)]
        impl $crate::glib::types::StaticType for $ref_name {
            #[inline]
            fn static_type() -> $crate::glib::types::Type {
                #[allow(clippy::macro_metavars_in_unsafe)]
                unsafe { $crate::glib::translate::from_glib($get_type()) }
            }
        }

        impl glib::value::ValueType for $name {
            type Type = Self;
        }

        impl glib::value::ValueTypeOptional for $name { }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for $name {
            type Checker = $crate::glib::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                $crate::glib::translate::from_glib_none(
                    $crate::glib::gobject_ffi::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(value).0) as *mut $ffi_name
                )
            }
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for &'a $name {
            type Checker = $crate::glib::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                let value = &*(value as *const $crate::glib::Value as *const $crate::glib::gobject_ffi::GValue);
                $name::from_glib_ptr_borrow(&*(&value.data[0].v_pointer as *const $crate::glib::ffi::gpointer as *const *mut $ffi_name))
            }
        }

        impl $crate::glib::value::ToValue for $name {
            #[inline]
            fn to_value(&self) -> $crate::glib::Value {
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(self).0 as *mut _,
                    )
                }
                value
            }

            #[inline]
            fn value_type(&self) -> $crate::glib::Type {
                <Self as $crate::glib::prelude::StaticType>::static_type()
            }
        }

        impl $crate::glib::value::ToValueOptional for $name {
            #[inline]
            fn to_value_optional(s: Option<&Self>) -> $crate::glib::Value {
                skip_assert_initialized!();
                let mut value = $crate::glib::Value::for_value_type::<Self>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_set_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&s).0 as *mut _,
                    )
                }
                value
            }
        }

        impl From<$name> for $crate::glib::Value {
            #[inline]
            fn from(v: $name) -> $crate::glib::Value {
                skip_assert_initialized!();
                let mut value = $crate::glib::Value::for_value_type::<$name>();
                unsafe {
                    $crate::glib::gobject_ffi::g_value_take_boxed(
                        $crate::glib::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::glib::translate::IntoGlibPtr::<*mut $ffi_name>::into_glib_ptr(v) as *mut _,
                    )
                }
                value
            }
        }

        unsafe impl<'a> $crate::glib::value::FromValue<'a> for &'a $ref_name {
            type Checker = $crate::glib::value::GenericValueTypeOrNoneChecker<Self>;

            #[inline]
            unsafe fn from_value(value: &'a $crate::glib::Value) -> Self {
                skip_assert_initialized!();
                &*($crate::glib::gobject_ffi::g_value_get_boxed($crate::glib::translate::ToGlibPtr::to_glib_none(value).0) as *const $ref_name)
            }
        }

        // Can't have SetValue/SetValueOptional impls as otherwise one could use it to get
        // immutable references from a mutable reference without borrowing via the value

        impl $crate::glib::prelude::HasParamSpec for $name {
            type ParamSpec = $crate::glib::ParamSpecBoxed;
            type SetValue = Self;
            type BuilderFn = fn(&str) -> $crate::glib::ParamSpecBoxedBuilder<Self>;

            fn param_spec_builder() -> Self::BuilderFn {
                |name| Self::ParamSpec::builder(name)
            }
        }
    };
);

#[cfg(not(any(feature = "v1_20", docsrs)))]
mini_object_wrapper!(MiniObject, MiniObjectRef, ffi::GstMiniObject);

#[cfg(feature = "v1_20")]
mini_object_wrapper!(MiniObject, MiniObjectRef, ffi::GstMiniObject, || {
    ffi::gst_mini_object_get_type()
});

impl MiniObject {
    #[inline]
    pub fn downcast<T: IsMiniObject + StaticType>(self) -> Result<T, Self> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Ok(from_glib_full(self.into_glib_ptr() as *mut T::FfiType)) }
        } else {
            Err(self)
        }
    }
}

impl fmt::Debug for MiniObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl fmt::Debug for MiniObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MiniObject")
            .field("ptr", &self.as_ptr())
            .field("type", &self.type_())
            .finish()
    }
}

impl MiniObjectRef {
    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn downcast_ref<T: IsMiniObject + StaticType>(&self) -> Option<&T::RefType> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Some(&*(self as *const Self as *const T::RefType)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: IsMiniObject + StaticType>(&mut self) -> Option<&mut T::RefType> {
        if self.type_().is_a(T::static_type()) {
            unsafe { Some(&mut *(self as *mut Self as *mut T::RefType)) }
        } else {
            None
        }
    }
}
