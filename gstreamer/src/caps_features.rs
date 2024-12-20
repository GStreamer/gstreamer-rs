// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, BorrowMut, ToOwned},
    fmt,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr, str,
};

use crate::ffi;
use glib::{prelude::*, translate::*};
use once_cell::sync::Lazy;

#[doc(alias = "GstCapsFeatures")]
#[repr(transparent)]
pub struct CapsFeatures(ptr::NonNull<ffi::GstCapsFeatures>);
unsafe impl Send for CapsFeatures {}
unsafe impl Sync for CapsFeatures {}

impl CapsFeatures {
    #[doc(alias = "gst_caps_features_new")]
    pub fn new(features: impl IntoIterator<Item = impl IntoGStr>) -> Self {
        skip_assert_initialized!();
        let mut f = Self::new_empty();

        for feature in features {
            f.add(feature);
        }

        f
    }

    #[doc(alias = "gst_caps_features_new_id")]
    pub fn from_quarks(features: impl IntoIterator<Item = glib::Quark>) -> Self {
        skip_assert_initialized!();
        let mut f = Self::new_empty();

        for feature in features.into_iter() {
            f.add_from_quark(feature);
        }

        f
    }

    #[doc(alias = "gst_caps_features_new_empty")]
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe {
            CapsFeatures(ptr::NonNull::new_unchecked(
                ffi::gst_caps_features_new_empty(),
            ))
        }
    }

    #[doc(alias = "gst_caps_features_new_any")]
    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { CapsFeatures(ptr::NonNull::new_unchecked(ffi::gst_caps_features_new_any())) }
    }
}

impl IntoGlibPtr<*mut ffi::GstCapsFeatures> for CapsFeatures {
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut ffi::GstCapsFeatures {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr()
    }
}

impl Deref for CapsFeatures {
    type Target = CapsFeaturesRef;

    #[inline]
    fn deref(&self) -> &CapsFeaturesRef {
        unsafe { &*(self.0.as_ref() as *const ffi::GstCapsFeatures as *const CapsFeaturesRef) }
    }
}

impl DerefMut for CapsFeatures {
    #[inline]
    fn deref_mut(&mut self) -> &mut CapsFeaturesRef {
        unsafe { &mut *(self.0.as_mut() as *mut ffi::GstCapsFeatures as *mut CapsFeaturesRef) }
    }
}

impl AsRef<CapsFeaturesRef> for CapsFeatures {
    #[inline]
    fn as_ref(&self) -> &CapsFeaturesRef {
        self.deref()
    }
}

impl AsMut<CapsFeaturesRef> for CapsFeatures {
    #[inline]
    fn as_mut(&mut self) -> &mut CapsFeaturesRef {
        self.deref_mut()
    }
}

impl Clone for CapsFeatures {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::gst_caps_features_copy(self.0.as_ref());
            debug_assert!(!ptr.is_null());
            CapsFeatures(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for CapsFeatures {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::gst_caps_features_free(self.0.as_mut()) }
    }
}

impl fmt::Debug for CapsFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CapsFeatures")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for CapsFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to make sure to not call ToString::to_string() here, which
        // we have because of the Display impl. We need CapsFeaturesRef::to_string()
        f.write_str(&CapsFeaturesRef::to_string(self.as_ref()))
    }
}

impl str::FromStr for CapsFeatures {
    type Err = glib::BoolError;

    #[doc(alias = "gst_caps_features_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = s.run_with_gstr(|s| ffi::gst_caps_features_from_string(s.as_ptr()));
            if ptr.is_null() {
                return Err(glib::bool_error!(
                    "Failed to parse caps features from string"
                ));
            }

            Ok(Self(ptr::NonNull::new_unchecked(ptr)))
        }
    }
}

impl Borrow<CapsFeaturesRef> for CapsFeatures {
    #[inline]
    fn borrow(&self) -> &CapsFeaturesRef {
        self.as_ref()
    }
}

impl BorrowMut<CapsFeaturesRef> for CapsFeatures {
    #[inline]
    fn borrow_mut(&mut self) -> &mut CapsFeaturesRef {
        self.as_mut()
    }
}

impl glib::types::StaticType for CapsFeatures {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_caps_features_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstCapsFeatures, Self> {
        unsafe { Stash(self.0.as_ref(), PhantomData) }
    }

