// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, BorrowMut, ToOwned},
    fmt,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr, str,
};

use glib::{
    prelude::*,
    translate::*,
    value::{FromValue, SendValue, Value},
    IntoGStr,
};

use crate::{ffi, Fraction};

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum GetError<E: std::error::Error> {
    #[error("GetError: Structure field with name {name} not found")]
    FieldNotFound { name: &'static str },
    #[error("GetError: Structure field with name {name} not retrieved")]
    ValueGetError {
        name: &'static str,
        #[source]
        error: E,
    },
}

impl<E: std::error::Error> GetError<E> {
    fn new_field_not_found(name: &'static str) -> Self {
        skip_assert_initialized!();
        GetError::FieldNotFound { name }
    }

    fn from_value_get_error(name: &'static str, error: E) -> Self {
        skip_assert_initialized!();
        GetError::ValueGetError { name, error }
    }
}

#[doc(alias = "GstStructure")]
#[repr(transparent)]
pub struct Structure(ptr::NonNull<ffi::GstStructure>);
unsafe impl Send for Structure {}
unsafe impl Sync for Structure {}

impl Structure {
    #[doc(alias = "gst_structure_new")]
    pub fn builder(name: impl IntoGStr) -> Builder {
        skip_assert_initialized!();
        Builder::new(name)
    }

    #[doc(alias = "gst_structure_new_empty")]
    pub fn new_empty(name: impl IntoGStr) -> Structure {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = name.run_with_gstr(|name| ffi::gst_structure_new_empty(name.as_ptr()));
            debug_assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(
        name: impl IntoGStr,
        iter: impl IntoIterator<Item = (impl IntoGStr, SendValue)>,
    ) -> Structure {
        skip_assert_initialized!();
        let mut structure = Structure::new_empty(name);

        iter.into_iter()
            .for_each(|(f, v)| structure.set_value(f, v));

        structure
    }
}

impl IntoGlibPtr<*mut ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut ffi::GstStructure {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr()
    }
}

impl Deref for Structure {
    type Target = StructureRef;

    #[inline]
    fn deref(&self) -> &StructureRef {
        unsafe { &*(self.0.as_ptr() as *const StructureRef) }
    }
}

impl DerefMut for Structure {
    #[inline]
    fn deref_mut(&mut self) -> &mut StructureRef {
        unsafe { &mut *(self.0.as_ptr() as *mut StructureRef) }
    }
}

impl AsRef<StructureRef> for Structure {
    #[inline]
    fn as_ref(&self) -> &StructureRef {
        self.deref()
    }
}

impl AsMut<StructureRef> for Structure {
    #[inline]
    fn as_mut(&mut self) -> &mut StructureRef {
        self.deref_mut()
    }
}

impl Clone for Structure {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::gst_structure_copy(self.0.as_ref());
            debug_assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for Structure {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::gst_structure_free(self.0.as_mut()) }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Structure").field(self.as_ref()).finish()
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to make sure to not call ToString::to_string() here, which
        // we have because of the Display impl. We need StructureRef::to_string()
        f.write_str(&StructureRef::to_string(self.as_ref()))
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Structure) -> bool {
        StructureRef::eq(self, other)
    }
}

impl Eq for Structure {}

impl PartialEq<StructureRef> for Structure {
    fn eq(&self, other: &StructureRef) -> bool {
        StructureRef::eq(self, other)
    }
}

impl PartialEq<Structure> for StructureRef {
    fn eq(&self, other: &Structure) -> bool {
        StructureRef::eq(other, self)
    }
}

impl str::FromStr for Structure {
    type Err = glib::BoolError;

    #[doc(alias = "gst_structure_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            let structure =
                s.run_with_gstr(|s| ffi::gst_structure_from_string(s.as_ptr(), ptr::null_mut()));
            if structure.is_null() {
                Err(glib::bool_error!("Failed to parse structure from string"))
            } else {
                Ok(Self(ptr::NonNull::new_unchecked(structure)))
            }
        }
    }
}

impl Borrow<StructureRef> for Structure {
    #[inline]
    fn borrow(&self) -> &StructureRef {
        self.as_ref()
    }
}

impl BorrowMut<StructureRef> for Structure {
    #[inline]
    fn borrow_mut(&mut self) -> &mut StructureRef {
        self.as_mut()
    }
}

impl ToOwned for StructureRef {
    type Owned = Structure;

