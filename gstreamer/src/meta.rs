// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use std::ptr;
use std::{
    fmt,
    marker::PhantomData,
    ops::{self, Bound, RangeBounds},
};

use glib::translate::*;

use crate::{ffi, Buffer, BufferRef, Caps, CapsRef, ClockTime};

pub unsafe trait MetaAPI: Sync + Send + Sized {
    type GstType;

    #[doc(alias = "get_meta_api")]
    fn meta_api() -> glib::Type;
}

pub trait MetaAPIExt: MetaAPI {
    #[inline]
    unsafe fn from_ptr(buffer: &BufferRef, ptr: *const Self::GstType) -> MetaRef<Self> {
        debug_assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            debug_assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRef {
            meta: &*(ptr as *const Self),
            buffer,
        }
    }

    #[inline]
    unsafe fn from_mut_ptr<T>(
        buffer: &mut BufferRef,
        ptr: *mut Self::GstType,
    ) -> MetaRefMut<Self, T> {
        debug_assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            debug_assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRefMut {
            meta: &mut *(ptr as *mut Self),
            buffer,
            mode: PhantomData,
        }
    }
}

impl<A: MetaAPI> MetaAPIExt for A {}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct MetaSeqnum(u64);

pub struct MetaRef<'a, T: 'a> {
    meta: &'a T,
    buffer: &'a BufferRef,
}

pub enum Standalone {}
pub enum Iterated {}

pub struct MetaRefMut<'a, T: 'a, U> {
    meta: &'a mut T,
    buffer: &'a mut BufferRef,
    mode: PhantomData<U>,
}

impl<'a, T: fmt::Debug + 'a> fmt::Debug for MetaRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<'a, T: fmt::Debug + 'a, U> fmt::Debug for MetaRefMut<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .field("mode", &self.mode)
            .finish()
    }
}

impl<T> ops::Deref for MetaRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T> AsRef<MetaRef<'a, T>> for MetaRef<'a, T> {
    #[inline]
    fn as_ref(&self) -> &MetaRef<'a, T> {
        self
    }
}

impl<T> AsRef<T> for MetaRef<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.meta
    }
}

impl<'a, T: 'a> Clone for MetaRef<'a, T> {
    fn clone(&self) -> Self {
        MetaRef {
            meta: self.meta,
            buffer: self.buffer,
        }
    }
}

impl<T, U> ops::Deref for MetaRefMut<'_, T, U> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.meta
    }
}

impl<T, U> ops::DerefMut for MetaRefMut<'_, T, U> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T, U> AsRef<MetaRef<'a, T>> for MetaRefMut<'a, T, U> {
    #[inline]
    fn as_ref(&self) -> &MetaRef<'a, T> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, T>) }
    }
}