    #[inline]
    fn to_glib_full(&self) -> *const ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstCapsFeatures, Self> {
        unsafe {
            Stash(
                self.0.as_ref() as *const ffi::GstCapsFeatures as *mut ffi::GstCapsFeatures,
                PhantomData,
            )
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GstCapsFeatures {
        unsafe { ffi::gst_caps_features_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstCapsFeatures> for CapsFeatures {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GstCapsFeatures, Self> {
        unsafe { StashMut(self.0.as_mut(), PhantomData) }
    }
}

impl FromGlibPtrNone<*const ffi::GstCapsFeatures> for CapsFeatures {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstCapsFeatures) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        debug_assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrNone<*mut ffi::GstCapsFeatures> for CapsFeatures {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstCapsFeatures) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = ffi::gst_caps_features_copy(ptr);
        debug_assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrFull<*const ffi::GstCapsFeatures> for CapsFeatures {
    #[inline]
    unsafe fn from_glib_full(ptr: *const ffi::GstCapsFeatures) -> Self {
        debug_assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(
            ptr as *mut ffi::GstCapsFeatures,
        ))
    }
}

impl FromGlibPtrFull<*mut ffi::GstCapsFeatures> for CapsFeatures {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstCapsFeatures) -> Self {
        debug_assert!(!ptr.is_null());
        CapsFeatures(ptr::NonNull::new_unchecked(ptr))
    }
}

impl glib::value::ValueType for CapsFeatures {
    type Type = Self;
}

impl glib::value::ValueTypeOptional for CapsFeatures {}

unsafe impl<'a> glib::value::FromValue<'a> for CapsFeatures {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstCapsFeatures)
    }
}

impl glib::value::ToValue for CapsFeatures {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut ffi::GstCapsFeatures>::to_glib_none(self).0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for CapsFeatures {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                ToGlibPtr::<*mut ffi::GstCapsFeatures>::to_glib_none(&s).0 as *mut _,
            )
        }
        value
    }
}

impl From<CapsFeatures> for glib::Value {
    fn from(v: CapsFeatures) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<CapsFeatures>();
        unsafe {
            glib::gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                IntoGlibPtr::<*mut ffi::GstCapsFeatures>::into_glib_ptr(v) as *mut _,
            )
        }
        value
    }
}

impl GlibPtrDefault for CapsFeatures {
    type GlibType = *mut ffi::GstCapsFeatures;
}

unsafe impl TransparentPtrType for CapsFeatures {}

#[repr(transparent)]
#[doc(alias = "GstCapsFeatures")]
pub struct CapsFeaturesRef(ffi::GstCapsFeatures);

impl CapsFeaturesRef {
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstCapsFeatures) -> &'a CapsFeaturesRef {
        debug_assert!(!ptr.is_null());

        &*(ptr as *mut CapsFeaturesRef)
    }

    #[inline]
    pub unsafe fn from_glib_borrow_mut<'a>(
        ptr: *mut ffi::GstCapsFeatures,
    ) -> &'a mut CapsFeaturesRef {
        debug_assert!(!ptr.is_null());

        &mut *(ptr as *mut CapsFeaturesRef)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstCapsFeatures {
        self as *const Self as *const ffi::GstCapsFeatures
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstCapsFeatures {
        self as *const Self as *mut ffi::GstCapsFeatures
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0 && !self.is_any()
    }

    #[doc(alias = "gst_caps_features_is_any")]
    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_features_is_any(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_features_contains")]
    pub fn contains(&self, feature: impl IntoGStr) -> bool {
        unsafe {
            feature.run_with_gstr(|feature| {
                from_glib(ffi::gst_caps_features_contains(
                    self.as_ptr(),
                    feature.as_ptr(),
                ))
            })
        }
    }

    #[doc(alias = "gst_caps_features_contains_id")]
    pub fn contains_quark(&self, feature: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_contains_id(
                self.as_ptr(),
                feature.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_caps_features_get_size")]
    pub fn size(&self) -> usize {
        unsafe { ffi::gst_caps_features_get_size(self.as_ptr()) as usize }
    }

    #[doc(alias = "get_nth")]
    #[doc(alias = "gst_caps_features_get_nth")]
    pub fn nth(&self, idx: usize) -> Option<&glib::GStr> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let feature = ffi::gst_caps_features_get_nth(self.as_ptr(), idx as u32);
            if feature.is_null() {
                return None;
            }

            // Safety: we can return a GStr based on the feature here because
            // the lifetime of the returned value is constrained by &self.
            Some(glib::GStr::from_ptr(feature))
        }
    }

    #[doc(alias = "gst_caps_features_get_nth_id")]
    pub fn nth_quark(&self, idx: usize) -> Option<glib::Quark> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let feature = ffi::gst_caps_features_get_nth_id(self.as_ptr(), idx as u32);
            Some(from_glib(feature))
        }
    }

    #[doc(alias = "gst_caps_features_add")]
    pub fn add(&mut self, feature: impl IntoGStr) {
        unsafe {
            feature.run_with_gstr(|feature| {
                ffi::gst_caps_features_add(self.as_mut_ptr(), feature.as_ptr())
            })
        }
    }

    #[doc(alias = "gst_caps_features_remove")]
    pub fn remove(&mut self, feature: impl IntoGStr) {
        unsafe {
            feature.run_with_gstr(|feature| {
                ffi::gst_caps_features_remove(self.as_mut_ptr(), feature.as_ptr())
            })
        }
    }

    #[doc(alias = "gst_caps_features_add_id")]
    pub fn add_from_quark(&mut self, feature: glib::Quark) {
        unsafe { ffi::gst_caps_features_add_id(self.as_mut_ptr(), feature.into_glib()) }
    }

    #[doc(alias = "gst_caps_features_remove_id")]
    pub fn remove_by_quark(&mut self, feature: glib::Quark) {
        unsafe { ffi::gst_caps_features_remove_id(self.as_mut_ptr(), feature.into_glib()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    // This is not an equivalence relation with regards to ANY. Everything is equal to ANY
    #[doc(alias = "gst_caps_features_is_equal")]
    pub fn is_equal(&self, other: &CapsFeaturesRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_features_is_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }
}

impl glib::types::StaticType for CapsFeaturesRef {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

impl<'a> std::iter::Extend<&'a str> for CapsFeaturesRef {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        iter.into_iter().for_each(|f| self.add(f));
    }
}

impl<'a> std::iter::Extend<&'a glib::GStr> for CapsFeaturesRef {
    fn extend<T: IntoIterator<Item = &'a glib::GStr>>(&mut self, iter: T) {
        iter.into_iter().for_each(|f| self.add(f));
    }
}

impl std::iter::Extend<String> for CapsFeaturesRef {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        iter.into_iter().for_each(|f| self.add(&f));
    }
}

