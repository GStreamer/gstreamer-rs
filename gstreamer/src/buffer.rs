// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    cmp, fmt,
    marker::PhantomData,
    mem, ops,
    ops::{Bound, ControlFlow, Range, RangeBounds},
    ptr, slice,
};

use glib::translate::*;

use crate::{
    ffi, meta::*, BufferCursor, BufferFlags, BufferRefCursor, ClockTime, Memory, MemoryRef,
};

pub enum Readable {}
pub enum Writable {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BufferMetaForeachAction {
    Keep,
    Remove,
}

mini_object_wrapper!(Buffer, BufferRef, ffi::GstBuffer, || {
    ffi::gst_buffer_get_type()
});

pub struct BufferMap<'a, T> {
    buffer: &'a BufferRef,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct MappedBuffer<T> {
    buffer: Buffer,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

impl Buffer {
    #[doc(alias = "gst_buffer_new")]
    #[inline]
    pub fn new() -> Self {
        assert_initialized_main_thread!();

        unsafe { from_glib_full(ffi::gst_buffer_new()) }
    }

    #[doc(alias = "gst_buffer_new_allocate")]
    #[doc(alias = "gst_buffer_new_and_alloc")]
    #[inline]
    pub fn with_size(size: usize) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_new_allocate(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate buffer"))
        }
    }