impl<T, U> AsMut<T> for MetaRefMut<'_, T, U> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T> MetaRef<'a, T> {
    #[doc(alias = "get_api")]
    #[inline]
    pub fn api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[inline]
    pub fn flags(&self) -> crate::MetaFlags {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            from_glib((*meta).flags)
        }
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).type_)
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    #[inline]
    pub fn seqnum(&self) -> MetaSeqnum {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            MetaSeqnum(ffi::gst_meta_get_seqnum(meta))
        }
    }

    #[inline]
    #[doc(alias = "gst_meta_api_type_has_tag")]
    pub fn has_tag<MT: MetaTag>(&self) -> bool {
        self.has_tag_by_quark(MT::quark())
    }

    #[inline]
    pub fn has_tag_by_quark(&self, tag: glib::Quark) -> bool {
        meta_api_type_has_tag_by_quark(self.api(), tag)
    }

    #[inline]
    #[doc(alias = "gst_meta_api_type_get_tags")]
    pub fn tags<'b>(&self) -> &'b [glib::GStringPtr] {
        meta_api_type_get_tags(self.api())
    }

    #[inline]
    pub fn upcast_ref(&self) -> &MetaRef<'a, Meta> {
        unsafe { &*(self as *const MetaRef<'a, T> as *const MetaRef<'a, Meta>) }
    }

    pub fn transform<MT>(&self, buffer: &mut BufferRef, data: &'a MT) -> Result<(), glib::BoolError>
    where
        T: MetaAPI,
        MT: MetaTransform<'a>,
    {
        unsafe {
            let info = *(*self.upcast_ref().as_ptr()).info;
            let Some(transform_func) = info.transform_func else {
                return Err(glib::bool_error!(
                    "Can't copy meta without transform function"
                ));
            };

            let data = data.to_raw(self)?;

            glib::result_from_gboolean!(
                transform_func(
                    buffer.as_mut_ptr(),
                    mut_override(self.upcast_ref().as_ptr()),
                    mut_override(self.buffer.as_ptr()),
                    MT::quark().into_glib(),
                    mut_override(&data) as *mut _,
                ),
                "Failed to transform meta"
            )
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_meta_serialize")]
    pub fn serialize<B: ByteArrayInterface + ?Sized>(
        &self,
        writer: &mut B,
    ) -> Result<usize, glib::BoolError> {
        unsafe {
            #[repr(C)]
            struct Writer<'a, B: ?Sized> {
                iface_: ffi::GstByteArrayInterface,
                writer: &'a mut B,
            }

            unsafe extern "C" fn resize<B: ByteArrayInterface + ?Sized>(
                iface_: *mut ffi::GstByteArrayInterface,
                size: usize,
            ) -> glib::ffi::gboolean {
                let iface_ = &mut *(iface_ as *mut Writer<B>);

                match iface_.writer.resize(size) {
                    Some(new_data) => {
                        iface_.iface_.data = new_data.as_mut_ptr();
                        iface_.iface_.len = size;

                        glib::ffi::GTRUE
                    }
                    None => glib::ffi::GFALSE,
                }
            }

            let initial_len = writer.initial_len();

            let mut iface_ = Writer {
                iface_: ffi::GstByteArrayInterface {
                    data: writer.as_mut().as_mut_ptr(),
                    len: initial_len,
                    resize: Some(resize::<B>),
                    _gst_reserved: [ptr::null_mut(); 4],
                },
                writer: &mut *writer,
            };

            let res = bool::from_glib(ffi::gst_meta_serialize(
                self.meta as *const T as *const ffi::GstMeta,
                &mut iface_.iface_,
            ));

            if !res {
                return Err(glib::bool_error!("Failed to serialize meta"));
            }

            assert!(iface_.iface_.len >= initial_len);

            Ok(iface_.iface_.len - initial_len)
        }
    }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
pub trait ByteArrayInterface: AsMut<[u8]> {
    fn initial_len(&self) -> usize;
    fn resize(&mut self, size: usize) -> Option<&mut [u8]>;
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
impl ByteArrayInterface for Vec<u8> {
    fn initial_len(&self) -> usize {
        self.len()
    }

    fn resize(&mut self, size: usize) -> Option<&mut [u8]> {
        self.resize(size, 0);
        Some(&mut self[0..size])
    }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
impl<A: smallvec::Array<Item = u8>> ByteArrayInterface for smallvec::SmallVec<A> {
    fn initial_len(&self) -> usize {
        self.len()
    }

    fn resize(&mut self, size: usize) -> Option<&mut [u8]> {
        self.resize(size, 0);
        Some(&mut self[0..size])
    }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
impl ByteArrayInterface for &mut [u8] {
    fn initial_len(&self) -> usize {
        0
    }

    fn resize(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.len() < size {
            return None;
        }

        Some(&mut self[0..size])
    }
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
impl<const N: usize> ByteArrayInterface for [u8; N] {
    fn initial_len(&self) -> usize {
        0
    }

    fn resize(&mut self, size: usize) -> Option<&mut [u8]> {
        if N < size {
            return None;
        }

        Some(&mut self[0..size])
    }
}

impl<'a> MetaRef<'a, Meta> {
    #[inline]
    pub fn downcast_ref<T: MetaAPI>(&self) -> Option<&MetaRef<'a, T>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, T>) })
        } else {
            None
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
    pub fn try_as_custom_meta(&self) -> Option<&MetaRef<'a, CustomMeta>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, CustomMeta>))
        }
    }
}

impl<'a, T, U> MetaRefMut<'a, T, U> {
    #[doc(alias = "get_api")]
    #[inline]
    pub fn api(&self) -> glib::Type {
        self.as_meta_ref().api()
    }

    #[inline]
    pub fn flags(&self) -> crate::MetaFlags {
        self.as_meta_ref().flags()
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        self.as_meta_ref().type_()
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    #[inline]
    pub fn seqnum(&self) -> MetaSeqnum {
        self.as_meta_ref().seqnum()
    }

    #[inline]
    #[doc(alias = "gst_meta_api_type_has_tag")]
    pub fn has_tag<MT: MetaTag>(&self) -> bool {
        self.as_meta_ref().has_tag::<MT>()
    }

    #[inline]
    pub fn has_tag_by_quark(&self, tag: glib::Quark) -> bool {
        self.as_meta_ref().has_tag_by_quark(tag)
    }

    #[inline]
    #[doc(alias = "gst_meta_api_type_get_tags")]
    pub fn tags<'b>(&self) -> &'b [glib::GStringPtr] {
        self.as_meta_ref().tags()
    }

    #[inline]
    pub fn upcast_ref(&self) -> &MetaRef<'a, Meta> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, Meta>) }
    }

    #[inline]
    pub fn upcast_mut(&mut self) -> &mut MetaRefMut<'a, Meta, U> {
        unsafe { &mut *(self as *mut MetaRefMut<'a, T, U> as *mut MetaRefMut<'a, Meta, U>) }
    }

    #[inline]
    pub fn as_meta_ref(&self) -> MetaRef<T> {
        MetaRef {
            meta: self.meta,
            buffer: self.buffer,
        }
    }

    pub fn transform<MT>(
        &'a self,
        buffer: &mut BufferRef,
        data: &'a MT,
    ) -> Result<(), glib::BoolError>
    where
        T: MetaAPI,
        MT: MetaTransform<'a>,
    {
        self.as_meta_ref().transform(buffer, data)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *mut _ as *mut <T as MetaAPI>::GstType
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_meta_serialize")]
    pub fn serialize<B: ByteArrayInterface + ?Sized>(
        &self,
        writer: &mut B,
    ) -> Result<usize, glib::BoolError> {
        self.as_meta_ref().serialize(writer)
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    pub fn clear(&mut self) -> Result<(), glib::BoolError>
    where
        T: MetaAPI,
    {
        unsafe {
            let info = *(*self.upcast_ref().as_ptr()).info;

            if let Some(clear_func) = info.clear_func {
                clear_func(self.buffer.as_mut_ptr(), self.upcast_mut().as_mut_ptr());
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to clear meta"))
            }
        }
    }
}

impl<T> MetaRefMut<'_, T, Standalone> {
    #[doc(alias = "gst_buffer_remove_meta")]
    pub fn remove(self) -> Result<(), glib::BoolError> {
        if self.flags().contains(crate::MetaFlags::LOCKED) {
            return Err(glib::bool_error!("Can't remove locked meta"));
        }

        unsafe {
            let res = ffi::gst_buffer_remove_meta(
                self.buffer.as_mut_ptr(),
                self.meta as *mut T as *mut ffi::GstMeta,
            );
            debug_assert_ne!(res, glib::ffi::GFALSE);

            Ok(())
        }
    }
}

impl<'a, U> MetaRefMut<'a, Meta, U> {
    #[inline]
    pub fn downcast_ref<T: MetaAPI>(&mut self) -> Option<&MetaRefMut<'a, T, U>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *mut MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, T, U>) })
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: MetaAPI>(&mut self) -> Option<&mut MetaRefMut<'a, T, U>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe {
                &mut *(self as *mut MetaRefMut<'a, Meta, U> as *mut MetaRefMut<'a, T, U>)
            })
        } else {
            None
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
    pub fn try_as_custom_meta(&self) -> Option<&MetaRefMut<'a, CustomMeta, U>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&*(self as *const MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, CustomMeta, U>))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
    pub fn try_as_mut_custom_meta(&mut self) -> Option<&mut MetaRefMut<'a, CustomMeta, U>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&mut *(self as *mut MetaRefMut<'a, Meta, U> as *mut MetaRefMut<'a, CustomMeta, U>))
        }
    }
}

