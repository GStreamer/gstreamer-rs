// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::*;
use glib::value::{FromValueOptional, ToValue};
use glib::StaticType;
use glib::Value;
use glib_ffi;
use glib_ffi::{gconstpointer, gpointer};
use gobject_ffi;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::iter::Iterator as StdIterator;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IteratorError {
    Resync,
    Error,
}

impl fmt::Display for IteratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IteratorError::Resync => write!(f, "Resync"),
            IteratorError::Error => write!(f, "Error"),
        }
    }
}

impl Error for IteratorError {
    fn description(&self) -> &str {
        match *self {
            IteratorError::Resync => "Resync",
            IteratorError::Error => "Error",
        }
    }
}

// Implemented manually so that we can use generics for the item
pub struct Iterator<T> {
    iter: ptr::NonNull<ffi::GstIterator>,
    borrowed: bool,
    phantom: PhantomData<T>,
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    pub fn resync(&mut self) {
        unsafe {
            ffi::gst_iterator_resync(self.to_glib_none_mut().0);
        }
    }

    pub fn filter<F>(self, func: F) -> Self
    where
        F: Fn(T) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let it = self.to_glib_none().0;
            mem::forget(self);

            let func_box: Box<Fn(T) -> bool + Send + Sync + 'static> = Box::new(func);
            // FIXME: Use Value::from_type once we depend on new enough GLib
            let mut closure_value = glib::Value::uninitialized();
            gobject_ffi::g_value_init(
                closure_value.to_glib_none_mut().0,
                filter_boxed_get_type::<T>(),
            );
            gobject_ffi::g_value_set_boxed(
                closure_value.to_glib_none_mut().0,
                Arc::into_raw(Arc::new(func_box)) as gpointer,
            );