    #[doc(alias = "gst_buffer_new_wrapped")]
    #[doc(alias = "gst_buffer_new_wrapped_full")]
    #[inline]
    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        let mem = Memory::from_mut_slice(slice);
        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(mem);
            buffer.unset_flags(BufferFlags::TAG_MEMORY);
        }

        buffer
    }

    #[doc(alias = "gst_buffer_new_wrapped")]
    #[doc(alias = "gst_buffer_new_wrapped_full")]
    #[inline]
    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        let mem = Memory::from_slice(slice);
        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(mem);
            buffer.unset_flags(BufferFlags::TAG_MEMORY);
        }

        buffer
    }

    #[doc(alias = "gst_buffer_map")]
    #[inline]
    pub fn into_mapped_buffer_readable(self) -> Result<MappedBuffer<Readable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res: bool = from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ,
            ));
            if res {
                Ok(MappedBuffer {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    #[doc(alias = "gst_buffer_map")]
    #[inline]
    pub fn into_mapped_buffer_writable(self) -> Result<MappedBuffer<Writable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res: bool = from_glib(ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            ));
            if res {
                Ok(MappedBuffer {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    #[inline]
    pub fn into_cursor_readable(self) -> BufferCursor<Readable> {
        BufferCursor::new_readable(self)
    }

    #[inline]
    pub fn into_cursor_writable(self) -> Result<BufferCursor<Writable>, glib::BoolError> {
        BufferCursor::new_writable(self)
    }

    #[doc(alias = "gst_buffer_append")]
    pub fn append(&mut self, other: Self) {
        unsafe {
            let ptr = ffi::gst_buffer_append(self.as_mut_ptr(), other.into_glib_ptr());
            self.replace_ptr(ptr);
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferRef {
    #[doc(alias = "gst_buffer_map")]
    #[inline]
    pub fn map_readable(&self) -> Result<BufferMap<Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res =
                ffi::gst_buffer_map(self.as_mut_ptr(), map_info.as_mut_ptr(), ffi::GST_MAP_READ);
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer readable"))
            }
        }
    }

    #[doc(alias = "gst_buffer_map")]
    #[inline]
    pub fn map_writable(&mut self) -> Result<BufferMap<Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = ffi::gst_buffer_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            );
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer writable"))
            }
        }
    }

    fn memory_range_into_idx_len(
        &self,
        range: impl RangeBounds<usize>,
    ) -> Result<(u32, i32), glib::BoolError> {
        let n_memory = self.n_memory();
        debug_assert!(n_memory <= u32::MAX as usize);

        let start_idx = match range.start_bound() {
            ops::Bound::Included(idx) if *idx >= n_memory => {
                return Err(glib::bool_error!("Invalid range start"));
            }
            ops::Bound::Included(idx) => *idx,
            ops::Bound::Excluded(idx) if idx.checked_add(1).map_or(true, |idx| idx >= n_memory) => {
                return Err(glib::bool_error!("Invalid range start"));
            }
            ops::Bound::Excluded(idx) => *idx + 1,
            ops::Bound::Unbounded => 0,
        };

        let end_idx = match range.end_bound() {
            ops::Bound::Included(idx) if idx.checked_add(1).map_or(true, |idx| idx > n_memory) => {
                return Err(glib::bool_error!("Invalid range end"));
            }
            ops::Bound::Included(idx) => *idx + 1,
            ops::Bound::Excluded(idx) if *idx > n_memory => {
                return Err(glib::bool_error!("Invalid range end"));
            }
            ops::Bound::Excluded(idx) => *idx,
            ops::Bound::Unbounded => n_memory,
        };

        Ok((
            start_idx as u32,
            i32::try_from(end_idx - start_idx).map_err(|_| glib::bool_error!("Too large range"))?,
        ))
    }

    #[doc(alias = "gst_buffer_map_range")]
    #[inline]
    pub fn map_range_readable(
        &self,
        range: impl RangeBounds<usize>,
    ) -> Result<BufferMap<Readable>, glib::BoolError> {
        let (idx, len) = self.memory_range_into_idx_len(range)?;
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = ffi::gst_buffer_map_range(
                self.as_mut_ptr(),
                idx,
                len,
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ,
            );
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer readable"))
            }
        }
    }

    #[doc(alias = "gst_buffer_map_range")]
    #[inline]
    pub fn map_range_writable(
        &mut self,
        range: impl RangeBounds<usize>,
    ) -> Result<BufferMap<Writable>, glib::BoolError> {
        let (idx, len) = self.memory_range_into_idx_len(range)?;
        unsafe {
            let mut map_info = mem::MaybeUninit::uninit();
            let res = ffi::gst_buffer_map_range(
                self.as_mut_ptr(),
                idx,
                len,
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            );
            if res == glib::ffi::GTRUE {
                Ok(BufferMap {
                    buffer: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map buffer writable"))
            }
        }
    }

    pub(crate) fn byte_range_into_offset_len(
        &self,
        range: impl RangeBounds<usize>,
    ) -> Result<(usize, usize), glib::BoolError> {
        let size = self.size();

        let start_idx = match range.start_bound() {
            ops::Bound::Included(idx) if *idx >= size => {
                return Err(glib::bool_error!("Invalid range start"));
            }
            ops::Bound::Included(idx) => *idx,
            ops::Bound::Excluded(idx) if idx.checked_add(1).map_or(true, |idx| idx >= size) => {
                return Err(glib::bool_error!("Invalid range start"));
            }
            ops::Bound::Excluded(idx) => *idx + 1,
            ops::Bound::Unbounded => 0,
        };

        let end_idx = match range.end_bound() {
            ops::Bound::Included(idx) if idx.checked_add(1).map_or(true, |idx| idx > size) => {
                return Err(glib::bool_error!("Invalid range end"));
            }
            ops::Bound::Included(idx) => *idx + 1,
            ops::Bound::Excluded(idx) if *idx > size => {
                return Err(glib::bool_error!("Invalid range end"));
            }
            ops::Bound::Excluded(idx) => *idx,
            ops::Bound::Unbounded => size,
        };

        Ok((start_idx, end_idx - start_idx))
    }

    #[doc(alias = "gst_buffer_copy_region")]
    pub fn copy_region(
        &self,
        flags: crate::BufferCopyFlags,
        range: impl RangeBounds<usize>,
    ) -> Result<Buffer, glib::BoolError> {
        let (offset, size) = self.byte_range_into_offset_len(range)?;

        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_copy_region(
                self.as_mut_ptr(),
                flags.into_glib(),
                offset,
                size,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to copy region of buffer"))
        }
    }

    #[doc(alias = "gst_buffer_copy_into")]
    pub fn copy_into(
        &self,
        dest: &mut BufferRef,
        flags: crate::BufferCopyFlags,
        range: impl RangeBounds<usize>,
    ) -> Result<(), glib::BoolError> {
        let (offset, size) = self.byte_range_into_offset_len(range)?;

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_copy_into(
                    dest.as_mut_ptr(),
                    self.as_mut_ptr(),
                    flags.into_glib(),
                    offset,
                    size,
                ),
                "Failed to copy into destination buffer",
            )
        }
    }

    #[doc(alias = "gst_buffer_fill")]
    pub fn copy_from_slice(&mut self, offset: usize, slice: &[u8]) -> Result<(), usize> {
        let maxsize = self.maxsize();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let src = slice.as_ptr();
            ffi::gst_buffer_fill(
                self.as_mut_ptr(),
                offset,
                src as glib::ffi::gconstpointer,
                size,
            )
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    #[doc(alias = "gst_buffer_extract")]
    pub fn copy_to_slice(&self, offset: usize, slice: &mut [u8]) -> Result<(), usize> {
        let maxsize = self.size();
        let size = slice.len();

        assert!(maxsize >= offset && maxsize - offset >= size);

        let copied = unsafe {
            let dest = slice.as_mut_ptr();
            ffi::gst_buffer_extract(self.as_mut_ptr(), offset, dest as glib::ffi::gpointer, size)
        };

        if copied == size {
            Ok(())
        } else {
            Err(copied)
        }
    }

    #[doc(alias = "gst_buffer_copy_deep")]
    pub fn copy_deep(&self) -> Result<Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_buffer_copy_deep(self.as_ptr()))
                .ok_or_else(|| glib::bool_error!("Failed to deep copy buffer"))
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_buffer_get_size")]
    pub fn size(&self) -> usize {
        unsafe { ffi::gst_buffer_get_size(self.as_mut_ptr()) }
    }

    #[doc(alias = "get_maxsize")]
    pub fn maxsize(&self) -> usize {
        unsafe {
            let mut maxsize = mem::MaybeUninit::uninit();
            ffi::gst_buffer_get_sizes_range(
                self.as_mut_ptr(),
                0,
                -1,
                ptr::null_mut(),
                maxsize.as_mut_ptr(),
            );

            maxsize.assume_init()
        }
    }

    #[doc(alias = "gst_buffer_set_size")]
    pub fn set_size(&mut self, size: usize) {
        assert!(self.maxsize() >= size);

        unsafe {
            ffi::gst_buffer_set_size(self.as_mut_ptr(), size as isize);
        }
    }

    #[doc(alias = "get_offset")]
    #[doc(alias = "GST_BUFFER_OFFSET")]
    #[inline]
    pub fn offset(&self) -> u64 {
        self.0.offset
    }

    #[inline]
    pub fn set_offset(&mut self, offset: u64) {
        self.0.offset = offset;
    }

    #[doc(alias = "get_offset_end")]
    #[doc(alias = "GST_BUFFER_OFFSET_END")]
    #[inline]
    pub fn offset_end(&self) -> u64 {
        self.0.offset_end
    }

    #[inline]
    pub fn set_offset_end(&mut self, offset_end: u64) {
        self.0.offset_end = offset_end;
    }

    #[doc(alias = "get_pts")]
    #[doc(alias = "GST_BUFFER_PTS")]
    #[inline]
    pub fn pts(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.pts) }
    }

    #[inline]
    pub fn set_pts(&mut self, pts: impl Into<Option<ClockTime>>) {
        self.0.pts = pts.into().into_glib();
    }

    #[doc(alias = "get_dts")]
    #[doc(alias = "GST_BUFFER_DTS")]
    #[inline]
    pub fn dts(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.dts) }
    }

    #[inline]
    pub fn set_dts(&mut self, dts: impl Into<Option<ClockTime>>) {
        self.0.dts = dts.into().into_glib();
    }

    #[doc(alias = "get_dts_or_pts")]
    #[doc(alias = "GST_BUFFER_DTS_OR_PTS")]
    #[inline]
    pub fn dts_or_pts(&self) -> Option<ClockTime> {
        let val = self.dts();
        if val.is_none() {
            self.pts()
        } else {
            val
        }
    }

    #[doc(alias = "get_duration")]
    #[doc(alias = "GST_BUFFER_DURATION")]
    #[inline]
    pub fn duration(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.duration) }
    }

    #[inline]
    pub fn set_duration(&mut self, duration: impl Into<Option<ClockTime>>) {
        self.0.duration = duration.into().into_glib();
    }

    #[doc(alias = "get_flags")]
    #[doc(alias = "GST_BUFFER_FLAGS")]
    #[inline]
    pub fn flags(&self) -> BufferFlags {
        BufferFlags::from_bits_truncate(self.0.mini_object.flags)
    }

    #[doc(alias = "GST_BUFFER_FLAG_SET")]
    #[inline]
    pub fn set_flags(&mut self, flags: BufferFlags) {
        self.0.mini_object.flags |= flags.bits();
    }

    #[doc(alias = "GST_BUFFER_FLAG_UNSET")]
    #[inline]
    pub fn unset_flags(&mut self, flags: BufferFlags) {
        self.0.mini_object.flags &= !flags.bits();
    }

    #[doc(alias = "get_meta")]
    #[doc(alias = "gst_buffer_get_meta")]
    #[inline]
    pub fn meta<T: MetaAPI>(&self) -> Option<MetaRef<T>> {
        unsafe {
            let meta = ffi::gst_buffer_get_meta(self.as_mut_ptr(), T::meta_api().into_glib());
            if meta.is_null() {
                None
            } else {
                Some(T::from_ptr(self, meta as *const <T as MetaAPI>::GstType))
            }
        }
    }

    #[doc(alias = "get_meta_mut")]
    #[inline]
    pub fn meta_mut<T: MetaAPI>(&mut self) -> Option<MetaRefMut<T, crate::meta::Standalone>> {
        unsafe {
            let meta = ffi::gst_buffer_get_meta(self.as_mut_ptr(), T::meta_api().into_glib());
            if meta.is_null() {
                None
            } else {
                Some(T::from_mut_ptr(self, meta as *mut <T as MetaAPI>::GstType))
            }
        }
    }

    pub fn iter_meta<T: MetaAPI>(&self) -> MetaIter<T> {
        MetaIter::new(self)
    }

    pub fn iter_meta_mut<T: MetaAPI>(&mut self) -> MetaIterMut<T> {
        MetaIterMut::new(self)
    }

    #[doc(alias = "gst_buffer_foreach_meta")]
    pub fn foreach_meta<F: FnMut(MetaRef<Meta>) -> ControlFlow<(), ()>>(&self, func: F) -> bool {
        unsafe extern "C" fn trampoline<F: FnMut(MetaRef<Meta>) -> ControlFlow<(), ()>>(
            buffer: *mut ffi::GstBuffer,
            meta: *mut *mut ffi::GstMeta,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *mut F;
            let res = (*func)(Meta::from_ptr(BufferRef::from_ptr(buffer), *meta));

            matches!(res, ControlFlow::Continue(_)).into_glib()
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            from_glib(ffi::gst_buffer_foreach_meta(
                mut_override(self.as_ptr()),
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_buffer_foreach_meta")]
    pub fn foreach_meta_mut<
        F: FnMut(
            MetaRefMut<Meta, crate::meta::Iterated>,
        ) -> ControlFlow<BufferMetaForeachAction, BufferMetaForeachAction>,
    >(
        &mut self,
        func: F,
    ) -> bool {
        unsafe extern "C" fn trampoline<
            F: FnMut(
                MetaRefMut<Meta, crate::meta::Iterated>,
            ) -> ControlFlow<BufferMetaForeachAction, BufferMetaForeachAction>,
        >(
            buffer: *mut ffi::GstBuffer,
            meta: *mut *mut ffi::GstMeta,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = user_data as *mut F;
            let res = (*func)(Meta::from_mut_ptr(BufferRef::from_mut_ptr(buffer), *meta));

            let (cont, action) = match res {
                ControlFlow::Continue(action) => (true, action),
                ControlFlow::Break(action) => (false, action),
            };

            if action == BufferMetaForeachAction::Remove {
                *meta = ptr::null_mut();
            }

            cont.into_glib()
        }

        unsafe {
            let mut func = func;
            let func_ptr: &mut F = &mut func;

            from_glib(ffi::gst_buffer_foreach_meta(
                mut_override(self.as_ptr()),
                Some(trampoline::<F>),
                func_ptr as *mut _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_buffer_append_memory")]
    pub fn append_memory(&mut self, mem: Memory) {
        unsafe { ffi::gst_buffer_append_memory(self.as_mut_ptr(), mem.into_glib_ptr()) }
    }

    #[doc(alias = "gst_buffer_find_memory")]
    pub fn find_memory(&self, range: impl RangeBounds<usize>) -> Option<(Range<usize>, usize)> {
        let (offset, size) = self.byte_range_into_offset_len(range).ok()?;

        unsafe {
            let mut idx = mem::MaybeUninit::uninit();
            let mut length = mem::MaybeUninit::uninit();
            let mut skip = mem::MaybeUninit::uninit();

            let res = from_glib(ffi::gst_buffer_find_memory(
                self.as_mut_ptr(),
                offset,
                size,
                idx.as_mut_ptr(),
                length.as_mut_ptr(),
                skip.as_mut_ptr(),
            ));

            if res {
                let idx = idx.assume_init() as usize;
                let length = length.assume_init() as usize;
                let skip = skip.assume_init();
                Some((idx..(idx + length), skip))
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_all_memory")]
    #[doc(alias = "gst_buffer_get_all_memory")]
    pub fn all_memory(&self) -> Option<Memory> {
        unsafe { from_glib_full(ffi::gst_buffer_get_all_memory(self.as_mut_ptr())) }
    }

    #[doc(alias = "get_max_memory")]
    #[doc(alias = "gst_buffer_get_max_memory")]
    pub fn max_memory() -> usize {
        unsafe { ffi::gst_buffer_get_max_memory() as usize }
    }

    #[doc(alias = "get_memory")]
    #[doc(alias = "gst_buffer_get_memory")]
    pub fn memory(&self, idx: usize) -> Option<Memory> {
        if idx >= self.n_memory() {
            return None;
        }
        unsafe {
            let res = ffi::gst_buffer_get_memory(self.as_mut_ptr(), idx as u32);
            Some(from_glib_full(res))
        }
    }

    #[doc(alias = "get_memory_range")]
    #[doc(alias = "gst_buffer_get_memory_range")]
    pub fn memory_range(&self, range: impl RangeBounds<usize>) -> Option<Memory> {
        let (idx, len) = self.memory_range_into_idx_len(range).ok()?;

        unsafe {
            let res = ffi::gst_buffer_get_memory_range(self.as_mut_ptr(), idx, len);
            from_glib_full(res)
        }
    }

    #[doc(alias = "gst_buffer_insert_memory")]
    pub fn insert_memory(&mut self, idx: impl Into<Option<usize>>, mem: Memory) {
        let n_memory = self.n_memory();
        let idx = idx.into();
        let idx = idx.unwrap_or(n_memory);
        assert!(idx <= self.n_memory());
        unsafe { ffi::gst_buffer_insert_memory(self.as_mut_ptr(), idx as i32, mem.into_glib_ptr()) }
    }

    #[doc(alias = "gst_buffer_is_all_memory_writable")]
    pub fn is_all_memory_writable(&self) -> bool {
        unsafe { from_glib(ffi::gst_buffer_is_all_memory_writable(self.as_mut_ptr())) }
    }

    #[doc(alias = "gst_buffer_is_memory_range_writable")]
    pub fn is_memory_range_writable(&self, range: impl RangeBounds<usize>) -> bool {
        let Some((idx, len)) = self.memory_range_into_idx_len(range).ok() else {
            return false;
        };

        unsafe {
            from_glib(ffi::gst_buffer_is_memory_range_writable(
                self.as_mut_ptr(),
                idx,
                len,
            ))
        }
    }

    #[doc(alias = "gst_buffer_n_memory")]
    pub fn n_memory(&self) -> usize {
        unsafe { ffi::gst_buffer_n_memory(self.as_ptr() as *mut _) as usize }
    }

    #[doc(alias = "gst_buffer_peek_memory")]
    pub fn peek_memory(&self, idx: usize) -> &MemoryRef {
        assert!(idx < self.n_memory());
        unsafe { MemoryRef::from_ptr(ffi::gst_buffer_peek_memory(self.as_mut_ptr(), idx as u32)) }
    }

    #[doc(alias = "gst_buffer_peek_memory")]
    pub fn peek_memory_mut(&mut self, idx: usize) -> Result<&mut MemoryRef, glib::BoolError> {
        assert!(idx < self.n_memory());
        unsafe {
            let mem = ffi::gst_buffer_peek_memory(self.as_mut_ptr(), idx as u32);
            if ffi::gst_mini_object_is_writable(mem as *mut _) == glib::ffi::GFALSE {
                Err(glib::bool_error!("Memory not writable"))
            } else {
                Ok(MemoryRef::from_mut_ptr(mem))
            }
        }
    }

    #[doc(alias = "gst_buffer_prepend_memory")]
    pub fn prepend_memory(&mut self, mem: Memory) {
        unsafe { ffi::gst_buffer_prepend_memory(self.as_mut_ptr(), mem.into_glib_ptr()) }
    }

    #[doc(alias = "gst_buffer_remove_all_memory")]
    pub fn remove_all_memory(&mut self) {
        unsafe { ffi::gst_buffer_remove_all_memory(self.as_mut_ptr()) }
    }

    #[doc(alias = "gst_buffer_remove_memory")]
    pub fn remove_memory(&mut self, idx: usize) {
        assert!(idx < self.n_memory());
        unsafe { ffi::gst_buffer_remove_memory(self.as_mut_ptr(), idx as u32) }
    }

    #[doc(alias = "gst_buffer_remove_memory_range")]
    pub fn remove_memory_range(&mut self, range: impl RangeBounds<usize>) {
        let (idx, len) = self
            .memory_range_into_idx_len(range)
            .expect("Invalid memory range");

        unsafe { ffi::gst_buffer_remove_memory_range(self.as_mut_ptr(), idx, len) }
    }

    #[doc(alias = "gst_buffer_replace_all_memory")]
    pub fn replace_all_memory(&mut self, mem: Memory) {
        unsafe { ffi::gst_buffer_replace_all_memory(self.as_mut_ptr(), mem.into_glib_ptr()) }
    }

    #[doc(alias = "gst_buffer_replace_memory")]
    pub fn replace_memory(&mut self, idx: usize, mem: Memory) {
        assert!(idx < self.n_memory());
        unsafe {
            ffi::gst_buffer_replace_memory(self.as_mut_ptr(), idx as u32, mem.into_glib_ptr())
        }
    }

    #[doc(alias = "gst_buffer_replace_memory_range")]
    pub fn replace_memory_range(&mut self, range: impl RangeBounds<usize>, mem: Memory) {
        let (idx, len) = self
            .memory_range_into_idx_len(range)
            .expect("Invalid memory range");

        unsafe {
            ffi::gst_buffer_replace_memory_range(self.as_mut_ptr(), idx, len, mem.into_glib_ptr())
        }
    }

    pub fn iter_memories(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_memories_mut(&mut self) -> Result<IterMut, glib::BoolError> {
        if !self.is_all_memory_writable() {
            Err(glib::bool_error!("Not all memory are writable"))
        } else {
            Ok(IterMut::new(self))
        }
    }

    pub fn iter_memories_owned(&self) -> IterOwned {
        IterOwned::new(self)
    }

    pub fn as_cursor_readable(&self) -> BufferRefCursor<&BufferRef> {
        BufferRefCursor::new_readable(self)
    }

    pub fn as_cursor_writable(
        &mut self,
    ) -> Result<BufferRefCursor<&mut BufferRef>, glib::BoolError> {
        BufferRefCursor::new_writable(self)
    }

    #[doc(alias = "gst_util_dump_buffer")]
    pub fn dump(&self) -> Dump {
        Dump {
            buffer: self,
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }

    #[doc(alias = "gst_util_dump_buffer")]
    pub fn dump_range(&self, range: impl RangeBounds<usize>) -> Dump {
        Dump {
            buffer: self,
            start: range.start_bound().cloned(),
            end: range.end_bound().cloned(),
        }
    }
}

macro_rules! define_meta_iter(
    ($name:ident, $typ:ty, $mtyp:ty, $prepare_buffer:expr, $from_ptr:expr) => {
    pub struct $name<'a, T: MetaAPI + 'a> {
        buffer: $typ,
        state: glib::ffi::gpointer,
        meta_api: glib::Type,
        items: PhantomData<$mtyp>,
    }

    unsafe impl<'a, T: MetaAPI> Send for $name<'a, T> { }
    unsafe impl<'a, T: MetaAPI> Sync for $name<'a, T> { }

    impl<'a, T: MetaAPI> fmt::Debug for $name<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct(stringify!($name))
                .field("buffer", &self.buffer)
                .field("state", &self.state)
                .field("meta_api", &self.meta_api)
                .field("items", &self.items)
                .finish()
        }
    }

    impl<'a, T: MetaAPI> $name<'a, T> {
        fn new(buffer: $typ) -> $name<'a, T> {
            skip_assert_initialized!();

            $name {
                buffer,
                state: ptr::null_mut(),
                meta_api: T::meta_api(),
                items: PhantomData,
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a, T: MetaAPI> Iterator for $name<'a, T> {
        type Item = $mtyp;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                unsafe {
                    let meta = ffi::gst_buffer_iterate_meta(self.buffer.as_mut_ptr(), &mut self.state);

                    if meta.is_null() {
                        return None;
                    } else if self.meta_api == glib::Type::INVALID || glib::Type::from_glib((*(*meta).info).api) == self.meta_api {
                        // FIXME: Workaround for a lifetime issue with the mutable iterator only
                        let buffer = $prepare_buffer(self.buffer.as_mut_ptr());
                        let item = $from_ptr(buffer, meta);
                        return Some(item);
                    }
                }
            }
        }
    }

    impl<'a, T: MetaAPI> std::iter::FusedIterator for $name<'a, T> { }
    }
);

define_meta_iter!(
    MetaIter,
    &'a BufferRef,
    MetaRef<'a, T>,
    |buffer: *const ffi::GstBuffer| BufferRef::from_ptr(buffer),
    |buffer, meta| T::from_ptr(buffer, meta as *const <T as MetaAPI>::GstType)
);
define_meta_iter!(
    MetaIterMut,
    &'a mut BufferRef,
    MetaRefMut<'a, T, crate::meta::Iterated>,
    |buffer: *mut ffi::GstBuffer| BufferRef::from_mut_ptr(buffer),
    |buffer: &'a mut BufferRef, meta| T::from_mut_ptr(buffer, meta as *mut <T as MetaAPI>::GstType)
);

macro_rules! define_iter(
    ($name:ident, $typ:ty, $mtyp:ty, $get_item:expr) => {
    pub struct $name<'a> {
        buffer: $typ,
        idx: usize,
        n_memory: usize,
    }

    impl<'a> fmt::Debug for $name<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct(stringify!($name))
                .field("buffer", &self.buffer)
                .field("idx", &self.idx)
                .field("n_memory", &self.n_memory)
                .finish()
        }
    }

    impl<'a> $name<'a> {
        fn new(buffer: $typ) -> $name<'a> {
            skip_assert_initialized!();

            let n_memory = buffer.n_memory();

            $name {
                buffer,
                idx: 0,
                n_memory,
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> Iterator for $name<'a> {
        type Item = $mtyp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.n_memory {
                return None;
            }

            #[allow(unused_unsafe)]
            unsafe {
                let item = $get_item(self.buffer, self.idx).unwrap();
                self.idx += 1;
                Some(item)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.n_memory - self.idx;

            (remaining, Some(remaining))
        }

        fn count(self) -> usize {
            self.n_memory - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.n_memory || overflow {
                self.idx = self.n_memory;
                None
            } else {
                #[allow(unused_unsafe)]
                unsafe {
                    self.idx = end + 1;
                    Some($get_item(self.buffer, end).unwrap())
                }
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.n_memory {
                None
            } else {
                #[allow(unused_unsafe)]
                unsafe {
                    Some($get_item(self.buffer, self.n_memory - 1).unwrap())
                }
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.n_memory {
                return None;
            }

            #[allow(unused_unsafe)]
            unsafe {
                self.n_memory -= 1;
                Some($get_item(self.buffer, self.n_memory).unwrap())
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.n_memory.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.n_memory;
                None
            } else {
                #[allow(unused_unsafe)]
                unsafe {
                    self.n_memory = end - 1;
                    Some($get_item(self.buffer, self.n_memory).unwrap())
                }
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}

    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(
    Iter,
    &'a BufferRef,
    &'a MemoryRef,
    |buffer: &BufferRef, idx| {
        let ptr = ffi::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx as u32);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_ptr(ptr as *const ffi::GstMemory))
        }
    }
);

define_iter!(
    IterMut,
    &'a mut BufferRef,
    &'a mut MemoryRef,
    |buffer: &mut BufferRef, idx| {
        let ptr = ffi::gst_buffer_peek_memory(buffer.as_mut_ptr(), idx as u32);
        if ptr.is_null() {
            None
        } else {
            Some(MemoryRef::from_mut_ptr(ptr))
        }
    }
);

impl<'a> IntoIterator for &'a BufferRef {
    type IntoIter = Iter<'a>;
    type Item = &'a MemoryRef;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_memories()
    }
}

impl From<Memory> for Buffer {
    fn from(value: Memory) -> Self {
        skip_assert_initialized!();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(value);
        }
        buffer
    }
}

impl<const N: usize> From<[Memory; N]> for Buffer {
    fn from(value: [Memory; N]) -> Self {
        skip_assert_initialized!();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            value.into_iter().for_each(|b| buffer.append_memory(b));
        }
        buffer
    }
}

impl std::iter::FromIterator<Memory> for Buffer {
    fn from_iter<T: IntoIterator<Item = Memory>>(iter: T) -> Self {
        skip_assert_initialized!();
        let iter = iter.into_iter();

        let mut buffer = Buffer::new();

        {
            let buffer = buffer.get_mut().unwrap();
            iter.for_each(|m| buffer.append_memory(m));
        }

        buffer
    }
}

impl std::iter::Extend<Memory> for BufferRef {
    fn extend<T: IntoIterator<Item = Memory>>(&mut self, iter: T) {
        iter.into_iter().for_each(|m| self.append_memory(m));
    }
}

define_iter!(
    IterOwned,
    &'a BufferRef,
    Memory,
    |buffer: &BufferRef, idx| { buffer.memory(idx) }
);

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        BufferRef::fmt(self, f)
    }
}

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        BufferRef::eq(self, other)
    }
}

impl Eq for Buffer {}

impl PartialEq<BufferRef> for Buffer {
    fn eq(&self, other: &BufferRef) -> bool {
        BufferRef::eq(self, other)
    }
}
impl PartialEq<Buffer> for BufferRef {
    fn eq(&self, other: &Buffer) -> bool {
        BufferRef::eq(other, self)
    }
}

impl fmt::Debug for BufferRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::cell::RefCell;

        use crate::utils::Displayable;

        struct DebugIter<I>(RefCell<I>);
        impl<I: Iterator> fmt::Debug for DebugIter<I>
        where
            I::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_list().entries(&mut *self.0.borrow_mut()).finish()
            }
        }

        f.debug_struct("Buffer")
            .field("ptr", &self.as_ptr())
            .field("pts", &self.pts().display())
            .field("dts", &self.dts().display())
            .field("duration", &self.duration().display())
            .field("size", &self.size())
            .field("offset", &self.offset())
            .field("offset_end", &self.offset_end())
            .field("flags", &self.flags())
            .field(
                "metas",
                &DebugIter(RefCell::new(
                    self.iter_meta::<crate::Meta>().map(|m| m.api()),
                )),
            )
            .finish()
    }
}