#[repr(transparent)]
#[doc(alias = "GstMeta")]
pub struct Meta(ffi::GstMeta);

unsafe impl Send for Meta {}
unsafe impl Sync for Meta {}

unsafe impl MetaAPI for Meta {
    type GstType = ffi::GstMeta;

    #[inline]
    fn meta_api() -> glib::Type {
        glib::Type::INVALID
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Meta")
            .field("api", &unsafe { glib::Type::from_glib((*self.0.info).api) })
            .field("type", &unsafe {
                glib::Type::from_glib((*self.0.info).type_)
            })
            .field("flags", &unsafe {
                crate::MetaFlags::from_glib(self.0.flags)
            })
            .finish()
    }
}

impl Meta {
    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_meta_deserialize")]
    pub fn deserialize<'a>(
        buffer: &'a mut BufferRef,
        data: &[u8],
        consumed: &mut usize,
    ) -> Result<MetaRefMut<'a, Self, Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        unsafe {
            use std::mem;

            let mut consumed_u32 = mem::MaybeUninit::uninit();

            let res = ffi::gst_meta_deserialize(
                buffer.as_mut_ptr(),
                data.as_ptr(),
                data.len(),
                consumed_u32.as_mut_ptr(),
            );

            *consumed = consumed_u32.assume_init() as usize;

            if res.is_null() {
                return Err(glib::bool_error!("Failed to deserialize meta"));
            }

            Ok(MetaRefMut {
                meta: &mut *(res as *mut Self),
                buffer,
                mode: PhantomData,
            })
        }
    }
}