            from_glib_full(ffi::gst_iterator_filter(
                it as *mut _,
                Some(filter_trampoline::<T>),
                closure_value.to_glib_none().0,
            ))
        }
    }

    pub fn find_simple<F>(&mut self, func: F) -> Option<T>
    where
        F: FnMut(T) -> bool,
    {
        unsafe {
            let mut elem = glib::Value::uninitialized();

            let mut func = func;
            let func_obj: &mut (FnMut(T) -> bool) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(T) -> bool) as gpointer;

            let res = from_glib(ffi::gst_iterator_find_custom(
                self.to_glib_none_mut().0,
                Some(find_trampoline::<T>),
                elem.to_glib_none_mut().0,
                func_ptr,
            ));
            if res {
                Some(elem.get::<T>().unwrap())
            } else {
                None
            }
        }
    }

    pub fn foreach<F>(&mut self, func: F) -> Result<(), IteratorError>
    where
        F: FnMut(T),
    {
        unsafe {
            let mut func = func;
            let func_obj: &mut (FnMut(T)) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(T)) as gpointer;

            let res = ffi::gst_iterator_foreach(
                self.to_glib_none_mut().0,
                Some(foreach_trampoline::<T>),
                func_ptr,
            );

            match res {
                ffi::GST_ITERATOR_OK | ffi::GST_ITERATOR_DONE => Ok(()),
                ffi::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                ffi::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }

    pub fn fold_with_early_exit<F, U>(&mut self, init: U, func: F) -> Result<U, IteratorError>
    where
        F: FnMut(U, T) -> Result<U, U>,
    {
        unsafe {
            let mut func = func;
            let func_obj: &mut (FnMut(U, T) -> Result<U, U>) = &mut func;
            let func_ptr = &func_obj as *const &mut (FnMut(U, T) -> Result<U, U>) as gpointer;

            let mut accum = Some(init);
            // FIXME: Use Value::from_type once we depend on new enough GLib
            let mut ret = glib::Value::uninitialized();
            gobject_ffi::g_value_init(ret.to_glib_none_mut().0, gobject_ffi::G_TYPE_POINTER);
            gobject_ffi::g_value_set_pointer(
                ret.to_glib_none_mut().0,
                &mut accum as *mut _ as gpointer,
            );

            let res = ffi::gst_iterator_fold(
                self.to_glib_none_mut().0,
                Some(fold_trampoline::<T, U>),
                ret.to_glib_none_mut().0,
                func_ptr,
            );

            match res {
                ffi::GST_ITERATOR_OK | ffi::GST_ITERATOR_DONE => Ok(accum.unwrap()),
                ffi::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                ffi::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    pub fn new<I: IteratorImpl<T>>(imp: I) -> Self {
        static DUMMY_COOKIE: u32 = 0;

        unsafe {
            let it = ffi::gst_iterator_new(
                mem::size_of::<RsIterator<T, I>>() as u32,
                T::static_type().to_glib(),
                ptr::null_mut(),
                &DUMMY_COOKIE as *const _ as *mut _,
                Some(rs_iterator_copy::<T, I>),
                Some(rs_iterator_next::<T, I>),
                None,
                Some(rs_iterator_resync::<T, I>),
                Some(rs_iterator_free::<T, I>),
            );

            {
                let it = it as *mut RsIterator<T, I>;
                (*it).imp = Some(imp);
            }

            from_glib_full(it)
        }
    }
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Clone + Send + 'static,
{
    pub fn from_vec(items: Vec<T>) -> Self {
        Self::new(VecIteratorImpl::new(items))
    }
}

impl<T> StdIterator for Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    type Item = Result<T, IteratorError>;

    fn next(&mut self) -> Option<Result<T, IteratorError>> {
        unsafe {
            let mut value = Value::uninitialized();
            let res = ffi::gst_iterator_next(self.to_glib_none_mut().0, value.to_glib_none_mut().0);
            match res {
                ffi::GST_ITERATOR_OK => match value.get::<T>() {
                    Some(value) => Some(Ok(value)),
                    None => Some(Err(IteratorError::Error)),
                },
                ffi::GST_ITERATOR_DONE => None,
                ffi::GST_ITERATOR_RESYNC => Some(Err(IteratorError::Resync)),
                ffi::GST_ITERATOR_ERROR | _ => Some(Err(IteratorError::Error)),
            }
        }
    }
}

#[repr(C)]
struct RsIterator<T, I: IteratorImpl<T>>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    iter: ffi::GstIterator,
    imp: Option<I>,
    phantom: PhantomData<T>,
}

pub trait IteratorImpl<T>: Clone + Send + 'static
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>>;
    fn resync(&mut self);
}

unsafe extern "C" fn rs_iterator_copy<T, I: IteratorImpl<T>>(
    it: *const ffi::GstIterator,
    copy: *mut ffi::GstIterator,
) where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    callback_guard!();
    let it = it as *const RsIterator<T, I>;
    let copy = copy as *mut RsIterator<T, I>;

    ptr::write(&mut (*copy).imp, (*it).imp.clone());
}

unsafe extern "C" fn rs_iterator_free<T, I: IteratorImpl<T>>(it: *mut ffi::GstIterator)
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    callback_guard!();
    let it = it as *mut RsIterator<T, I>;
    let _ = (*it).imp.take();
}

unsafe extern "C" fn rs_iterator_next<T, I: IteratorImpl<T>>(
    it: *mut ffi::GstIterator,
    result: *mut gobject_ffi::GValue,
) -> ffi::GstIteratorResult
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    callback_guard!();
    let it = it as *mut RsIterator<T, I>;
    match (*it).imp.as_mut().map(|imp| imp.next()).unwrap() {
        Some(Ok(value)) => {
            let value = value.to_value();
            ptr::write(result, ptr::read(value.to_glib_none().0));
            mem::forget(value);
            ffi::GST_ITERATOR_OK
        }
        None => ffi::GST_ITERATOR_DONE,
        Some(Err(res)) => match res {
            IteratorError::Resync => ffi::GST_ITERATOR_RESYNC,
            IteratorError::Error => ffi::GST_ITERATOR_ERROR,
        },
    }
}

unsafe extern "C" fn rs_iterator_resync<T, I: IteratorImpl<T>>(it: *mut ffi::GstIterator)
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    callback_guard!();
    let it = it as *mut RsIterator<T, I>;
    (*it).imp.as_mut().map(|imp| imp.resync()).unwrap();
}

#[derive(Clone)]
struct VecIteratorImpl<T> {
    pos: usize,
    items: Vec<T>,
}

impl<T> VecIteratorImpl<T>
where
    for<'a> T: StaticType + ToValue + FromValueOptional<'a> + Clone + Send + 'static,
{
    fn new(items: Vec<T>) -> Self {
        Self {
            pos: 0,
            items: items,
        }
    }
}