impl PartialEq for BufferRef {
    fn eq(&self, other: &BufferRef) -> bool {
        if self.size() != other.size() {
            return false;
        }

        let self_map = self.map_readable();
        let other_map = other.map_readable();

        match (self_map, other_map) {
            (Ok(self_map), Ok(other_map)) => self_map.as_slice().eq(other_map.as_slice()),
            _ => false,
        }
    }
}

impl Eq for BufferRef {}

impl<T> BufferMap<'_, T> {
    #[doc(alias = "get_size")]
    #[inline]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_buffer")]
    #[inline]
    pub fn buffer(&self) -> &BufferRef {
        self.buffer
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        if self.map_info.size == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.map_info.data, self.map_info.size) }
    }
}

impl BufferMap<'_, Writable> {
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.map_info.size == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.map_info.data, self.map_info.size) }
    }
}

impl<T> AsRef<[u8]> for BufferMap<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for BufferMap<'_, Writable> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> ops::Deref for BufferMap<'_, T> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl ops::DerefMut for BufferMap<'_, Writable> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> fmt::Debug for BufferMap<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("BufferMap").field(&self.buffer()).finish()
    }
}

impl<'a, T> PartialEq for BufferMap<'a, T> {
    fn eq(&self, other: &BufferMap<'a, T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T> Eq for BufferMap<'_, T> {}

impl<T> Drop for BufferMap<'_, T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_buffer_unmap(self.buffer.as_mut_ptr(), &mut self.map_info);
        }
    }
}

