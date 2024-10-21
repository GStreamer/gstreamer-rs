// Take a look at the license at the top of the repository in the LICENSE file.

use std::{any::Any, fmt, iter, marker::PhantomData, mem, ptr, sync::Arc};

use crate::ffi;
use glib::{
    ffi::{gconstpointer, gpointer},
    prelude::*,
    translate::*,
    value::{FromValue, ToValue},
    Value,
};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum IteratorError {
    #[error("Resync")]
    Resync,
    #[error("Error")]
    Error,
}

// Implemented manually so that we can use generics for the item
pub struct Iterator<T> {
    iter: ptr::NonNull<ffi::GstIterator>,
    phantom: PhantomData<T>,
}

impl<T> Iterator<T>
where
    for<'a> T: FromValue<'a> + 'static,
{
    #[allow(clippy::should_implement_trait)]
    #[doc(alias = "gst_iterator_next")]
    pub fn next(&mut self) -> Result<Option<T>, IteratorError> {
        unsafe {
            let mut value = Value::uninitialized();
            let res = ffi::gst_iterator_next(self.to_glib_none_mut().0, value.to_glib_none_mut().0);

            #[allow(clippy::wildcard_in_or_patterns)]
            match res {
                ffi::GST_ITERATOR_OK => match value.get::<T>() {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => Err(IteratorError::Error),
                },
                ffi::GST_ITERATOR_DONE => Ok(None),
                ffi::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                ffi::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }

    #[doc(alias = "gst_iterator_resync")]
    pub fn resync(&mut self) {
        unsafe {
            ffi::gst_iterator_resync(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gst_iterator_filter")]
    #[must_use]
    pub fn filter<F>(self, func: F) -> Self
    where
        F: Fn(T) -> bool + Send + Sync + 'static,
        T: StaticType,
    {
        unsafe {
            let func_box: Box<dyn Any + Send + Sync + 'static> = Box::new(func);
            let mut closure_value = glib::Value::from_type_unchecked(filter_boxed_get_type());
            glib::gobject_ffi::g_value_take_boxed(
                closure_value.to_glib_none_mut().0,
                Arc::into_raw(Arc::new(func_box)) as gpointer,
            );

            from_glib_full(ffi::gst_iterator_filter(
                self.into_glib_ptr(),
                Some(filter_trampoline::<T, F>),
                closure_value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_iterator_find_custom")]
    pub fn find<F>(&mut self, func: F) -> Option<T>
    where
        F: FnMut(T) -> bool,
    {
        unsafe {
            let mut elem = glib::Value::uninitialized();

            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let res = from_glib(ffi::gst_iterator_find_custom(
                self.to_glib_none_mut().0,
                Some(find_trampoline::<T, F>),
                elem.to_glib_none_mut().0,
                func_ptr,
            ));
            if res {
                Some(elem.get::<T>().expect("Iterator::find"))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_iterator_foreach")]
    pub fn foreach<F>(&mut self, func: F) -> Result<(), IteratorError>
    where
        F: FnMut(T),
    {
        unsafe {
            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let res = ffi::gst_iterator_foreach(
                self.to_glib_none_mut().0,
                Some(foreach_trampoline::<T, F>),
                func_ptr,
            );

            #[allow(clippy::wildcard_in_or_patterns)]
            match res {
                ffi::GST_ITERATOR_OK | ffi::GST_ITERATOR_DONE => Ok(()),
                ffi::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                ffi::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }

    #[doc(alias = "gst_iterator_fold")]
    pub fn fold<F, U>(&mut self, init: U, func: F) -> Result<U, IteratorError>
    where
        F: FnMut(U, T) -> Result<U, U>,
    {
        unsafe {
            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let mut accum = Some(init);
            let mut ret = glib::Value::from_type_unchecked(glib::Type::POINTER);
            glib::gobject_ffi::g_value_set_pointer(
                ret.to_glib_none_mut().0,
                &mut accum as *mut _ as gpointer,
            );

            let res = ffi::gst_iterator_fold(
                self.to_glib_none_mut().0,
                Some(fold_trampoline::<T, U, F>),
                ret.to_glib_none_mut().0,
                func_ptr,
            );

            #[allow(clippy::wildcard_in_or_patterns)]
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
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    #[doc(alias = "gst_iterator_new")]
    pub fn new<I: IteratorImpl<T>>(imp: I) -> Self {
        assert_initialized_main_thread!();
        static DUMMY_COOKIE: u32 = 0;

        unsafe {
            let it = ffi::gst_iterator_new(
                mem::size_of::<RsIterator<T, I>>() as u32,
                T::static_type().into_glib(),
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
                ptr::write(&mut (*it).imp, imp);
            }

            from_glib_full(it)
        }
    }
}

impl<T> Iterator<T>
where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Clone + Send + 'static,
{
    pub fn from_vec(items: Vec<T>) -> Self {
        skip_assert_initialized!();
        Self::new(ArrayIteratorImpl::new(items))
    }

    pub fn from_array<A: AsRef<[T]> + Send + Clone + 'static>(items: A) -> Self {
        skip_assert_initialized!();
        Self::new(ArrayIteratorImpl::new(items))
    }

    pub fn from_option(items: Option<T>) -> Self {
        skip_assert_initialized!();
        Self::new(OptionIteratorImpl::new(items))
    }

    pub fn from_single(item: T) -> Self {
        skip_assert_initialized!();
        Self::new(OptionIteratorImpl::new(Some(item)))
    }
}

impl<T: 'static> IntoGlibPtr<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut ffi::GstIterator {
        let s = mem::ManuallyDrop::new(self);
        let it = s.to_glib_none().0;
        it as *mut _
    }
}

#[repr(C)]
struct RsIterator<T, I: IteratorImpl<T>>
where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    iter: ffi::GstIterator,
    imp: I,
    phantom: PhantomData<T>,
}

pub trait IteratorImpl<T>: Clone + Send + 'static
where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>>;
    fn resync(&mut self);
}

unsafe extern "C" fn rs_iterator_copy<T, I: IteratorImpl<T>>(
    it: *const ffi::GstIterator,
    copy: *mut ffi::GstIterator,
) where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *const RsIterator<T, I>;
    let copy = copy as *mut RsIterator<T, I>;

    ptr::write(&mut (*copy).imp, (*it).imp.clone());
}

unsafe extern "C" fn rs_iterator_free<T, I: IteratorImpl<T>>(it: *mut ffi::GstIterator)
where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    ptr::drop_in_place(&mut (*it).imp);
}

unsafe extern "C" fn rs_iterator_next<T, I: IteratorImpl<T>>(
    it: *mut ffi::GstIterator,
    result: *mut glib::gobject_ffi::GValue,
) -> ffi::GstIteratorResult
where
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    match (*it).imp.next() {
        Some(Ok(value)) => {
            let value = value.to_value();
            ptr::write(result, value.into_raw());
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
    for<'a> T: FromValue<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    (*it).imp.resync();
}

#[derive(Clone)]
struct ArrayIteratorImpl<A, T> {
    pos: usize,
    items: A,
    phantom: PhantomData<T>,
}

impl<T, A> ArrayIteratorImpl<A, T> {
    fn new(items: A) -> Self {
        skip_assert_initialized!();
        Self {
            pos: 0,
            items,
            phantom: PhantomData,
        }
    }
}

impl<T, A> IteratorImpl<T> for ArrayIteratorImpl<A, T>
where
    A: AsRef<[T]> + Send + Clone + 'static,
    for<'a> T: StaticType + ToValue + FromValue<'a> + Clone + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>> {
        let items = self.items.as_ref();
        if self.pos < items.len() {
            let res = Ok(items[self.pos].clone());
            self.pos += 1;
            return Some(res);
        }

        None
    }

    fn resync(&mut self) {
        self.pos = 0;
    }
}

#[derive(Clone)]
struct OptionIteratorImpl<T> {
    finished: bool,
    items: Option<T>,
}

impl<T> OptionIteratorImpl<T> {
    fn new(items: Option<T>) -> Self {
        skip_assert_initialized!();
        Self {
            finished: false,
            items,
        }
    }
}

impl<T> IteratorImpl<T> for OptionIteratorImpl<T>
where
    for<'a> T: StaticType + ToValue + FromValue<'a> + Clone + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>> {
        if self.finished {
            return None;
        }
        let res = Ok(self.items.clone()).transpose();
        self.finished = true;
        res
    }

    fn resync(&mut self) {
        self.finished = false;
    }
}

unsafe impl<T> Send for Iterator<T> {}
unsafe impl<T> Sync for Iterator<T> {}

unsafe extern "C" fn filter_trampoline<
    T: for<'a> FromValue<'a> + StaticType + 'static,
    F: Fn(T) -> bool + Send + Sync + 'static,
>(
    value: gconstpointer,
    func: gconstpointer,
) -> i32 {
    let value = value as *const glib::gobject_ffi::GValue;

    let func = func as *const glib::gobject_ffi::GValue;
    let func = glib::gobject_ffi::g_value_get_boxed(func);
    let func = &*(func as *const &(dyn Any + Send + Sync + 'static));
    let func = func.downcast_ref::<F>().unwrap();

    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator filter_trampoline");

    if func(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn filter_boxed_ref(boxed: gpointer) -> gpointer {
    let boxed = Arc::from_raw(boxed as *const Box<dyn Any + Send + Sync + 'static>);
    let copy = Arc::clone(&boxed);

    // Forget it and keep it alive, we will still need it later
    let _ = Arc::into_raw(boxed);

    Arc::into_raw(copy) as gpointer
}

unsafe extern "C" fn filter_boxed_unref(boxed: gpointer) {
    let _ = Arc::from_raw(boxed as *const Box<dyn Any + Send + Sync + 'static>);
}

unsafe extern "C" fn filter_boxed_get_type() -> glib::Type {
    static TYPE: std::sync::OnceLock<glib::Type> = std::sync::OnceLock::new();

    *TYPE.get_or_init(|| {
        let iter_type_name = {
            let mut idx = 0;

            loop {
                let iter_type_name = glib::gformat!("GstRsIteratorFilterBoxed-{}", idx);
                if glib::gobject_ffi::g_type_from_name(iter_type_name.as_ptr())
                    == glib::gobject_ffi::G_TYPE_INVALID
                {
                    break iter_type_name;
                }
                idx += 1;
            }
        };

        let t = glib::Type::from_glib(glib::gobject_ffi::g_boxed_type_register_static(
            iter_type_name.as_ptr(),
            Some(filter_boxed_ref),
            Some(filter_boxed_unref),
        ));

        assert!(t.is_valid());

        t
    })
}

unsafe extern "C" fn find_trampoline<T, F: FnMut(T) -> bool>(
    value: gconstpointer,
    func: gconstpointer,
) -> i32
where
    for<'a> T: FromValue<'a> + 'static,
{
    let value = value as *const glib::gobject_ffi::GValue;

    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator find_trampoline");

    if (*func)(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn foreach_trampoline<T, F: FnMut(T)>(
    value: *const glib::gobject_ffi::GValue,
    func: gpointer,
) where
    for<'a> T: FromValue<'a> + 'static,
{
    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator foreach_trampoline");

    (*func)(value);
}

unsafe extern "C" fn fold_trampoline<T, U, F: FnMut(U, T) -> Result<U, U>>(
    value: *const glib::gobject_ffi::GValue,
    ret: *mut glib::gobject_ffi::GValue,
    func: gpointer,
) -> glib::ffi::gboolean
where
    for<'a> T: FromValue<'a> + 'static,
{
    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator fold_trampoline");

    let accum = &mut *(glib::gobject_ffi::g_value_get_pointer(ret) as *mut Option<U>);

    match (*func)(accum.take().unwrap(), value) {
        Ok(next_accum) => {
            *accum = Some(next_accum);
            glib::ffi::GTRUE
        }
        Err(next_accum) => {
            *accum = Some(next_accum);
            glib::ffi::GFALSE
        }
    }
}

impl<T: StaticType + 'static> Clone for Iterator<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { from_glib_full(ffi::gst_iterator_copy(self.to_glib_none().0)) }
    }
}

impl<T> fmt::Debug for Iterator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Iterator")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<T> Drop for Iterator<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_iterator_free(self.iter.as_ptr());
        }
    }
}

impl<T> iter::IntoIterator for Iterator<T>
where
    for<'a> T: FromValue<'a> + 'static,
{
    type Item = Result<T, IteratorError>;
    type IntoIter = StdIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> glib::types::StaticType for Iterator<T> {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_iterator_get_type()) }
    }
}

impl<T: StaticType + 'static> glib::value::ValueType for Iterator<T> {
    type Type = Self;
}

impl<T: StaticType + 'static> glib::value::ValueTypeOptional for Iterator<T> {}

unsafe impl<'a, T: StaticType + 'static> glib::value::FromValue<'a> for Iterator<T> {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstIterator
        )
    }
}

impl<T: StaticType + 'static> glib::value::ToValue for Iterator<T> {
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

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl<T: StaticType + 'static> glib::value::ToValueOptional for Iterator<T> {
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

impl<T: StaticType + 'static> From<Iterator<T>> for glib::Value {
    fn from(v: Iterator<T>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Iterator<T>>();
        unsafe {
            glib::gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                v.into_glib_ptr() as *mut _,
            )
        }
        value
    }
}

#[doc(hidden)]
impl<T> glib::translate::GlibPtrDefault for Iterator<T> {
    type GlibType = *mut ffi::GstIterator;
}

#[doc(hidden)]
unsafe impl<T: StaticType + 'static> TransparentPtrType for Iterator<T> {}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtr<'a, *const ffi::GstIterator> for Iterator<T> {
    type Storage = PhantomData<&'a Iterator<T>>;

    #[inline]
    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstIterator, Self> {
        glib::translate::Stash(self.iter.as_ptr(), PhantomData)
    }

    fn to_glib_full(&self) -> *const ffi::GstIterator {
        unimplemented!()
    }
}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtrMut<'a, *mut ffi::GstIterator> for Iterator<T> {
    type Storage = PhantomData<&'a mut Iterator<T>>;

    #[inline]
    fn to_glib_none_mut(
        &'a mut self,
    ) -> glib::translate::StashMut<'a, *mut ffi::GstIterator, Self> {
        glib::translate::StashMut(self.iter.as_ptr(), PhantomData)
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*const ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstIterator) -> Self {
        debug_assert_ne!(
            glib::gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().into_glib()),
            glib::ffi::GFALSE
        );
        from_glib_full(ffi::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstIterator) -> Self {
        debug_assert_ne!(
            glib::gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().into_glib()),
            glib::ffi::GFALSE
        );
        from_glib_full(ffi::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrBorrow<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstIterator) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());
        debug_assert_ne!(
            glib::gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().into_glib()),
            glib::ffi::GFALSE
        );
        Borrowed::new(Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        })
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrFull<*mut ffi::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstIterator) -> Self {
        debug_assert!(!ptr.is_null());
        debug_assert_ne!(
            glib::gobject_ffi::g_type_is_a((*ptr).type_, T::static_type().into_glib()),
            glib::ffi::GFALSE
        );
        Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }
}