impl<T> IteratorImpl<T> for VecIteratorImpl<T>
where
    for<'a> T: StaticType + ToValue + FromValueOptional<'a> + Clone + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>> {
        if self.pos < self.items.len() {
            let res = Ok(self.items[self.pos].clone());
            self.pos += 1;
            return Some(res);
        }

        None
    }

    fn resync(&mut self) {
        self.pos = 0;
    }
}

unsafe impl<T> Send for Iterator<T> {}

unsafe extern "C" fn filter_trampoline<T>(value: gconstpointer, func: gconstpointer) -> i32
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    callback_guard!();
    let value = value as *const gobject_ffi::GValue;

    let func = func as *const gobject_ffi::GValue;
    let func = gobject_ffi::g_value_get_boxed(func);
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &&(Fn(T) -> bool + Send + Sync + 'static) = mem::transmute(func);

    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().unwrap();

    if func(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn filter_boxed_ref<T: 'static>(boxed: gpointer) -> gpointer {
    callback_guard!();

    let boxed = Arc::from_raw(boxed as *const (Box<Fn(T) -> bool + Send + Sync + 'static>));
    let copy = Arc::clone(&boxed);

    // Forget it and keep it alive, we will still need it later
    let _ = Arc::into_raw(boxed);

    Arc::into_raw(copy) as gpointer
}

unsafe extern "C" fn filter_boxed_unref<T: 'static>(boxed: gpointer) {
    callback_guard!();

    let _ = Arc::from_raw(boxed as *const (Box<Fn(T) -> bool + Send + Sync + 'static>));
}

unsafe extern "C" fn filter_boxed_get_type<T: StaticType + 'static>() -> glib_ffi::GType {
    use std::sync::{Once, ONCE_INIT};

    callback_guard!();

    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_name = {
            let mut idx = 0;

            loop {
                let type_name = CString::new(format!(
                    "GstRsIteratorFilterBoxed-{}-{}",
                    T::static_type().name(),
                    idx
                )).unwrap();
                if gobject_ffi::g_type_from_name(type_name.as_ptr()) == gobject_ffi::G_TYPE_INVALID
                {
                    break type_name;
                }
                idx += 1;
            }
        };

        TYPE = gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(filter_boxed_ref::<T>),
            Some(filter_boxed_unref::<T>),
        );
    });

    TYPE
}

unsafe extern "C" fn find_trampoline<T>(value: gconstpointer, func: gconstpointer) -> i32
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    callback_guard!();
    let value = value as *const gobject_ffi::GValue;

    let func = func as *const &mut (FnMut(T) -> bool);
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().unwrap();

    if (*func)(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn foreach_trampoline<T>(value: *const gobject_ffi::GValue, func: gpointer)
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    callback_guard!();
    let func = func as *const &mut (FnMut(T));
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().unwrap();

    (*func)(value);
}

unsafe extern "C" fn fold_trampoline<T, U>(
    value: *const gobject_ffi::GValue,
    ret: *mut gobject_ffi::GValue,
    func: gpointer,
) -> glib_ffi::gboolean
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    callback_guard!();
    let func = func as *const &mut (FnMut(U, T) -> Result<U, U>);
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().unwrap();

    let accum = &mut *(gobject_ffi::g_value_get_pointer(ret) as *mut Option<U>);

    match (*func)(accum.take().unwrap(), value) {
        Ok(next_accum) => {
            *accum = Some(next_accum);
            glib_ffi::GTRUE
        }
        Err(next_accum) => {
            *accum = Some(next_accum);
            glib_ffi::GFALSE
        }
    }
}

impl<T: StaticType + 'static> Clone for Iterator<T> {
    fn clone(&self) -> Self {
        unsafe { from_glib_full(ffi::gst_iterator_copy(self.to_glib_none().0)) }
    }
}

impl<T> Drop for Iterator<T> {
    fn drop(&mut self) {
        if !self.borrowed {
            unsafe {
                ffi::gst_iterator_free(self.iter.as_ptr());
            }
        }
    }
}

impl<T> glib::types::StaticType for Iterator<T> {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_iterator_get_type()) }
    }
}