    fn to_owned(&self) -> Structure {
        unsafe {
            let ptr = ffi::gst_structure_copy(&self.0);
            debug_assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl glib::types::StaticType for Structure {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstStructure> for Structure {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstStructure, Self> {
        unsafe { Stash(self.0.as_ref(), PhantomData) }
    }

    #[inline]
    fn to_glib_full(&self) -> *const ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstStructure> for Structure {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstStructure, Self> {
        unsafe {
            Stash(
                self.0.as_ref() as *const ffi::GstStructure as *mut ffi::GstStructure,
                PhantomData,
            )
        }
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstStructure> for Structure {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GstStructure, Self> {
        unsafe { StashMut(self.0.as_mut(), PhantomData) }
    }
}

impl FromGlibPtrNone<*const ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstStructure) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        debug_assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrNone<*mut ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstStructure) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        debug_assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrFull<*const ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_full(ptr: *const ffi::GstStructure) -> Self {
        debug_assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut ffi::GstStructure))
    }
}

impl FromGlibPtrFull<*mut ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstStructure) -> Self {
        debug_assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrBorrow<*const ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const ffi::GstStructure) -> Borrowed<Self> {
        Borrowed::new(from_glib_full(ptr))
    }
}

impl FromGlibPtrBorrow<*mut ffi::GstStructure> for Structure {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStructure) -> Borrowed<Self> {
        Borrowed::new(from_glib_full(ptr))
    }
}

impl glib::value::ValueType for Structure {
    type Type = Self;
}

impl glib::value::ValueTypeOptional for Structure {}

unsafe impl<'a> glib::value::FromValue<'a> for Structure {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstStructure
        )
    }
}

impl glib::value::ToValue for Structure {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                glib::translate::ToGlibPtr::<*const ffi::GstStructure>::to_glib_none(self).0
                    as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for Structure {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                glib::translate::ToGlibPtr::<*const ffi::GstStructure>::to_glib_none(&s).0
                    as *mut _,
            )
        }
        value
    }
}

impl From<Structure> for glib::Value {
    fn from(v: Structure) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Structure>();
        unsafe {
            glib::gobject_ffi::g_value_take_boxed(
                value.to_glib_none_mut().0,
                glib::translate::IntoGlibPtr::<*mut ffi::GstStructure>::into_glib_ptr(v) as *mut _,
            )
        }
        value
    }
}

impl GlibPtrDefault for Structure {
    type GlibType = *mut ffi::GstStructure;
}

unsafe impl TransparentPtrType for Structure {}

#[repr(transparent)]
#[doc(alias = "GstStructure")]
pub struct StructureRef(ffi::GstStructure);

unsafe impl Send for StructureRef {}
unsafe impl Sync for StructureRef {}