unsafe impl<T> Send for BufferMap<'_, T> {}
unsafe impl<T> Sync for BufferMap<'_, T> {}

impl<T> MappedBuffer<T> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        if self.map_info.size == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.map_info.data, self.map_info.size) }
    }

    #[doc(alias = "get_size")]
    #[inline]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_buffer")]
    #[inline]
    pub fn buffer(&self) -> &BufferRef {
        self.buffer.as_ref()
    }

    #[inline]
    pub fn into_buffer(self) -> Buffer {
        let mut s = mem::ManuallyDrop::new(self);
        let buffer = unsafe { ptr::read(&s.buffer) };
        unsafe {
            ffi::gst_buffer_unmap(buffer.as_mut_ptr(), &mut s.map_info);
        }

        buffer
    }
}

impl MappedBuffer<Readable> {
    #[doc(alias = "get_buffer")]
    #[inline]
    pub fn buffer_owned(&self) -> Buffer {
        self.buffer.clone()
    }
}

impl MappedBuffer<Writable> {
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.map_info.size == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(self.map_info.data, self.map_info.size) }
    }
}

impl<T> AsRef<[u8]> for MappedBuffer<T> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for MappedBuffer<Writable> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> ops::Deref for MappedBuffer<T> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl ops::DerefMut for MappedBuffer<Writable> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> Drop for MappedBuffer<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_buffer_unmap(self.buffer.as_mut_ptr(), &mut self.map_info);
        }
    }
}