#[doc(hidden)]
impl<'a, T: StaticType> glib::value::FromValueOptional<'a> for Iterator<T> {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<Iterator<T>>::from_glib_none(
            gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstIterator,
        )
    }
}

#[doc(hidden)]
impl<T: 'static> glib::value::SetValue for Iterator<T> {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstIterator>::to_glib_none(this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T: 'static> glib::value::SetValueOptional for Iterator<T> {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const ffi::GstIterator>::to_glib_none(&this).0
                as glib_ffi::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T> glib::translate::GlibPtrDefault for Iterator<T> {
    type GlibType = *mut ffi::GstIterator;
}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtr<'a, *const ffi::GstIterator> for Iterator<T> {
    type Storage = &'a Iterator<T>;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstIterator, Self> {
        glib::translate::Stash(self.iter.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *const ffi::GstIterator {
        unimplemented!()
    }
}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtrMut<'a, *mut ffi::GstIterator> for Iterator<T> {
    type Storage = &'a mut Iterator<T>;

    #[inline]
    fn to_glib_none_mut(
        &'a mut self,
    ) -> glib::translate::StashMut<'a, *mut ffi::GstIterator, Self> {
        glib::translate::StashMut(self.iter.as_ptr(), self)
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*const ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstIterator) -> Self {
        assert_ne!(
            gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_ffi::GFALSE
        );
        from_glib_full(ffi::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstIterator) -> Self {
        assert_ne!(
            gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_ffi::GFALSE
        );
        from_glib_full(ffi::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrBorrow<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstIterator) -> Self {
        assert!(!ptr.is_null());
        assert_ne!(
            gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_ffi::GFALSE
        );
        Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            borrowed: true,
            phantom: PhantomData,
        }
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrFull<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstIterator) -> Self {
        assert!(!ptr.is_null());
        assert_ne!(
            gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_ffi::GFALSE
        );
        Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            borrowed: false,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let val = it.next().unwrap();
        assert_eq!(val, Ok(1));
        let val = it.next().unwrap();
        assert_eq!(val, Ok(2));
        let val = it.next().unwrap();
        assert_eq!(val, Ok(3));
        assert!(it.next().is_none());

        let vec = vec![1i32, 2, 3];
        let it = Iterator::from_vec(vec);
        let vals: Vec<_> = it.map(|v| v.unwrap()).collect();
        assert_eq!(vals, [1, 2, 3]);

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let mut vals = Vec::new();
        while let Some(res) = it.next() {
            match res {
                Ok(v) => vals.push(v),
                _ => unreachable!(),
            }
        }
        assert_eq!(vals, [1, 2, 3]);

        let vec = vec![1i32, 2, 3];
        let it = Iterator::from_vec(vec);
        let mut vals = Vec::new();
        for v in it {
            vals.push(v.unwrap());
        }
        assert_eq!(vals, [1, 2, 3]);
    }

    #[test]
    fn test_filter() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let it = Iterator::from_vec(vec).filter(|val| val % 2 == 1);
        let vals: Vec<_> = it.map(|v| v.unwrap()).collect();
        assert_eq!(vals, [1, 3]);
    }

    #[test]
    fn test_find() {
        ::init().unwrap();

        // Our find
        let vec = vec![1i32, 2, 3];
        let val = Iterator::from_vec(vec).find_simple(|val| val == 2);
        assert_eq!(val.unwrap(), 2);

        // Find from std::iter::Iterator
        let vec = vec![1i32, 2, 3];
        let val = Iterator::from_vec(vec).find(|val| val.unwrap() == 2);
        assert_eq!(val.unwrap(), Ok(2));
    }

    #[test]
    fn test_foreach() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut sum = 0;
        let res = Iterator::from_vec(vec).foreach(|val| sum += val);
        assert_eq!(res, Ok(()));
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold() {
        ::init().unwrap();

        // Our fold
        let vec = vec![1i32, 2, 3];
        let res = Iterator::from_vec(vec).fold_with_early_exit(0, |mut sum, val| {
            sum += val;
            Ok(sum)
        });
        assert_eq!(res.unwrap(), 6);

        // Fold from std::iter::Iterator
        let vec = vec![1i32, 2, 3];
        let res = Iterator::from_vec(vec).fold(0, |mut sum, val| {
            sum += val.unwrap();
            sum
        });
        assert_eq!(res, 6);
    }
}