pub struct StdIterator<T> {
    inner: Iterator<T>,
    error: Option<IteratorError>,
}

impl<T> StdIterator<T> {
    fn new(inner: Iterator<T>) -> Self {
        skip_assert_initialized!();
        Self { inner, error: None }
    }
}

impl<T: StaticType + 'static> Clone for StdIterator<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            error: self.error,
        }
    }
}

impl<T> fmt::Debug for StdIterator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StdIterator")
            .field("inner", &self.inner)
            .field("error", &self.error)
            .finish()
    }
}

impl<T> iter::Iterator for StdIterator<T>
where
    for<'a> T: FromValue<'a> + 'static,
{
    type Item = Result<T, IteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.error {
            // Fuse the iterator after returning IteratorError::Error
            Some(IteratorError::Error) => return None,

            // The iterator needs a resync
            Some(IteratorError::Resync) => self.inner.resync(),

            None => {}
        }

        let res = self.inner.next();
        self.error = res.as_ref().err().copied();
        res.transpose()
    }
}

impl<T> iter::FusedIterator for StdIterator<T> where for<'a> T: FromValue<'a> + 'static {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        crate::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let val = it.next();
        assert_eq!(val, Ok(Some(1)));
        let val = it.next();
        assert_eq!(val, Ok(Some(2)));
        let val = it.next();
        assert_eq!(val, Ok(Some(3)));
        assert_eq!(it.next(), Ok(None));

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let mut vals = Vec::new();
        while let Ok(Some(res)) = it.next() {
            vals.push(res);
        }
        assert_eq!(vals, [1, 2, 3]);
    }

    #[test]
    fn test_filter() {
        crate::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec).filter(|val| val % 2 == 1);

        let mut vals = Vec::new();
        while let Ok(Some(res)) = it.next() {
            vals.push(res);
        }
        assert_eq!(vals, [1, 3]);
    }

    #[test]
    fn test_find() {
        crate::init().unwrap();

        // Our find
        let vec = vec![1i32, 2, 3];
        let val = Iterator::from_vec(vec).find(|val| val == 2);
        assert_eq!(val.unwrap(), 2);
    }

    #[test]
    fn test_foreach() {
        crate::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut sum = 0;
        let res = Iterator::from_vec(vec).foreach(|val| sum += val);
        assert_eq!(res, Ok(()));
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold() {
        crate::init().unwrap();

        // Our fold
        let vec = vec![1i32, 2, 3];
        let res = Iterator::from_vec(vec).fold(0, |mut sum, val| {
            sum += val;
            Ok(sum)
        });
        assert_eq!(res.unwrap(), 6);
    }

    #[test]
    fn test_std() {
        crate::init().unwrap();

        let mut it = Iterator::from_vec(vec![1i32, 2, 3]).into_iter();
        assert_eq!(it.next(), Some(Ok(1)));
        assert_eq!(it.next(), Some(Ok(2)));
        assert_eq!(it.next(), Some(Ok(3)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_into_iter() {
        crate::init().unwrap();

        let mut v = vec![1i32, 2, 3].into_iter();
        for x in Iterator::from_vec(vec![1i32, 2, 3]) {
            assert_eq!(x.unwrap(), v.next().unwrap());
        }
        assert_eq!(v.next(), None);
    }

    #[test]
    fn test_std_resync_collect() {
        use std::collections::BTreeSet;

        use crate::prelude::*;

        crate::init().unwrap();

        let bin = crate::Bin::new();
        let id1 = crate::ElementFactory::make("identity").build().unwrap();
        let id2 = crate::ElementFactory::make("identity").build().unwrap();

        bin.add(&id1).unwrap();

        let mut it = bin.iterate_elements().into_iter();
        assert_eq!(it.next().unwrap().unwrap(), id1);

        bin.add(&id2).unwrap();

        let res = it.by_ref().collect::<Result<Vec<_>, _>>().unwrap_err();
        assert_eq!(res, IteratorError::Resync);

        let mut elems = BTreeSet::new();
        elems.insert(id1);
        elems.insert(id2);

        let res = it.by_ref().collect::<Result<BTreeSet<_>, _>>().unwrap();
        assert_eq!(res, elems);

        let res = it.collect::<Result<Vec<_>, _>>().unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_std_resync_find() {
        use crate::prelude::*;

        crate::init().unwrap();

        let bin = crate::Bin::new();
        let id1 = crate::ElementFactory::make("identity").build().unwrap();
        let id2 = crate::ElementFactory::make("identity").build().unwrap();

        bin.add(&id1).unwrap();

        let mut it = bin.iterate_elements().into_iter();
        assert_eq!(it.next().unwrap().unwrap(), id1);

        bin.add(&id2).unwrap();

        let res = it.find(|x| x.as_ref() == Ok(&id1));
        assert_eq!(res.unwrap().unwrap(), id1);
    }
}