impl<T> fmt::Debug for MappedBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MappedBuffer").field(&self.buffer()).finish()
    }
}

impl<T> PartialEq for MappedBuffer<T> {
    fn eq(&self, other: &MappedBuffer<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T> Eq for MappedBuffer<T> {}

unsafe impl<T> Send for MappedBuffer<T> {}
unsafe impl<T> Sync for MappedBuffer<T> {}

#[doc(alias = "GST_BUFFER_COPY_METADATA")]
pub const BUFFER_COPY_METADATA: crate::BufferCopyFlags =
    crate::BufferCopyFlags::from_bits_truncate(ffi::GST_BUFFER_COPY_METADATA);
#[doc(alias = "GST_BUFFER_COPY_ALL")]
pub const BUFFER_COPY_ALL: crate::BufferCopyFlags =
    crate::BufferCopyFlags::from_bits_truncate(ffi::GST_BUFFER_COPY_ALL);

pub struct Dump<'a> {
    buffer: &'a BufferRef,
    start: Bound<usize>,
    end: Bound<usize>,
}

struct BufferChunked16Iter<'a> {
    buffer: &'a BufferRef,
    mem_idx: usize,
    mem_len: usize,
    map: Option<crate::memory::MemoryMap<'a, crate::memory::Readable>>,
    map_offset: usize,
    len: usize,
}