impl std::iter::Extend<glib::GString> for CapsFeaturesRef {
    fn extend<T: IntoIterator<Item = glib::GString>>(&mut self, iter: T) {
        iter.into_iter().for_each(|f| self.add(&f));
    }
}

impl std::iter::Extend<glib::Quark> for CapsFeaturesRef {
    fn extend<T: IntoIterator<Item = glib::Quark>>(&mut self, iter: T) {
        iter.into_iter().for_each(|f| self.add_from_quark(f));
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a CapsFeaturesRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const CapsFeaturesRef)
    }
}

impl glib::value::ToValue for CapsFeaturesRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<CapsFeatures>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_mut_ptr() as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for CapsFeaturesRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<CapsFeatures>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| s.as_mut_ptr()).unwrap_or(ptr::null_mut()) as *mut _,
            )
        }
        value
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    caps_features: &'a CapsFeaturesRef,
    idx: usize,
    n_features: usize,
}

impl<'a> Iter<'a> {
    fn new(caps_features: &'a CapsFeaturesRef) -> Iter<'a> {
        skip_assert_initialized!();
        let n_features = caps_features.size();

        Iter {
            caps_features,
            idx: 0,
            n_features,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a glib::GStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n_features {
            return None;
        }

        unsafe {
            let feature =
                ffi::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.idx as u32);
            debug_assert!(!feature.is_null());

            self.idx += 1;

            // Safety: we can return a GStr based on the feature here because the lifetime
            // of the returned Item is constrained by the underlying CapsFeatureRef.
            Some(glib::GStr::from_ptr(feature))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.n_features - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.n_features - self.idx
    }

    // checker-ignore-item
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.n_features || overflow {
            self.idx = self.n_features;
            None
        } else {
            unsafe {
                self.idx = end + 1;
                let feature =
                    ffi::gst_caps_features_get_nth(self.caps_features.as_ptr(), end as u32);
                debug_assert!(!feature.is_null());

                // Safety: we can return a GStr based on the feature here because the lifetime
                // of the returned Item is constrained by the underlying CapsFeatureRef.
                Some(glib::GStr::from_ptr(feature))
            }
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.n_features {
            None
        } else {
            unsafe {
                let feature = ffi::gst_caps_features_get_nth(
                    self.caps_features.as_ptr(),
                    self.n_features as u32 - 1,
                );
                debug_assert!(!feature.is_null());

                // Safety: we can return a GStr based on the feature here because the lifetime
                // of the returned Item is constrained by the underlying CapsFeatureRef.
                Some(glib::GStr::from_ptr(feature))
            }
        }
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.n_features {
            return None;
        }

        self.n_features -= 1;

        unsafe {
            let feature =
                ffi::gst_caps_features_get_nth(self.caps_features.as_ptr(), self.n_features as u32);
            debug_assert!(!feature.is_null());

            // Safety: we can return a GStr based on the feature here because the lifetime
            // of the returned Item is constrained by the underlying CapsFeatureRef.
            Some(glib::GStr::from_ptr(feature))
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.n_features.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.n_features;
            None
        } else {
            unsafe {
                self.n_features = end - 1;
                let feature = ffi::gst_caps_features_get_nth(
                    self.caps_features.as_ptr(),
                    self.n_features as u32,
                );
                debug_assert!(!feature.is_null());

                // Safety: we can return a GStr based on the feature here because the lifetime
                // of the returned Item is constrained by the underlying CapsFeatureRef.
                Some(glib::GStr::from_ptr(feature))
            }
        }
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl std::iter::FusedIterator for Iter<'_> {}

impl<'a> IntoIterator for &'a CapsFeaturesRef {
    type IntoIter = Iter<'a>;
    type Item = &'a glib::GStr;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> From<&'a str> for CapsFeatures {
    fn from(value: &'a str) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        features.add(value);

        features
    }
}

impl<'a> From<&'a glib::GStr> for CapsFeatures {
    fn from(value: &'a glib::GStr) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        features.add(value);

        features
    }
}

impl From<glib::Quark> for CapsFeatures {
    fn from(value: glib::Quark) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        features.add_from_quark(value);

        features
    }
}

impl<'a, const N: usize> From<[&'a str; N]> for CapsFeatures {
    fn from(value: [&'a str; N]) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        value.into_iter().for_each(|f| features.add(f));

        features
    }
}

impl<'a, const N: usize> From<[&'a glib::GStr; N]> for CapsFeatures {
    fn from(value: [&'a glib::GStr; N]) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        value.into_iter().for_each(|f| features.add(f));

        features
    }
}

impl<const N: usize> From<[String; N]> for CapsFeatures {
    fn from(value: [String; N]) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        value.into_iter().for_each(|f| features.add(&f));

        features
    }
}

impl<const N: usize> From<[glib::GString; N]> for CapsFeatures {
    fn from(value: [glib::GString; N]) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        value.into_iter().for_each(|f| features.add(&f));

        features
    }
}

impl<const N: usize> From<[glib::Quark; N]> for CapsFeatures {
    fn from(value: [glib::Quark; N]) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        value.into_iter().for_each(|f| features.add_from_quark(f));

        features
    }
}

impl<'a> std::iter::FromIterator<&'a str> for CapsFeatures {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        iter.into_iter().for_each(|f| features.add(f));