impl StructureRef {
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstStructure) -> &'a StructureRef {
        debug_assert!(!ptr.is_null());

        &*(ptr as *mut StructureRef)
    }

    #[inline]
    pub unsafe fn from_glib_borrow_mut<'a>(ptr: *mut ffi::GstStructure) -> &'a mut StructureRef {
        debug_assert!(!ptr.is_null());

        &mut *(ptr as *mut StructureRef)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstStructure {
        self as *const Self as *const ffi::GstStructure
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstStructure {
        self as *const Self as *mut ffi::GstStructure
    }

    #[doc(alias = "gst_structure_get")]
    pub fn get<'a, T: FromValue<'a>>(
        &'a self,
        name: impl IntoGStr,
    ) -> Result<T, GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>>
    {
        let name = glib::Quark::from_str(name);
        self.get_by_quark(name)
    }

    #[doc(alias = "gst_structure_get")]
    pub fn get_optional<'a, T: FromValue<'a>>(
        &'a self,
        name: impl IntoGStr,
    ) -> Result<
        Option<T>,
        GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>,
    > {
        let name = glib::Quark::from_str(name);
        self.get_optional_by_quark(name)
    }

    #[doc(alias = "get_value")]
    #[doc(alias = "gst_structure_get_value")]
    pub fn value(
        &self,
        name: impl IntoGStr,
    ) -> Result<&SendValue, GetError<std::convert::Infallible>> {
        let name = glib::Quark::from_str(name);
        self.value_by_quark(name)
    }

    #[doc(alias = "gst_structure_id_get")]
    pub fn get_by_quark<'a, T: FromValue<'a>>(
        &'a self,
        name: glib::Quark,
    ) -> Result<T, GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>>
    {
        self.value_by_quark(name)
            .map_err(|err| match err {
                GetError::FieldNotFound { name } => GetError::FieldNotFound { name },
                _ => unreachable!(),
            })?
            .get()
            .map_err(|err| GetError::from_value_get_error(name.as_str(), err))
    }

    #[doc(alias = "gst_structure_id_get")]
    pub fn get_optional_by_quark<'a, T: FromValue<'a>>(
        &'a self,
        name: glib::Quark,
    ) -> Result<
        Option<T>,
        GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>,
    > {
        self.value_by_quark(name)
            .ok()
            .map(|v| v.get())
            .transpose()
            .map_err(|err| GetError::from_value_get_error(name.as_str(), err))
    }

    #[doc(alias = "gst_structure_id_get_value")]
    pub fn value_by_quark(
        &self,
        name: glib::Quark,
    ) -> Result<&SendValue, GetError<std::convert::Infallible>> {
        unsafe {
            let value = ffi::gst_structure_id_get_value(&self.0, name.into_glib());

            if value.is_null() {
                return Err(GetError::new_field_not_found(name.as_str()));
            }

            Ok(&*(value as *const SendValue))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[doc(alias = "gst_structure_set")]
    pub fn set(&mut self, name: impl IntoGStr, value: impl Into<glib::Value> + Send) {
        let value = glib::SendValue::from_owned(value);
        self.set_value(name, value);
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given `value` if the `predicate` evaluates to `true`.
    ///
    /// This has no effect if the `predicate` evaluates to `false`,
    /// i.e. default or previous value for `name` is kept.
    #[doc(alias = "gst_structure_set")]
    pub fn set_if(
        &mut self,
        name: impl IntoGStr,
        value: impl Into<glib::Value> + Send,
        predicate: bool,
    ) {
        if predicate {
            self.set(name, value);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given inner value if `value` is `Some`.
    ///
    /// This has no effect if the value is `None`, i.e. default or previous value for `name` is kept.
    #[doc(alias = "gst_structure_set")]
    pub fn set_if_some(
        &mut self,
        name: impl IntoGStr,
        value: Option<impl Into<glib::Value> + Send>,
    ) {
        if let Some(value) = value {
            self.set(name, value);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` using the given `ValueType` `V` built from `iter`'s the `Item`s.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn set_from_iter<V: ValueType + Into<Value> + FromIterator<SendValue> + Send>(
        &mut self,
        name: impl IntoGStr,
        iter: impl IntoIterator<Item = impl ToSendValue>,
    ) {
        let iter = iter.into_iter().map(|item| item.to_send_value());
        self.set(name, V::from_iter(iter));
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` using the given `ValueType` `V` built from `iter`'s Item`s,
    /// if `iter` is not empty.
    ///
    /// This has no effect if `iter` is empty, i.e. previous value for `name` is unchanged.
    #[inline]
    pub fn set_if_not_empty<V: ValueType + Into<Value> + FromIterator<SendValue> + Send>(
        &mut self,
        name: impl IntoGStr,
        iter: impl IntoIterator<Item = impl ToSendValue>,
    ) {
        let mut iter = iter.into_iter().peekable();
        if iter.peek().is_some() {
            let iter = iter.map(|item| item.to_send_value());
            self.set(name, V::from_iter(iter));
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[doc(alias = "gst_structure_set_value")]
    pub fn set_value(&mut self, name: impl IntoGStr, value: SendValue) {
        unsafe {
            name.run_with_gstr(|name| {
                ffi::gst_structure_take_value(&mut self.0, name.as_ptr(), &mut value.into_raw())
            });
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given `value` if the `predicate` evaluates to `true`.
    ///
    /// This has no effect if the `predicate` evaluates to `false`,
    /// i.e. default or previous value for `name` is kept.
    #[doc(alias = "gst_structure_set_value")]
    pub fn set_value_if(&mut self, name: impl IntoGStr, value: SendValue, predicate: bool) {
        if predicate {
            self.set_value(name, value);
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given inner value if `value` is `Some`.
    ///
    /// This has no effect if the value is `None`, i.e. default or previous value for `name` is kept.
    #[doc(alias = "gst_structure_set_value")]
    pub fn set_value_if_some(&mut self, name: impl IntoGStr, value: Option<SendValue>) {
        if let Some(value) = value {
            self.set_value(name, value);
        }
    }

    #[doc(alias = "gst_structure_id_set")]
    pub fn set_by_quark(&mut self, name: glib::Quark, value: impl Into<glib::Value> + Send) {
        let value = glib::SendValue::from_owned(value);
        self.set_value_by_quark(name, value);
    }

    #[doc(alias = "gst_structure_id_set")]
    pub fn set_by_quark_if_some(
        &mut self,
        name: glib::Quark,
        value: Option<impl Into<glib::Value> + Send>,
    ) {
        if let Some(value) = value {
            self.set_by_quark(name, value);
        }
    }

    #[doc(alias = "gst_structure_id_set_value")]
    pub fn set_value_by_quark(&mut self, name: glib::Quark, value: SendValue) {
        unsafe {
            ffi::gst_structure_id_take_value(&mut self.0, name.into_glib(), &mut value.into_raw());
        }
    }

    #[doc(alias = "gst_structure_id_set_value")]
    pub fn set_value_by_quark_if_some(&mut self, name: glib::Quark, value: Option<SendValue>) {
        if let Some(value) = value {
            self.set_value_by_quark(name, value);
        }
    }

    #[doc(alias = "get_name")]
    #[doc(alias = "gst_structure_get_name")]
    pub fn name<'a>(&self) -> &'a glib::GStr {
        unsafe {
            let name = ffi::gst_structure_get_name(&self.0);
            // Ensure the name is static whatever the GStreamer version being used.
            glib::GStr::from_ptr(glib::ffi::g_intern_string(name))
        }
    }

    #[doc(alias = "gst_structure_get_name_id")]
    pub fn name_quark(&self) -> glib::Quark {
        unsafe { from_glib(ffi::gst_structure_get_name_id(&self.0)) }
    }

    #[doc(alias = "gst_structure_set_name")]
    pub fn set_name(&mut self, name: impl IntoGStr) {
        unsafe {
            name.run_with_gstr(|name| ffi::gst_structure_set_name(&mut self.0, name.as_ptr()))
        }
    }

    #[doc(alias = "gst_structure_set_name")]
    pub fn set_name_if_some(&mut self, name: Option<impl IntoGStr>) {
        if let Some(name) = name {
            self.set_name(name);
        }
    }

    #[doc(alias = "gst_structure_has_name")]
    pub fn has_name(&self, name: &str) -> bool {
        self.name() == name
    }

    #[doc(alias = "gst_structure_has_field")]
    pub fn has_field(&self, field: impl IntoGStr) -> bool {
        unsafe {
            field.run_with_gstr(|field| {
                from_glib(ffi::gst_structure_has_field(&self.0, field.as_ptr()))
            })
        }
    }

    #[doc(alias = "gst_structure_has_field_typed")]
    pub fn has_field_with_type(&self, field: impl IntoGStr, type_: glib::Type) -> bool {
        unsafe {
            field.run_with_gstr(|field| {
                from_glib(ffi::gst_structure_has_field_typed(
                    &self.0,
                    field.as_ptr(),
                    type_.into_glib(),
                ))
            })
        }
    }

    #[doc(alias = "gst_structure_id_has_field")]
    pub fn has_field_by_quark(&self, field: glib::Quark) -> bool {
        unsafe { from_glib(ffi::gst_structure_id_has_field(&self.0, field.into_glib())) }
    }

    #[doc(alias = "gst_structure_id_has_field_typed")]
    pub fn has_field_with_type_by_quark(&self, field: glib::Quark, type_: glib::Type) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_id_has_field_typed(
                &self.0,
                field.into_glib(),
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_structure_remove_field")]
    pub fn remove_field(&mut self, field: impl IntoGStr) {
        unsafe {
            field.run_with_gstr(|field| {
                ffi::gst_structure_remove_field(&mut self.0, field.as_ptr())
            });
        }
    }

    #[doc(alias = "gst_structure_remove_fields")]
    pub fn remove_fields(&mut self, fields: impl IntoIterator<Item = impl IntoGStr>) {
        for f in fields.into_iter() {
            self.remove_field(f)
        }
    }

    #[doc(alias = "gst_structure_remove_all_fields")]
    pub fn remove_all_fields(&mut self) {
        unsafe {
            ffi::gst_structure_remove_all_fields(&mut self.0);
        }
    }

    pub fn fields(&self) -> FieldIterator {
        FieldIterator::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    #[doc(alias = "get_nth_field_name")]
    #[doc(alias = "gst_structure_nth_field_name")]
    pub fn nth_field_name<'a>(&self, idx: usize) -> Option<&'a glib::GStr> {
        if idx >= self.n_fields() {
            return None;
        }

        unsafe {
            let field_name = ffi::gst_structure_nth_field_name(&self.0, idx as u32);
            debug_assert!(!field_name.is_null());

            // Ensure the name is static whatever the GStreamer version being used.
            Some(glib::GStr::from_ptr(glib::ffi::g_intern_string(field_name)))
        }
    }

    #[doc(alias = "gst_structure_n_fields")]
    pub fn n_fields(&self) -> usize {
        unsafe { ffi::gst_structure_n_fields(&self.0) as usize }
    }

    pub fn len(&self) -> usize {
        self.n_fields()
    }

    pub fn is_empty(&self) -> bool {
        self.n_fields() == 0
    }

    #[doc(alias = "gst_structure_can_intersect")]
    pub fn can_intersect(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_can_intersect(&self.0, &other.0)) }
    }

    #[doc(alias = "gst_structure_intersect")]
    pub fn intersect(&self, other: &StructureRef) -> Option<Structure> {
        unsafe { from_glib_full(ffi::gst_structure_intersect(&self.0, &other.0)) }
    }

    #[doc(alias = "gst_structure_is_subset")]
    pub fn is_subset(&self, superset: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_subset(&self.0, &superset.0)) }
    }

    #[doc(alias = "gst_structure_fixate")]
    pub fn fixate(&mut self) {
        unsafe { ffi::gst_structure_fixate(&mut self.0) }
    }

    #[doc(alias = "gst_structure_fixate_field")]
    pub fn fixate_field(&mut self, name: impl IntoGStr) -> bool {
        unsafe {
            name.run_with_gstr(|name| {
                from_glib(ffi::gst_structure_fixate_field(&mut self.0, name.as_ptr()))
            })
        }
    }

    #[doc(alias = "gst_structure_fixate_field_boolean")]
    pub fn fixate_field_bool(&mut self, name: impl IntoGStr, target: bool) -> bool {
        unsafe {
            name.run_with_gstr(|name| {
                from_glib(ffi::gst_structure_fixate_field_boolean(
                    &mut self.0,
                    name.as_ptr(),
                    target.into_glib(),
                ))
            })
        }
    }

    #[doc(alias = "gst_structure_fixate_field_string")]
    pub fn fixate_field_str(&mut self, name: impl IntoGStr, target: impl IntoGStr) -> bool {
        unsafe {
            name.run_with_gstr(|name| {
                target.run_with_gstr(|target| {
                    from_glib(ffi::gst_structure_fixate_field_string(
                        &mut self.0,
                        name.as_ptr(),
                        target.as_ptr(),
                    ))
                })
            })
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_double")]
    pub fn fixate_field_nearest_double(&mut self, name: impl IntoGStr, target: f64) -> bool {
        unsafe {
            name.run_with_gstr(|name| {
                from_glib(ffi::gst_structure_fixate_field_nearest_double(
                    &mut self.0,
                    name.as_ptr(),
                    target,
                ))
            })
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_fraction")]
    pub fn fixate_field_nearest_fraction(
        &mut self,
        name: impl IntoGStr,
        target: impl Into<Fraction>,
    ) -> bool {
        skip_assert_initialized!();

        let target = target.into();
        unsafe {
            name.run_with_gstr(|name| {
                from_glib(ffi::gst_structure_fixate_field_nearest_fraction(
                    &mut self.0,
                    name.as_ptr(),
                    target.numer(),
                    target.denom(),
                ))
            })
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_int")]
    pub fn fixate_field_nearest_int(&mut self, name: impl IntoGStr, target: i32) -> bool {
        unsafe {
            name.run_with_gstr(|name| {
                from_glib(ffi::gst_structure_fixate_field_nearest_int(
                    &mut self.0,
                    name.as_ptr(),
                    target,
                ))
            })
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_structure_serialize")]
    pub fn serialize(&self, flags: crate::SerializeFlags) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_structure_serialize(&self.0, flags.into_glib())) }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_structure_serialize")]
    #[doc(alias = "gst_structure_serialize_full")]
    pub fn serialize_strict(
        &self,
        flags: crate::SerializeFlags,
    ) -> Result<glib::GString, glib::BoolError> {
        unsafe {
            let res = ffi::gst_structure_serialize_full(
                &self.0,
                flags.into_glib() | ffi::GST_SERIALIZE_FLAG_STRICT,
            );
            if res.is_null() {
                Err(glib::bool_error!("Failed to serialize structure to string"))
            } else {
                Ok(from_glib_full(res))
            }
        }
    }

    #[doc(alias = "gst_structure_foreach")]
    pub fn foreach<F: FnMut(glib::Quark, &glib::Value) -> std::ops::ControlFlow<()>>(
        &self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, &glib::Value) -> std::ops::ControlFlow<()>,
            >(
                quark: glib::ffi::GQuark,
                value: *const glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(from_glib(quark), &*(value as *const glib::Value));

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_structure_foreach(
                self.as_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_structure_map_in_place")]
    pub fn map_in_place<F: FnMut(glib::Quark, &mut glib::Value) -> std::ops::ControlFlow<()>>(
        &mut self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, &mut glib::Value) -> std::ops::ControlFlow<()>,
            >(
                quark: glib::ffi::GQuark,
                value: *mut glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(from_glib(quark), &mut *(value as *mut glib::Value));

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_structure_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_structure_filter_and_map_in_place")]
    pub fn filter_map_in_place<F: FnMut(glib::Quark, glib::Value) -> Option<glib::Value>>(
        &mut self,
        mut func: F,
    ) {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, glib::Value) -> Option<glib::Value>,
            >(
                quark: glib::ffi::GQuark,
                value: *mut glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);

                let v = mem::replace(
                    &mut *(value as *mut glib::Value),
                    glib::Value::uninitialized(),
                );
                match func(from_glib(quark), v) {
                    None => glib::ffi::GFALSE,
                    Some(v) => {
                        *value = v.into_raw();
                        glib::ffi::GTRUE
                    }
                }
            }

            let func = &mut func as *mut F;
            ffi::gst_structure_filter_and_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            );
        }
    }
}

impl fmt::Display for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { glib::GString::from_glib_full(ffi::gst_structure_to_string(&self.0)) };
        f.write_str(&s)
    }
}

impl fmt::Debug for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct(self.name());

        for (id, field) in self.iter() {
            if field.type_() == Structure::static_type() {
                let s = field.get::<Structure>().unwrap();
                debug.field(id, &s);
            } else if field.type_() == crate::Array::static_type() {
                let arr = field.get::<crate::Array>().unwrap();
                debug.field(id, &arr);
            } else if field.type_() == crate::List::static_type() {
                let list = field.get::<crate::List>().unwrap();
                debug.field(id, &list);
            } else {
                debug.field(id, &field);
            }
        }

        debug.finish()
    }
}

impl PartialEq for StructureRef {
    #[doc(alias = "gst_structure_is_equal")]
    fn eq(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for StructureRef {}

impl glib::types::StaticType for StructureRef {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a StructureRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const StructureRef)
    }
}

impl glib::value::ToValue for StructureRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Structure>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_ptr() as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for StructureRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Structure>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| s.as_ptr()).unwrap_or(ptr::null()) as *mut _,
            )
        }
        value
    }
}

#[derive(Debug)]
pub struct FieldIterator<'a> {
    structure: &'a StructureRef,
    idx: usize,
    n_fields: usize,
}

impl<'a> FieldIterator<'a> {
    fn new(structure: &'a StructureRef) -> FieldIterator<'a> {
        skip_assert_initialized!();
        let n_fields = structure.n_fields();

        FieldIterator {
            structure,
            idx: 0,
            n_fields,
        }
    }
}

impl Iterator for FieldIterator<'_> {
    type Item = &'static glib::GStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n_fields {
            return None;
        }

        // Safety: nth_field_name() ensures static lifetime for the returned string,
        // whatever the GStreamer version being used.
        let field_name = self.structure.nth_field_name(self.idx).unwrap();
        self.idx += 1;

        Some(field_name)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.n_fields - self.idx;

        (remaining, Some(remaining))
    }
}

impl DoubleEndedIterator for FieldIterator<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.n_fields {
            return None;
        }

        self.n_fields -= 1;
        // Safety: nth_field_name() ensures static lifetime for the returned string,
        // whatever the GStreamer version being used.
        Some(self.structure.nth_field_name(self.n_fields).unwrap())
    }
}

impl ExactSizeIterator for FieldIterator<'_> {}

impl std::iter::FusedIterator for FieldIterator<'_> {}

#[derive(Debug)]
pub struct Iter<'a> {
    // Safety: FieldIterator ensures static lifetime for the returned Item,
    // whatever the GStreamer version being used.
    iter: FieldIterator<'a>,
}

impl<'a> Iter<'a> {
    fn new(structure: &'a StructureRef) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            iter: FieldIterator::new(structure),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'static glib::GStr, &'a SendValue);

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.iter.next()?;
        let v = self.iter.structure.value(f);
        Some((f, v.unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let f = self.iter.nth(n)?;
        let v = self.iter.structure.value(f);
        Some((f, v.unwrap()))
    }

    fn last(self) -> Option<Self::Item> {
        let structure = self.iter.structure;
        let f = self.iter.last()?;
        let v = structure.value(f);
        Some((f, v.unwrap()))
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let f = self.iter.next_back()?;
        let v = self.iter.structure.value(f);
        Some((f, v.unwrap()))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let f = self.iter.nth_back(n)?;
        let v = self.iter.structure.value(f);
        Some((f, v.unwrap()))
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl std::iter::FusedIterator for Iter<'_> {}

impl<'a> IntoIterator for &'a StructureRef {
    type IntoIter = Iter<'a>;
    type Item = (&'static glib::GStr, &'a SendValue);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::iter::Extend<(&'a str, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (&'a str, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(f, v));
    }
}

impl<'a> std::iter::Extend<(&'a glib::GStr, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (&'a glib::GStr, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(f, v));
    }
}

impl std::iter::Extend<(String, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (String, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(&f, v));
    }
}

impl std::iter::Extend<(glib::GString, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (glib::GString, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(&f, v));
    }
}

impl std::iter::Extend<(glib::Quark, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (glib::Quark, SendValue)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(f, v)| self.set_value_by_quark(f, v));
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct Builder {
    s: Structure,
}

impl Builder {
    fn new(name: impl IntoGStr) -> Self {
        skip_assert_initialized!();
        Builder {
            s: Structure::new_empty(name),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn field(mut self, name: impl IntoGStr, value: impl Into<glib::Value> + Send) -> Self {
        self.s.set(name, value);
        self
    }

    impl_builder_gvalue_extra_setters!(field);

    #[must_use = "Building the structure without using it has no effect"]
    pub fn build(self) -> Structure {
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_get() {
        use glib::{value, Type};

        crate::init().unwrap();

        let mut s = Structure::new_empty("test");
        assert_eq!(s.name(), "test");

        s.set("f1", "abc");
        s.set("f2", String::from("bcd"));
        s.set("f3", 123i32);
        s.set("f5", Some("efg"));
        s.set("f7", 42i32);

        assert_eq!(s.get::<&str>("f1"), Ok("abc"));
        assert_eq!(s.get::<Option<&str>>("f2"), Ok(Some("bcd")));
        assert_eq!(s.get::<i32>("f3"), Ok(123i32));
        assert_eq!(s.get_optional::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get_optional::<&str>("f4"), Ok(None));
        assert_eq!(s.get_optional::<i32>("f3"), Ok(Some(123i32)));
        assert_eq!(s.get_optional::<i32>("f4"), Ok(None));
        assert_eq!(s.get::<&str>("f5"), Ok("efg"));
        assert_eq!(s.get::<i32>("f7"), Ok(42i32));

        assert_eq!(
            s.get::<i32>("f2"),
            Err(GetError::from_value_get_error(
                "f2",
                value::ValueTypeMismatchError::new(Type::STRING, Type::I32),
            ))
        );
        assert_eq!(
            s.get::<bool>("f3"),
            Err(GetError::from_value_get_error(
                "f3",
                value::ValueTypeMismatchError::new(Type::I32, Type::BOOL),
            ))
        );
        assert_eq!(
            s.get::<&str>("f4"),
            Err(GetError::new_field_not_found("f4"))
        );
        assert_eq!(s.get::<i32>("f4"), Err(GetError::new_field_not_found("f4")));

        assert_eq!(
            s.fields().collect::<Vec<_>>(),
            vec!["f1", "f2", "f3", "f5", "f7"]
        );

        let v = s.iter().map(|(f, v)| (f, v.clone())).collect::<Vec<_>>();
        assert_eq!(v.len(), 5);
        assert_eq!(v[0].0, "f1");
        assert_eq!(v[0].1.get::<&str>(), Ok("abc"));
        assert_eq!(v[1].0, "f2");
        assert_eq!(v[1].1.get::<&str>(), Ok("bcd"));
        assert_eq!(v[2].0, "f3");
        assert_eq!(v[2].1.get::<i32>(), Ok(123i32));
        assert_eq!(v[3].0, "f5");
        assert_eq!(v[3].1.get::<&str>(), Ok("efg"));
        assert_eq!(v[4].0, "f7");
        assert_eq!(v[4].1.get::<i32>(), Ok(42i32));

        let s2 = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", String::from("bcd"))
            .field("f3", 123i32)
            .field_if_some("f4", Option::<i32>::None)
            .field_if_some("f5", Some("efg"))
            .field_if_some("f6", Option::<&str>::None)
            .field_if("f7", 42i32, true)
            .field_if("f8", 21i32, false)
            .build();
        assert_eq!(s, s2);

        let mut s3 = Structure::new_empty("test");

        s3.set_if_some("f1", Some("abc"));
        s3.set_if_some("f2", Some(String::from("bcd")));
        s3.set_if_some("f3", Some(123i32));
        s3.set_if_some("f4", Option::<i32>::None);
        s3.set_if_some("f5", Some("efg"));
        s3.set_if_some("f6", Option::<&str>::None);
        s3.set_if("f7", 42i32, true);
        s3.set_if("f8", 21i32, false);
        assert_eq!(s, s3);
    }

    #[test]
    fn test_string_conversion() {
        crate::init().unwrap();

        let a = "Test, f1=(string)abc, f2=(uint)123;";

        let s = a.parse::<Structure>().unwrap();
        assert_eq!(s.get::<&str>("f1"), Ok("abc"));
        assert_eq!(s.get::<u32>("f2"), Ok(123));

        assert_eq!(a, s.to_string());
    }

    #[test]
    fn test_from_value_optional() {
        use glib::value::ToValue;

        crate::init().unwrap();

        let a = None::<&Structure>.to_value();
        assert!(a.get::<Option<Structure>>().unwrap().is_none());
        let b = "foo".parse::<Structure>().unwrap().to_value();
        assert!(b.get::<Option<Structure>>().unwrap().is_some());
    }

    #[test]
    fn test_new_from_iter() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", String::from("bcd"))
            .field("f3", 123i32)
            .build();

        let s2 = Structure::from_iter(
            s.name(),
            s.iter()
                .filter(|(f, _)| *f == "f1")
                .map(|(f, v)| (f, v.clone())),
        );

        assert_eq!(s2.name(), "test");
        assert_eq!(s2.get::<&str>("f1"), Ok("abc"));
        assert!(s2.get::<&str>("f2").is_err());
        assert!(s2.get::<&str>("f3").is_err());
    }

    #[test]
    fn test_debug() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", String::from("bcd"))
            .field("f3", 123i32)
            .field(
                "f4",
                Structure::builder("nested").field("badger", true).build(),
            )
            .field("f5", crate::Array::new(["a", "b", "c"]))
            .field("f6", crate::List::new(["d", "e", "f"]))
            .build();

        assert_eq!(format!("{s:?}"), "Structure(test { f1: (gchararray) \"abc\", f2: (gchararray) \"bcd\", f3: (gint) 123, f4: Structure(nested { badger: (gboolean) TRUE }), f5: Array([(gchararray) \"a\", (gchararray) \"b\", (gchararray) \"c\"]), f6: List([(gchararray) \"d\", (gchararray) \"e\", (gchararray) \"f\"]) })");
    }

    #[test]
    fn builder_field_from_iter() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field_from_iter::<crate::Array>("array", [&1, &2, &3])
            .field_from_iter::<crate::List>("list", [&4, &5, &6])
            .build();
        assert!(s
            .get::<crate::Array>("array")
            .unwrap()
            .iter()
            .map(|val| val.get::<i32>().unwrap())
            .eq([1, 2, 3]));
        assert!(s
            .get::<crate::List>("list")
            .unwrap()
            .iter()
            .map(|val| val.get::<i32>().unwrap())
            .eq([4, 5, 6]));

        let array = Vec::<i32>::new();
        let s = Structure::builder("test")
            .field_from_iter::<crate::Array>("array", &array)
            .field_from_iter::<crate::List>("list", &array)
            .build();
        assert!(s.get::<crate::Array>("array").unwrap().as_ref().is_empty());
        assert!(s.get::<crate::List>("list").unwrap().as_ref().is_empty());
    }

    #[test]
    fn builder_field_if_not_empty() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field_if_not_empty::<crate::Array>("array", [&1, &2, &3])
            .field_if_not_empty::<crate::List>("list", [&4, &5, &6])
            .build();
        assert!(s
            .get::<crate::Array>("array")
            .unwrap()
            .iter()
            .map(|val| val.get::<i32>().unwrap())
            .eq([1, 2, 3]));
        assert!(s
            .get::<crate::List>("list")
            .unwrap()
            .iter()
            .map(|val| val.get::<i32>().unwrap())
            .eq([4, 5, 6]));

        let array = Vec::<i32>::new();
        let s = Structure::builder("test")
            .field_if_not_empty::<crate::Array>("array", &array)
            .field_if_not_empty::<crate::List>("list", &array)
            .build();
        assert!(!s.has_field("array"));
        assert!(!s.has_field("list"));
    }

    #[test]
    fn test_name_and_field_name_lt() {
        crate::init().unwrap();

        let (name, field_name) = {
            let s = Structure::builder("name")
                .field("field0", "val0")
                .field("field1", "val1")
                .build();

            (s.name(), s.nth_field_name(0).unwrap())
        };

        assert_eq!(name, "name");
        assert_eq!(field_name, "field0");
    }
}