impl Iterator for BufferChunked16Iter<'_> {
    // FIXME: Return a `&'self [u8]` once there's some GAT iterator trait
    type Item = ([u8; 16], usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.mem_idx == self.mem_len || self.len == 0 {
            return None;
        }

        let mut item = [0u8; 16];
        let mut data = item.as_mut_slice();

        while !data.is_empty() && self.mem_idx < self.mem_len && self.len > 0 {
            if self.map.is_none() {
                let mem = self.buffer.peek_memory(self.mem_idx);
                self.map = Some(mem.map_readable().expect("failed to map memory"));
            }

            let map = self.map.as_ref().unwrap();
            debug_assert!(self.map_offset < map.len());
            let copy = cmp::min(cmp::min(map.len() - self.map_offset, data.len()), self.len);
            data[..copy].copy_from_slice(&map[self.map_offset..][..copy]);
            self.map_offset += copy;
            self.len -= copy;
            data = &mut data[copy..];

            if self.map_offset == map.len() {
                self.map = None;
                self.map_offset = 0;
                self.mem_idx += 1;
            }
        }

        let copied = 16 - data.len();
        Some((item, copied))
    }
}

impl Dump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter, debug: bool) -> fmt::Result {
        let n_memory = self.buffer.n_memory();
        if n_memory == 0 {
            write!(f, "<empty>")?;
            return Ok(());
        }

        use std::fmt::Write;

        let len = self.buffer.size();