#[repr(transparent)]
#[doc(alias = "GstParentBufferMeta")]
pub struct ParentBufferMeta(ffi::GstParentBufferMeta);

unsafe impl Send for ParentBufferMeta {}
unsafe impl Sync for ParentBufferMeta {}

impl ParentBufferMeta {
    #[doc(alias = "gst_buffer_add_parent_buffer_meta")]
    pub fn add<'a>(buffer: &'a mut BufferRef, parent: &Buffer) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_parent_buffer_meta(
                buffer.as_mut_ptr(),
                parent.to_glib_none().0,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_parent")]
    #[inline]
    pub fn parent(&self) -> &BufferRef {
        unsafe { BufferRef::from_ptr(self.0.buffer) }
    }

    #[doc(alias = "get_parent_owned")]
    #[inline]
    pub fn parent_owned(&self) -> Buffer {
        unsafe { from_glib_none(self.0.buffer) }
    }
}

unsafe impl MetaAPI for ParentBufferMeta {
    type GstType = ffi::GstParentBufferMeta;

    #[doc(alias = "gst_parent_buffer_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_parent_buffer_meta_api_get_type()) }
    }
}

impl fmt::Debug for ParentBufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParentBufferMeta")
            .field("parent", &self.parent())
            .finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstProtectionMeta")]
pub struct ProtectionMeta(ffi::GstProtectionMeta);

unsafe impl Send for ProtectionMeta {}
unsafe impl Sync for ProtectionMeta {}

impl ProtectionMeta {
    #[doc(alias = "gst_buffer_add_protection_meta")]
    pub fn add(buffer: &mut BufferRef, info: crate::Structure) -> MetaRefMut<Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta =
                ffi::gst_buffer_add_protection_meta(buffer.as_mut_ptr(), info.into_glib_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_info")]
    #[inline]
    pub fn info(&self) -> &crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow(self.0.info) }
    }

    #[doc(alias = "get_info_mut")]
    #[inline]
    pub fn info_mut(&mut self) -> &mut crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow_mut(self.0.info) }
    }
}

unsafe impl MetaAPI for ProtectionMeta {
    type GstType = ffi::GstProtectionMeta;

    #[doc(alias = "gst_protection_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_protection_meta_api_get_type()) }
    }
}

impl fmt::Debug for ProtectionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ProtectionMeta")
            .field("info", &self.info())
            .finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstReferenceTimestampMeta")]
pub struct ReferenceTimestampMeta(ffi::GstReferenceTimestampMeta);

unsafe impl Send for ReferenceTimestampMeta {}
unsafe impl Sync for ReferenceTimestampMeta {}