        features
    }
}

impl<'a> std::iter::FromIterator<&'a glib::GStr> for CapsFeatures {
    fn from_iter<T: IntoIterator<Item = &'a glib::GStr>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        let mut features = CapsFeatures::new_empty();

        iter.into_iter().for_each(|f| features.add(f));

        features
    }
}

impl std::iter::FromIterator<String> for CapsFeatures {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        iter.into_iter().for_each(|f| features.add(&f));

        features
    }
}

impl std::iter::FromIterator<glib::GString> for CapsFeatures {
    fn from_iter<T: IntoIterator<Item = glib::GString>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        let mut features = CapsFeatures::new_empty();

        iter.into_iter().for_each(|f| features.add(&f));

        features
    }
}

impl std::iter::FromIterator<glib::Quark> for CapsFeatures {
    fn from_iter<T: IntoIterator<Item = glib::Quark>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut features = CapsFeatures::new_empty();

        iter.into_iter().for_each(|f| features.add_from_quark(f));

        features
    }
}

impl fmt::Debug for CapsFeaturesRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CapsFeatures")
            .field(&self.to_string())
            .finish()
    }
}

impl fmt::Display for CapsFeaturesRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_caps_features_to_string(self.as_ptr()))
        };
        f.write_str(&s)
    }
}

impl ToOwned for CapsFeaturesRef {
    type Owned = CapsFeatures;

    #[inline]
    fn to_owned(&self) -> CapsFeatures {
        unsafe { from_glib_full(ffi::gst_caps_features_copy(self.as_ptr() as *const _) as *mut _) }
    }
}

unsafe impl Sync for CapsFeaturesRef {}
unsafe impl Send for CapsFeaturesRef {}

pub static CAPS_FEATURE_MEMORY_SYSTEM_MEMORY: &glib::GStr =
    unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_CAPS_FEATURE_MEMORY_SYSTEM_MEMORY) };
pub static CAPS_FEATURES_MEMORY_SYSTEM_MEMORY: Lazy<CapsFeatures> =
    Lazy::new(|| CapsFeatures::new([CAPS_FEATURE_MEMORY_SYSTEM_MEMORY]));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_value_optional() {
        use glib::value::ToValue;

        crate::init().unwrap();

        let a = None::<CapsFeatures>.to_value();
        assert!(a.get::<Option<CapsFeatures>>().unwrap().is_none());
        let b = glib::value::Value::from(&CapsFeatures::new_empty());
        assert!(b.get::<Option<CapsFeatures>>().unwrap().is_some());
    }
}