        // Kind of re-implementation of slice indexing to allow handling out of range values better
        // with specific output strings
        let mut start_idx = match self.start {
            Bound::Included(idx) if idx >= len => {
                write!(f, "<start out of range>")?;
                return Ok(());
            }
            Bound::Excluded(idx) if idx.checked_add(1).map_or(true, |idx| idx >= len) => {
                write!(f, "<start out of range>")?;
                return Ok(());
            }
            Bound::Included(idx) => idx,
            Bound::Excluded(idx) => idx + 1,
            Bound::Unbounded => 0,
        };

        let end_idx = match self.end {
            Bound::Included(idx) if idx.checked_add(1).map_or(true, |idx| idx > len) => {
                write!(f, "<end out of range>")?;
                return Ok(());
            }
            Bound::Excluded(idx) if idx > len => {
                write!(f, "<end out of range>")?;
                return Ok(());
            }
            Bound::Included(idx) => idx + 1,
            Bound::Excluded(idx) => idx,
            Bound::Unbounded => len,
        };

        if start_idx >= end_idx {
            write!(f, "<empty range>")?;
            return Ok(());
        }

        // This can't really fail because of the above
        let (memory_range, skip) = self
            .buffer
            .find_memory(start_idx..)
            .expect("can't find memory");

        let chunks = BufferChunked16Iter {
            buffer: self.buffer,
            mem_idx: memory_range.start,
            mem_len: n_memory,
            map: None,
            map_offset: skip,
            len: end_idx - start_idx,
        };

        if debug {
            for (line, line_len) in chunks {
                let line = &line[..line_len];

                match end_idx {
                    0x00_00..=0xff_ff => write!(f, "{:04x}:  ", start_idx)?,
                    0x01_00_00..=0xff_ff_ff => write!(f, "{:06x}:  ", start_idx)?,
                    0x01_00_00_00..=0xff_ff_ff_ff => write!(f, "{:08x}:  ", start_idx)?,
                    _ => write!(f, "{:016x}:  ", start_idx)?,
                }

                for (i, v) in line.iter().enumerate() {
                    if i > 0 {
                        write!(f, " {:02x}", v)?;
                    } else {
                        write!(f, "{:02x}", v)?;
                    }
                }

                for _ in line.len()..16 {
                    write!(f, "   ")?;
                }
                write!(f, "   ")?;

                for v in line {
                    if v.is_ascii() && !v.is_ascii_control() {
                        f.write_char((*v).into())?;
                    } else {
                        f.write_char('.')?;
                    }
                }

                start_idx = start_idx.saturating_add(16);
                if start_idx < end_idx {
                    writeln!(f)?;
                }
            }

            Ok(())
        } else {
            for (line, line_len) in chunks {
                let line = &line[..line_len];

                for (i, v) in line.iter().enumerate() {
                    if i > 0 {
                        write!(f, " {:02x}", v)?;
                    } else {
                        write!(f, "{:02x}", v)?;
                    }
                }

                start_idx = start_idx.saturating_add(16);
                if start_idx < end_idx {
                    writeln!(f)?;
                }
            }

            Ok(())
        }
    }
}

impl fmt::Display for Dump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f, false)
    }
}