impl ReferenceTimestampMeta {
    #[doc(alias = "gst_buffer_add_reference_timestamp_meta")]
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        reference: &Caps,
        timestamp: ClockTime,
        duration: impl Into<Option<ClockTime>>,
    ) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_reference_timestamp_meta(
                buffer.as_mut_ptr(),
                reference.to_glib_none().0,
                timestamp.into_glib(),
                duration.into().into_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_reference")]
    #[inline]
    pub fn reference(&self) -> &CapsRef {
        unsafe { CapsRef::from_ptr(self.0.reference) }
    }

    #[doc(alias = "get_reference_owned")]
    #[inline]
    pub fn reference_owned(&self) -> Caps {
        unsafe { from_glib_none(self.0.reference) }
    }

    #[doc(alias = "get_timestamp")]
    #[inline]
    pub fn timestamp(&self) -> ClockTime {
        unsafe { try_from_glib(self.0.timestamp).expect("undefined timestamp") }
    }

    #[doc(alias = "get_duration")]
    #[inline]
    pub fn duration(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.duration) }
    }
}

unsafe impl MetaAPI for ReferenceTimestampMeta {
    type GstType = ffi::GstReferenceTimestampMeta;

    #[doc(alias = "gst_reference_timestamp_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_reference_timestamp_meta_api_get_type()) }
    }
}

impl fmt::Debug for ReferenceTimestampMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;

        f.debug_struct("ReferenceTimestampMeta")
            .field("reference", &self.reference())
            .field("timestamp", &self.timestamp().display())
            .field("duration", &self.duration().display())
            .finish()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[repr(transparent)]
#[doc(alias = "GstCustomMeta")]
pub struct CustomMeta(ffi::GstCustomMeta);

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Send for CustomMeta {}
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Sync for CustomMeta {}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl CustomMeta {
    #[doc(alias = "gst_meta_register_custom")]
    pub fn register(name: &str, tags: &[&str]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                tags.to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
            );
        }
    }

    #[doc(alias = "gst_meta_register_custom")]
    pub fn register_with_transform<
        F: Fn(&mut BufferRef, &CustomMeta, &BufferRef, glib::Quark) -> bool + Send + Sync + 'static,
    >(
        name: &str,
        tags: &[&str],
        transform_func: F,
    ) {
        assert_initialized_main_thread!();
        unsafe extern "C" fn transform_func_trampoline<
            F: Fn(&mut BufferRef, &CustomMeta, &BufferRef, glib::Quark) -> bool
                + Send
                + Sync
                + 'static,
        >(
            dest: *mut ffi::GstBuffer,
            meta: *mut ffi::GstCustomMeta,
            src: *mut ffi::GstBuffer,
            type_: glib::ffi::GQuark,
            _data: glib::ffi::gpointer,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = &*(user_data as *const F);
            let res = func(
                BufferRef::from_mut_ptr(dest),
                &*(meta as *const CustomMeta),
                BufferRef::from_ptr(src),
                from_glib(type_),
            );
            res.into_glib()
        }

        unsafe extern "C" fn transform_func_free<F>(ptr: glib::ffi::gpointer) {
            let _ = Box::from_raw(ptr as *mut F);
        }

        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                tags.to_glib_none().0,
                Some(transform_func_trampoline::<F>),
                Box::into_raw(Box::new(transform_func)) as glib::ffi::gpointer,
                Some(transform_func_free::<F>),
            );
        }
    }

    #[doc(alias = "gst_meta_register_simple")]
    pub fn register_simple(name: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                [ptr::null()].as_mut_ptr(),
                None,
                ptr::null_mut(),
                None,
            );
        }
    }

    pub fn is_registered(name: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe { name.run_with_gstr(|name| !ffi::gst_meta_get_info(name.as_ptr()).is_null()) }
    }

    #[doc(alias = "gst_buffer_add_custom_meta")]
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        name: &str,
    ) -> Result<MetaRefMut<'a, Self, Standalone>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to add custom meta"));
            }

            Ok(MetaRefMut {
                meta: &mut *(meta as *mut Self),
                buffer,
                mode: PhantomData,
            })
        }
    }

    #[doc(alias = "gst_buffer_get_custom_meta")]
    pub fn from_buffer<'a>(
        buffer: &'a BufferRef,
        name: &str,
    ) -> Result<MetaRef<'a, Self>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_get_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to get custom meta"));
            }

            Ok(MetaRef {
                meta: &*(meta as *const Self),
                buffer,
            })
        }
    }

    #[doc(alias = "gst_buffer_get_custom_meta")]
    pub fn from_mut_buffer<'a>(
        buffer: &'a mut BufferRef,
        name: &str,
    ) -> Result<MetaRefMut<'a, Self, Standalone>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_get_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to get custom meta"));
            }

            Ok(MetaRefMut {
                meta: &mut *(meta as *mut Self),
                buffer,
                mode: PhantomData,
            })
        }
    }

    #[doc(alias = "gst_custom_meta_get_structure")]
    #[inline]
    pub fn structure(&self) -> &crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow(ffi::gst_custom_meta_get_structure(mut_override(
                &self.0,
            )))
        }
    }

    #[doc(alias = "gst_custom_meta_get_structure")]
    #[inline]
    pub fn mut_structure(&mut self) -> &mut crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow_mut(ffi::gst_custom_meta_get_structure(
                &mut self.0,
            ))
        }
    }

    #[doc(alias = "gst_custom_meta_has_name")]
    #[inline]
    pub fn has_name(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_custom_meta_has_name(
                mut_override(&self.0),
                name.to_glib_none().0,
            ))
        }
    }
}

