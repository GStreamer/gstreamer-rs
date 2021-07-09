// Take a look at the license at the top of the repository in the LICENSE file.

use crate::caps_features::*;
use crate::structure::*;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;
use std::str;

use crate::CapsIntersectMode;

use glib::translate::{from_glib, from_glib_full, FromGlibPtrFull, IntoGlib, ToGlibPtr};
use glib::value::ToSendValue;

mini_object_wrapper!(Caps, CapsRef, ffi::GstCaps, || { ffi::gst_caps_get_type() });

impl Caps {
    pub fn builder(name: &str) -> Builder<NoFeature> {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    pub fn builder_full() -> BuilderFull<SomeFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::new()
    }

    pub fn builder_full_with_features(features: CapsFeatures) -> BuilderFull<SomeFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::with_features(features)
    }

    pub fn builder_full_with_any_features() -> BuilderFull<AnyFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::with_any_features()
    }

    #[doc(alias = "gst_caps_new_empty")]
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_empty()) }
    }

    #[doc(alias = "gst_caps_new_any")]
    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_any()) }
    }

    pub fn new_simple(name: &str, values: &[(&str, &(dyn ToSendValue + Sync))]) -> Self {
        assert_initialized_main_thread!();
        let mut caps = Caps::new_empty();

        let structure = Structure::new(name, values);
        caps.get_mut().unwrap().append_structure(structure);

        caps
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<'a, I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a StructureRef>,
    {
        assert_initialized_main_thread!();
        let mut caps = Caps::new_empty();

        iter.into_iter()
            .for_each(|s| caps.get_mut().unwrap().append_structure(s.to_owned()));

        caps
    }

    pub fn from_iter_with_features<'a, 'b, I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a StructureRef, &'b CapsFeaturesRef)>,
    {
        assert_initialized_main_thread!();
        let mut caps = Caps::new_empty();

        iter.into_iter().for_each(|(s, f)| {
            caps.get_mut()
                .unwrap()
                .append_structure_full(s.to_owned(), Some(f.to_owned()))
        });

        caps
    }

    #[doc(alias = "gst_caps_fixate")]
    pub fn fixate(&mut self) {
        skip_assert_initialized!();
        unsafe {
            // See https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/388
            assert!(!self.is_any());
            let ptr = if self.is_empty() {
                ffi::gst_caps_new_empty()
            } else {
                ffi::gst_caps_fixate(self.as_mut_ptr())
            };
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge")]
    pub fn merge(&mut self, other: Self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_merge(self.as_mut_ptr(), other.into_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge_structure")]
    pub fn merge_structure(&mut self, structure: Structure) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_merge_structure(self.as_mut_ptr(), structure.into_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge_structure_full")]
    pub fn merge_structure_full(&mut self, structure: Structure, features: Option<CapsFeatures>) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_merge_structure_full(
                self.as_mut_ptr(),
                structure.into_ptr(),
                features.map(|f| f.into_ptr()).unwrap_or(ptr::null_mut()),
            );
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_normalize")]
    pub fn normalize(&mut self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_normalize(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_simplify")]
    pub fn simplify(&mut self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_simplify(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_truncate")]
    pub fn truncate(&mut self) {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_caps_truncate(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }
}

impl str::FromStr for Caps {
    type Err = glib::BoolError;

    #[doc(alias = "gst_caps_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_caps_from_string(s.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to parse caps from string"))
        }
    }
}

impl CapsRef {
    pub fn set_simple(&mut self, values: &[(&str, &(dyn ToSendValue + Sync))]) {
        for &(name, value) in values {
            let value = value.to_value();

            unsafe {
                ffi::gst_caps_set_value(
                    self.as_mut_ptr(),
                    name.to_glib_none().0,
                    value.to_glib_none().0,
                );
            }
        }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_caps_get_structure")]
    pub fn structure(&self, idx: u32) -> Option<&StructureRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow(structure))
        }
    }

    #[doc(alias = "get_mut_structure")]
    pub fn structure_mut(&mut self, idx: u32) -> Option<&mut StructureRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow_mut(structure))
        }
    }

    #[doc(alias = "get_features")]
    #[doc(alias = "gst_caps_get_features")]
    pub fn features(&self, idx: u32) -> Option<&CapsFeaturesRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let features = ffi::gst_caps_get_features(self.as_ptr(), idx);
            Some(CapsFeaturesRef::from_glib_borrow(features))
        }
    }

    #[doc(alias = "get_mut_features")]
    pub fn features_mut(&mut self, idx: u32) -> Option<&mut CapsFeaturesRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let features = ffi::gst_caps_get_features(self.as_ptr(), idx);
            Some(CapsFeaturesRef::from_glib_borrow_mut(features))
        }
    }

    #[doc(alias = "gst_caps_set_features")]
    pub fn set_features(&mut self, idx: u32, features: Option<CapsFeatures>) {
        assert!(idx < self.size());

        unsafe {
            ffi::gst_caps_set_features(
                self.as_mut_ptr(),
                idx,
                features.map(|f| f.into_ptr()).unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_caps_set_features_simple")]
    pub fn set_features_simple(&mut self, features: Option<CapsFeatures>) {
        unsafe {
            ffi::gst_caps_set_features_simple(
                self.as_mut_ptr(),
                features.map(|f| f.into_ptr()).unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_caps_get_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::gst_caps_get_size(self.as_ptr()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }

    pub fn iter_with_features(&self) -> IterFeatures {
        IterFeatures::new(self)
    }

    pub fn iter_with_features_mut(&mut self) -> IterFeaturesMut {
        IterFeaturesMut::new(self)
    }

    #[doc(alias = "gst_caps_append_structure")]
    pub fn append_structure(&mut self, structure: Structure) {
        unsafe { ffi::gst_caps_append_structure(self.as_mut_ptr(), structure.into_ptr()) }
    }

    #[doc(alias = "gst_caps_append_structure_full")]
    pub fn append_structure_full(&mut self, structure: Structure, features: Option<CapsFeatures>) {
        unsafe {
            ffi::gst_caps_append_structure_full(
                self.as_mut_ptr(),
                structure.into_ptr(),
                features.map(|f| f.into_ptr()).unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[doc(alias = "gst_caps_remove_structure")]
    pub fn remove_structure(&mut self, idx: u32) {
        unsafe { ffi::gst_caps_remove_structure(self.as_mut_ptr(), idx) }
    }

    #[doc(alias = "gst_caps_append")]
    pub fn append(&mut self, other: Caps) {
        unsafe { ffi::gst_caps_append(self.as_mut_ptr(), other.into_ptr()) }
    }

    #[doc(alias = "gst_caps_can_intersect")]
    pub fn can_intersect(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_can_intersect(self.as_ptr(), other.as_ptr())) }
    }

    #[doc(alias = "gst_caps_intersect")]
    pub fn intersect(&self, other: &Self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
            ))
        }
    }

    pub fn intersect_with_mode(&self, other: &Self, mode: CapsIntersectMode) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect_full(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
                mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_always_compatible")]
    pub fn is_always_compatible(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_always_compatible(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_any")]
    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_any(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_empty(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_fixed")]
    pub fn is_fixed(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_fixed(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_equal_fixed")]
    pub fn is_equal_fixed(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal_fixed(self.as_ptr(), other.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_strictly_equal")]
    pub fn is_strictly_equal(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_strictly_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_subset")]
    pub fn is_subset(&self, superset: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_subset(self.as_ptr(), superset.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_subset_structure")]
    pub fn is_subset_structure(&self, structure: &StructureRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_subset_structure(
                self.as_ptr(),
                structure.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_subset_structure_full")]
    pub fn is_subset_structure_full(
        &self,
        structure: &StructureRef,
        features: Option<&CapsFeaturesRef>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_subset_structure_full(
                self.as_ptr(),
                structure.as_ptr(),
                features.map(|f| f.as_ptr()).unwrap_or(ptr::null()),
            ))
        }
    }

    #[doc(alias = "gst_caps_subtract")]
    pub fn subtract(&self, other: &Self) -> Caps {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_caps_subtract(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_caps_serialize")]
    pub fn serialize(&self, flags: crate::SerializeFlags) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_caps_serialize(&self.0, flags.into_glib())) }
    }
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $styp:ty, $get_item:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        caps: $typ,
        idx: u32,
        n_structures: u32,
    }

    impl<'a> $name<'a> {
        fn new(caps: $typ) -> $name<'a> {
            skip_assert_initialized!();
            let n_structures = caps.size();

            $name {
                caps,
                idx: 0,
                n_structures,
            }
        }
    }

    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.n_structures {
                return None;
            }

            unsafe {
                let item = $get_item(self.caps, self.idx)?;
                self.idx += 1;
                Some(item)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.idx == self.n_structures {
                return (0, Some(0));
            }

            let remaining = (self.n_structures - self.idx) as usize;

            (remaining, Some(remaining))
        }
    }

    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.n_structures {
                return None;
            }

            self.n_structures -= 1;

            unsafe {
                $get_item(self.caps, self.n_structures)
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}
    }
);

define_iter!(
    Iter,
    &'a CapsRef,
    &'a StructureRef,
    |caps: &CapsRef, idx| {
        let ptr = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(StructureRef::from_glib_borrow(
                ptr as *const ffi::GstStructure,
            ))
        }
    }
);
define_iter!(
    IterMut,
    &'a mut CapsRef,
    &'a mut StructureRef,
    |caps: &CapsRef, idx| {
        let ptr = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(StructureRef::from_glib_borrow_mut(
                ptr as *mut ffi::GstStructure,
            ))
        }
    }
);
define_iter!(
    IterFeatures,
    &'a CapsRef,
    (&'a StructureRef, &'a CapsFeaturesRef),
    |caps: &CapsRef, idx| {
        let ptr1 = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        let ptr2 = ffi::gst_caps_get_features(caps.as_ptr(), idx);
        if ptr1.is_null() || ptr2.is_null() {
            None
        } else {
            Some((
                StructureRef::from_glib_borrow(ptr1 as *const ffi::GstStructure),
                CapsFeaturesRef::from_glib_borrow(ptr2 as *const ffi::GstCapsFeatures),
            ))
        }
    }
);
define_iter!(
    IterFeaturesMut,
    &'a mut CapsRef,
    (&'a mut StructureRef, &'a mut CapsFeaturesRef),
    |caps: &CapsRef, idx| {
        let ptr1 = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        let ptr2 = ffi::gst_caps_get_features(caps.as_ptr(), idx);
        if ptr1.is_null() || ptr2.is_null() {
            None
        } else {
            Some((
                StructureRef::from_glib_borrow_mut(ptr1 as *mut ffi::GstStructure),
                CapsFeaturesRef::from_glib_borrow_mut(ptr2 as *mut ffi::GstCapsFeatures),
            ))
        }
    }
);

impl fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <CapsRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for Caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <CapsRef as fmt::Display>::fmt(self, f)
    }
}

impl PartialEq for Caps {
    fn eq(&self, other: &Caps) -> bool {
        CapsRef::eq(self, other)
    }
}

impl Eq for Caps {}

impl fmt::Debug for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Caps").field(&self.to_string()).finish()
    }
}

impl fmt::Display for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { glib::GString::from_glib_full(ffi::gst_caps_to_string(self.as_ptr())) };
        f.write_str(&s)
    }
}

impl PartialEq for CapsRef {
    #[doc(alias = "gst_caps_is_equal")]
    fn eq(&self, other: &CapsRef) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for CapsRef {}

pub enum NoFeature {}
pub enum HasFeatures {}

pub struct Builder<T> {
    s: crate::Structure,
    features: Option<CapsFeatures>,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for Builder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder")
            .field("s", &self.s)
            .field("features", &self.features)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl Builder<NoFeature> {
    fn new(name: &str) -> Builder<NoFeature> {
        skip_assert_initialized!();
        Builder {
            s: crate::Structure::new_empty(name),
            features: None,
            phantom: PhantomData,
        }
    }

    pub fn features(self, features: &[&str]) -> Builder<HasFeatures> {
        Builder {
            s: self.s,
            features: Some(CapsFeatures::new(features)),
            phantom: PhantomData,
        }
    }

    pub fn any_features(self) -> Builder<HasFeatures> {
        Builder {
            s: self.s,
            features: Some(CapsFeatures::new_any()),
            phantom: PhantomData,
        }
    }
}

impl<T> Builder<T> {
    pub fn field<V: ToSendValue + Sync>(mut self, name: &str, value: V) -> Self {
        self.s.set(name, value);
        self
    }

    pub fn build(self) -> Caps {
        let mut caps = Caps::new_empty();

        caps.get_mut()
            .unwrap()
            .append_structure_full(self.s, self.features);
        caps
    }
}

pub enum AnyFeatures {}
pub enum SomeFeatures {}

pub struct BuilderFull<T> {
    caps: crate::Caps,
    features: Option<CapsFeatures>,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for BuilderFull<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder")
            .field("caps", &self.caps)
            .field("features", &self.features)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl BuilderFull<SomeFeatures> {
    fn new() -> Self {
        BuilderFull {
            caps: Caps::new_empty(),
            features: None,
            phantom: PhantomData,
        }
    }

    fn with_features(features: CapsFeatures) -> Self {
        skip_assert_initialized!();
        BuilderFull {
            caps: Caps::new_empty(),
            features: Some(features),
            phantom: PhantomData,
        }
    }

    pub fn structure_with_features(self, structure: Structure, features: CapsFeatures) -> Self {
        self.append_structure(structure, Some(features))
    }

    pub fn structure_with_any_features(self, structure: Structure) -> Self {
        self.append_structure(structure, Some(CapsFeatures::new_any()))
    }
}

impl BuilderFull<AnyFeatures> {
    fn with_any_features() -> Self {
        BuilderFull {
            caps: Caps::new_empty(),
            features: Some(CapsFeatures::new_any()),
            phantom: PhantomData,
        }
    }
}

impl<T> BuilderFull<T> {
    fn append_structure(mut self, structure: Structure, features: Option<CapsFeatures>) -> Self {
        let features = {
            match self.features {
                None => features,
                Some(ref result) => {
                    let mut result = result.clone();
                    match features {
                        None => Some(result),
                        Some(features) => {
                            features.iter().for_each(|feat| result.add(feat));
                            Some(result)
                        }
                    }
                }
            }
        };

        self.caps
            .get_mut()
            .unwrap()
            .append_structure_full(structure, features);
        self
    }

    pub fn structure(self, structure: Structure) -> Self {
        self.append_structure(structure, None)
    }

    pub fn build(self) -> Caps {
        self.caps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Array;
    use crate::Fraction;

    #[test]
    fn test_simple() {
        crate::init().unwrap();

        let mut caps = Caps::new_simple(
            "foo/bar",
            &[
                ("int", &12),
                ("bool", &true),
                ("string", &"bla"),
                ("fraction", &Fraction::new(1, 2)),
                ("array", &Array::new(&[&1, &2])),
            ],
        );
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );

        {
            let s = caps.structure(0).unwrap();
            assert_eq!(
                s,
                Structure::new(
                    "foo/bar",
                    &[
                        ("int", &12),
                        ("bool", &true),
                        ("string", &"bla"),
                        ("fraction", &Fraction::new(1, 2)),
                        ("array", &Array::new(&[&1, &2])),
                    ],
                )
                .as_ref()
            );
        }
        assert!(caps
            .features(0)
            .unwrap()
            .is_equal(crate::CAPS_FEATURES_MEMORY_SYSTEM_MEMORY.as_ref()));

        {
            let caps = caps.get_mut().unwrap();
            caps.set_features(0, Some(CapsFeatures::new(&["foo:bla"])));
        }
        assert!(caps
            .features(0)
            .unwrap()
            .is_equal(CapsFeatures::new(&["foo:bla"]).as_ref()));
    }

    #[test]
    fn test_builder() {
        crate::init().unwrap();

        let caps = Caps::builder("foo/bar")
            .field("int", 12)
            .field("bool", true)
            .field("string", "bla")
            .field("fraction", Fraction::new(1, 2))
            .field("array", Array::new(&[&1, &2]))
            .build();
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .any_features()
            .build();
        assert_eq!(caps.to_string(), "foo/bar(ANY), int=(int)12");

        let caps = Caps::builder("foo/bar")
            .field("int", &12)
            .features(&["foo:bla", "foo:baz"])
            .build();
        assert_eq!(caps.to_string(), "foo/bar(foo:bla, foo:baz), int=(int)12");
    }

    #[test]
    fn test_display() {
        crate::init().unwrap();

        let caps = Caps::new_simple("foo/bar", &[]);
        format!("{}", caps);
    }

    #[test]
    fn test_builder_full() {
        crate::init().unwrap();

        let caps = Caps::builder_full()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw; video/x-raw");

        let caps = Caps::builder_full()
            .structure(
                Structure::builder("audio/x-raw")
                    .field("format", &"S16LE")
                    .build(),
            )
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw, format=(string)S16LE; video/x-raw"
        );

        let caps = Caps::builder_full()
            .structure_with_any_features(Structure::builder("audio/x-raw").build())
            .structure_with_features(
                Structure::builder("video/x-raw").build(),
                CapsFeatures::new(&["foo:bla", "foo:baz"]),
            )
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw(ANY); video/x-raw(foo:bla, foo:baz)"
        );
    }

    #[test]
    fn test_builder_full_with_features() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_features(CapsFeatures::new(&["foo:bla"]))
            .structure(Structure::builder("audio/x-raw").build())
            .structure_with_features(
                Structure::builder("video/x-raw").build(),
                CapsFeatures::new(&["foo:baz"]),
            )
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw(foo:bla); video/x-raw(foo:bla, foo:baz)"
        );
    }

    #[test]
    fn test_builder_full_with_any_features() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw(ANY); video/x-raw(ANY)");

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw(ANY)");
    }

    #[test]
    fn test_new_from_iter() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();

        let audio = Caps::from_iter(caps.iter().filter(|s| s.name() == "audio/x-raw"));
        assert_eq!(audio.to_string(), "audio/x-raw");

        let audio = Caps::from_iter_with_features(
            caps.iter_with_features()
                .filter(|(s, _)| s.name() == "audio/x-raw"),
        );
        assert_eq!(audio.to_string(), "audio/x-raw(ANY)");
    }
}