impl fmt::Debug for Dump<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();

        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(ClockTime::NSECOND);
            buffer.set_dts(2 * ClockTime::NSECOND);
            buffer.set_offset(3);
            buffer.set_offset_end(4);
            buffer.set_duration(Some(5 * ClockTime::NSECOND));
        }
        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer.dts(), Some(2 * ClockTime::NSECOND));
        assert_eq!(buffer.offset(), 3);
        assert_eq!(buffer.offset_end(), 4);
        assert_eq!(buffer.duration(), Some(5 * ClockTime::NSECOND));
    }

    #[test]
    fn test_writability() {
        crate::init().unwrap();

        let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]);
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }
        assert_ne!(buffer.get_mut(), None);
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.set_pts(Some(ClockTime::NSECOND));
        }

        let mut buffer2 = buffer.clone();
        assert_eq!(buffer.get_mut(), None);

        assert_eq!(buffer2.as_ptr(), buffer.as_ptr());

        {
            let buffer2 = buffer2.make_mut();
            assert_ne!(buffer2.as_ptr(), buffer.as_ptr());

            buffer2.set_pts(Some(2 * ClockTime::NSECOND));

            let mut data = buffer2.map_writable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
            data.as_mut_slice()[0] = 0;
        }

        assert_eq!(buffer.pts(), Some(ClockTime::NSECOND));
        assert_eq!(buffer2.pts(), Some(2 * ClockTime::NSECOND));

        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());

            let data = buffer2.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![0, 2, 3, 4].as_slice());
        }
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_memories() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 5]));
            buffer.append_memory(crate::Memory::from_mut_slice(vec![0; 10]));
        }

        assert!(buffer.is_all_memory_writable());
        assert_eq!(buffer.n_memory(), 5);
        assert_eq!(buffer.size(), 30);

        for i in 0..5 {
            {
                let mem = buffer.memory(i).unwrap();
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                let mem = buffer.peek_memory(i);
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                let buffer = buffer.get_mut().unwrap();
                let mem = buffer.peek_memory_mut(i).unwrap();
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_writable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }
        }

        {
            let buffer = buffer.get_mut().unwrap();
            let mut last = 0;
            for (i, mem) in buffer.iter_memories_mut().unwrap().enumerate() {
                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_readable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                {
                    assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                    let map = mem.map_writable().unwrap();
                    assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
                }

                last = i;
            }

            assert_eq!(last, 4);
        }

        let mut last = 0;
        for (i, mem) in buffer.iter_memories().enumerate() {
            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);

        let mut last = 0;
        for (i, mem) in buffer.iter_memories_owned().enumerate() {
            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            {
                assert_eq!(mem.size(), if i < 4 { 5 } else { 10 });
                let map = mem.map_readable().unwrap();
                assert_eq!(map.size(), if i < 4 { 5 } else { 10 });
            }

            last = i;
        }

        assert_eq!(last, 4);
    }

    #[test]
    fn test_meta_foreach() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                ClockTime::ZERO,
                ClockTime::NONE,
            );
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                ClockTime::SECOND,
                ClockTime::NONE,
            );
        }

        let mut res = vec![];
        buffer.foreach_meta(|meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            ControlFlow::Continue(())
        });

        assert_eq!(&[ClockTime::ZERO, ClockTime::SECOND][..], &res[..]);
    }

    #[test]
    fn test_meta_foreach_mut() {
        crate::init().unwrap();

        let mut buffer = Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                ClockTime::ZERO,
                ClockTime::NONE,
            );
            crate::ReferenceTimestampMeta::add(
                buffer,
                &crate::Caps::builder("foo/bar").build(),
                ClockTime::SECOND,
                ClockTime::NONE,
            );
        }

        let mut res = vec![];
        buffer.get_mut().unwrap().foreach_meta_mut(|mut meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            if meta.timestamp() == ClockTime::SECOND {
                ControlFlow::Continue(BufferMetaForeachAction::Remove)
            } else {
                ControlFlow::Continue(BufferMetaForeachAction::Keep)
            }
        });

        assert_eq!(&[ClockTime::ZERO, ClockTime::SECOND][..], &res[..]);

        let mut res = vec![];
        buffer.foreach_meta(|meta| {
            let meta = meta
                .downcast_ref::<crate::ReferenceTimestampMeta>()
                .unwrap();
            res.push(meta.timestamp());
            ControlFlow::Continue(())
        });

        assert_eq!(&[ClockTime::ZERO][..], &res[..]);
    }

    #[test]
    fn test_ptr_eq() {
        crate::init().unwrap();

        let buffer1 = Buffer::new();
        assert!(BufferRef::ptr_eq(&buffer1, &buffer1));
        let buffer2 = Buffer::new();
        assert!(!BufferRef::ptr_eq(&buffer1, &buffer2));
    }

    #[test]
    fn test_copy_region() {
        crate::init().unwrap();

        let buffer1 = Buffer::from_mut_slice(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, ..).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 0..8).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 0..=7).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, ..=7).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, ..8).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 0..).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[0, 1, 2, 3, 4, 5, 6, 7]
        );

        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 0..=8).is_err());
        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 0..=10).is_err());
        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 8..=10).is_err());
        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 8..=8).is_err());
        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 10..).is_err());
        assert!(buffer1.copy_region(BUFFER_COPY_ALL, 10..100).is_err());

        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 2..4).unwrap();
        assert_eq!(buffer2.map_readable().unwrap().as_slice(), &[2, 3]);

        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 2..=4).unwrap();
        assert_eq!(buffer2.map_readable().unwrap().as_slice(), &[2, 3, 4]);

        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, 2..).unwrap();
        assert_eq!(
            buffer2.map_readable().unwrap().as_slice(),
            &[2, 3, 4, 5, 6, 7]
        );
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, ..2).unwrap();
        assert_eq!(buffer2.map_readable().unwrap().as_slice(), &[0, 1]);
        let buffer2 = buffer1.copy_region(BUFFER_COPY_ALL, ..=2).unwrap();
        assert_eq!(buffer2.map_readable().unwrap().as_slice(), &[0, 1, 2]);
    }

    #[test]
    fn test_dump() {
        use std::fmt::Write;

        crate::init().unwrap();

        let mut s = String::new();
        let buffer = crate::Buffer::from_slice(vec![1, 2, 3, 4]);
        write!(&mut s, "{:?}", buffer.dump()).unwrap();
        assert_eq!(
            s,
            "0000:  01 02 03 04                                       ...."
        );
        s.clear();
        write!(&mut s, "{}", buffer.dump()).unwrap();
        assert_eq!(s, "01 02 03 04");
        s.clear();

        let buffer = crate::Buffer::from_slice(vec![1, 2, 3, 4]);
        write!(&mut s, "{:?}", buffer.dump_range(..)).unwrap();
        assert_eq!(
            s,
            "0000:  01 02 03 04                                       ...."
        );
        s.clear();
        write!(&mut s, "{:?}", buffer.dump_range(..2)).unwrap();
        assert_eq!(
            s,
            "0000:  01 02                                             .."
        );
        s.clear();
        write!(&mut s, "{:?}", buffer.dump_range(2..=3)).unwrap();
        assert_eq!(
            s,
            "0002:  03 04                                             .."
        );
        s.clear();
        write!(&mut s, "{:?}", buffer.dump_range(..100)).unwrap();
        assert_eq!(s, "<end out of range>",);
        s.clear();
        write!(&mut s, "{:?}", buffer.dump_range(90..100)).unwrap();
        assert_eq!(s, "<start out of range>",);
        s.clear();

        let buffer = crate::Buffer::from_slice(vec![0; 19]);
        write!(&mut s, "{:?}", buffer.dump()).unwrap();
        assert_eq!(
            s,
            "0000:  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00   ................\n\
             0010:  00 00 00                                          ..."
        );
        s.clear();
    }

    #[test]
    fn test_dump_multi_memories() {
        use std::fmt::Write;

        crate::init().unwrap();

        let mut buffer = crate::Buffer::new();
        {
            let buffer = buffer.get_mut().unwrap();

            let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
            buffer.append_memory(mem);

            let mem = crate::Memory::from_slice(vec![5, 6, 7, 8]);
            buffer.append_memory(mem);

            let mem = crate::Memory::from_slice(vec![9, 10, 11, 12]);
            buffer.append_memory(mem);

            let mem = crate::Memory::from_slice(vec![13, 14, 15, 16]);
            buffer.append_memory(mem);

            let mem = crate::Memory::from_slice(vec![17, 18, 19]);
            buffer.append_memory(mem);
        }

        let mut s = String::new();
        write!(&mut s, "{:?}", buffer.dump()).unwrap();
        assert_eq!(
            s,
            "0000:  01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10   ................\n\
             0010:  11 12 13                                          ..."
        );
        s.clear();
        write!(&mut s, "{}", buffer.dump()).unwrap();
        assert_eq!(
            s,
            "01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10\n11 12 13"
        );
        s.clear();

        write!(&mut s, "{:?}", buffer.dump_range(2..)).unwrap();
        assert_eq!(
            s,
            "0002:  03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f 10 11 12   ................\n\
             0012:  13                                                ."
        );
        s.clear();

        write!(&mut s, "{:?}", buffer.dump_range(14..17)).unwrap();
        assert_eq!(
            s,
            "000e:  0f 10 11                                          ..."
        );
        s.clear();

        write!(&mut s, "{:?}", buffer.dump_range(14..20)).unwrap();
        assert_eq!(s, "<end out of range>");
        s.clear();

        #[allow(clippy::reversed_empty_ranges)]
        {
            write!(&mut s, "{:?}", buffer.dump_range(23..20)).unwrap();
            assert_eq!(s, "<start out of range>");
            s.clear();
        }
    }
}