pub trait MetaTag {
    const TAG_NAME: &'static glib::GStr;
    fn quark() -> glib::Quark;
}

#[macro_export]
macro_rules! impl_meta_tag(
    ($name:ident, $gst_tag:path) => {
        pub enum $name {}
        impl $crate::meta::MetaTag for $name {
            const TAG_NAME: &'static glib::GStr = unsafe { glib::GStr::from_utf8_with_nul_unchecked($gst_tag) };
	    fn quark() -> glib::Quark {
                static QUARK: std::sync::OnceLock<glib::Quark> = std::sync::OnceLock::new();
                *QUARK.get_or_init(|| glib::Quark::from_static_str(Self::TAG_NAME))
            }
        }
    };
);

pub mod tags {
    impl_meta_tag!(Memory, crate::ffi::GST_META_TAG_MEMORY_STR);
    impl_meta_tag!(
        MemoryReference,
        crate::ffi::GST_META_TAG_MEMORY_REFERENCE_STR
    );
}

pub unsafe trait MetaTransform<'a> {
    type GLibType;
    fn quark() -> glib::Quark;
    fn to_raw<T: MetaAPI>(&self, meta: &MetaRef<T>) -> Result<Self::GLibType, glib::BoolError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaTransformCopy {
    range: (Bound<usize>, Bound<usize>),
    region: bool,
}

impl MetaTransformCopy {
    pub fn new(region: bool, range: impl RangeBounds<usize>) -> Self {
        skip_assert_initialized!();
        MetaTransformCopy {
            range: (range.start_bound().cloned(), range.end_bound().cloned()),
            region,
        }
    }
}

unsafe impl MetaTransform<'_> for MetaTransformCopy {
    type GLibType = ffi::GstMetaTransformCopy;

    fn quark() -> glib::Quark {
        static QUARK: std::sync::OnceLock<glib::Quark> = std::sync::OnceLock::new();
        *QUARK.get_or_init(|| glib::Quark::from_static_str(glib::gstr!("gst-copy")))
    }
    fn to_raw<T: MetaAPI>(
        &self,
        meta: &MetaRef<T>,
    ) -> Result<ffi::GstMetaTransformCopy, glib::BoolError> {
        let (offset, size) = meta.buffer.byte_range_into_offset_len(self.range)?;

        Ok(ffi::GstMetaTransformCopy {
            region: self.region.into_glib(),
            offset,
            size,
        })
    }
}

#[inline]
#[doc(alias = "gst_meta_api_type_has_tag")]
pub fn meta_api_type_has_tag<MT: MetaTag>(type_: glib::Type) -> bool {
    skip_assert_initialized!();
    meta_api_type_has_tag_by_quark(type_, MT::quark())
}

#[inline]
#[doc(alias = "gst_meta_api_type_has_tag")]
pub fn meta_api_type_has_tag_by_quark(type_: glib::Type, tag: glib::Quark) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_meta_api_type_has_tag(
            type_.into_glib(),
            tag.into_glib(),
        ))
    }
}

#[inline]
#[doc(alias = "gst_meta_api_type_get_tags")]
pub fn meta_api_type_get_tags<'b>(type_: glib::Type) -> &'b [glib::GStringPtr] {
    skip_assert_initialized!();
    unsafe { glib::StrV::from_glib_borrow(ffi::gst_meta_api_type_get_tags(type_.into_glib())) }
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
#[doc(alias = "gst_meta_api_type_aggregate_params")]
pub fn meta_api_type_aggregate_params(
    type_: glib::Type,
    params1: &crate::StructureRef,
    params2: &crate::StructureRef,
) -> Result<Option<crate::Structure>, glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        let mut new_params = ptr::null_mut();
        let res = bool::from_glib(ffi::gst_meta_api_type_aggregate_params(
            type_.into_glib(),
            &mut new_params,
            params1.as_ptr(),
            params2.as_ptr(),
        ));

        if res {
            Ok(from_glib_full(new_params))
        } else {
            Err(glib::bool_error!("Failed to aggregate meta type params"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_iterate_meta() {
        crate::init().unwrap();

        let mut buffer = crate::Buffer::new();
        let parent = crate::Buffer::new();
        {
            let meta = ParentBufferMeta::add(buffer.get_mut().unwrap(), &parent);
            assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
            assert!(!metas[0].has_tag_by_quark(glib::Quark::from_str("video")));
            assert!(metas[0].has_tag::<tags::MemoryReference>());
            assert_eq!(metas[0].tags().len(), 1);

            assert_eq!(metas[0].tags(), metas[0].upcast_ref().tags());
        }

        {
            let meta = buffer
                .get_mut()
                .unwrap()
                .meta_mut::<ParentBufferMeta>()
                .unwrap();
            assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
            meta.remove().unwrap();
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }

        assert!(buffer.meta::<ParentBufferMeta>().is_none());
    }

    #[test]
    fn test_copy_reference_timestamp_meta() {
        crate::init().unwrap();

        let caps = crate::Caps::new_empty_simple("timestamp/x-ntp");
        let mut buffer = crate::Buffer::new();
        {
            ReferenceTimestampMeta::add(
                buffer.get_mut().unwrap(),
                &caps,
                crate::ClockTime::from_seconds(1),
                crate::ClockTime::NONE,
            );
        }

        let mut buffer_dest = crate::Buffer::new();
        {
            let meta = buffer.meta::<ReferenceTimestampMeta>().unwrap();
            let buffer_dest = buffer_dest.get_mut().unwrap();
            meta.transform(buffer_dest, &MetaTransformCopy::new(false, ..))
                .unwrap();
        }

        let meta = buffer_dest.meta::<ReferenceTimestampMeta>().unwrap();
        assert_eq!(meta.reference(), &caps);
        assert_eq!(meta.timestamp(), crate::ClockTime::from_seconds(1));
        assert_eq!(meta.duration(), crate::ClockTime::NONE);
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[test]
    fn test_meta_serialize() {
        use smallvec::SmallVec;

        crate::init().unwrap();

        let caps = crate::Caps::new_empty_simple("timestamp/x-ntp");
        let mut buffer = crate::Buffer::new();

        let meta = ReferenceTimestampMeta::add(
            buffer.get_mut().unwrap(),
            &caps,
            crate::ClockTime::from_seconds(1),
            crate::ClockTime::NONE,
        );

        let mut data_1 = Vec::new();
        let mut data_2 = [0u8; 128];
        let mut data_3 = SmallVec::<[u8; 128]>::new();

        let len_1 = meta.serialize(&mut data_1).unwrap();
        let len_2 = meta.serialize(&mut data_2).unwrap();
        let len_3 = meta.serialize(&mut data_3).unwrap();
        assert_eq!(&data_1[..len_1], &data_2[..len_2]);
        assert_eq!(&data_1[..len_1], &data_3[..len_3]);

        assert!(meta.serialize(&mut [0]).is_err());

        let mut buffer_dest = crate::Buffer::new();
        let mut consumed = 0;
        let mut meta =
            Meta::deserialize(buffer_dest.get_mut().unwrap(), &data_1, &mut consumed).unwrap();
        assert_eq!(consumed, len_1);

        let meta = meta.downcast_ref::<ReferenceTimestampMeta>().unwrap();
        assert_eq!(meta.reference(), &caps);
        assert_eq!(meta.timestamp(), crate::ClockTime::from_seconds(1));
        assert_eq!(meta.duration(), crate::ClockTime::NONE);

        let mut consumed = 0;
        assert!(
            Meta::deserialize(buffer_dest.get_mut().unwrap(), &[0, 1, 2], &mut consumed).is_err()
        );
        assert_eq!(consumed, 0);
    }
}
