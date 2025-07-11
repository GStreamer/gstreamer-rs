// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    marker::PhantomData,
    mem, ops,
    ops::{Deref, DerefMut},
    ptr,
};

use glib::{prelude::*, translate::*};

use crate::{ffi, AllocationParams, Allocator, BufferPool, Structure, StructureRef};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct BufferPoolConfig(Structure);

impl Deref for BufferPoolConfig {
    type Target = BufferPoolConfigRef;

    #[inline]
    fn deref(&self) -> &BufferPoolConfigRef {
        unsafe { &*(self.0.as_ptr() as *const StructureRef as *const BufferPoolConfigRef) }
    }
}

impl DerefMut for BufferPoolConfig {
    #[inline]
    fn deref_mut(&mut self) -> &mut BufferPoolConfigRef {
        unsafe { &mut *(self.0.as_ptr() as *mut StructureRef as *mut BufferPoolConfigRef) }
    }
}

impl AsRef<BufferPoolConfigRef> for BufferPoolConfig {
    #[inline]
    fn as_ref(&self) -> &BufferPoolConfigRef {
        self.deref()
    }
}

impl AsMut<BufferPoolConfigRef> for BufferPoolConfig {
    #[inline]
    fn as_mut(&mut self) -> &mut BufferPoolConfigRef {
        self.deref_mut()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct BufferPoolConfigRef(StructureRef);

impl BufferPoolConfigRef {
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstStructure) -> &'a BufferPoolConfigRef {
        debug_assert!(!ptr.is_null());

        &*(ptr as *mut StructureRef as *mut BufferPoolConfigRef)
    }

    #[inline]
    pub unsafe fn from_glib_borrow_mut<'a>(
        ptr: *mut ffi::GstStructure,
    ) -> &'a mut BufferPoolConfigRef {
        debug_assert!(!ptr.is_null());

        &mut *(ptr as *mut StructureRef as *mut BufferPoolConfigRef)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstStructure {
        self as *const Self as *const ffi::GstStructure
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstStructure {
        self as *const Self as *mut ffi::GstStructure
    }
}

impl ops::Deref for BufferPoolConfigRef {
    type Target = crate::StructureRef;

    #[inline]
    fn deref(&self) -> &crate::StructureRef {
        &self.0
    }
}

impl ops::DerefMut for BufferPoolConfigRef {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::StructureRef {
        &mut self.0
    }
}

impl AsRef<crate::StructureRef> for BufferPoolConfigRef {
    #[inline]
    fn as_ref(&self) -> &crate::StructureRef {
        &self.0
    }
}

impl AsMut<crate::StructureRef> for BufferPoolConfigRef {
    #[inline]
    fn as_mut(&mut self) -> &mut crate::StructureRef {
        &mut self.0
    }
}

impl BufferPoolConfigRef {
    #[doc(alias = "gst_buffer_pool_config_add_option")]
    pub fn add_option(&mut self, option: &str) {
        unsafe {
            ffi::gst_buffer_pool_config_add_option(self.0.as_mut_ptr(), option.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_buffer_pool_config_has_option")]
    pub fn has_option(&self, option: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_buffer_pool_config_has_option(
                self.0.as_mut_ptr(),
                option.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_options")]
    #[doc(alias = "gst_buffer_pool_config_n_options")]
    #[doc(alias = "gst_buffer_pool_config_get_option")]
    pub fn options(&self) -> OptionsIter<'_> {
        OptionsIter::new(self)
    }

    #[doc(alias = "gst_buffer_pool_config_set_params")]
    pub fn set_params(
        &mut self,
        caps: Option<&crate::Caps>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) {
        unsafe {
            ffi::gst_buffer_pool_config_set_params(
                self.0.as_mut_ptr(),
                caps.to_glib_none().0,
                size,
                min_buffers,
                max_buffers,
            );
        }
    }

    #[doc(alias = "get_params")]
    #[doc(alias = "gst_buffer_pool_config_get_params")]
    pub fn params(&self) -> Option<(Option<crate::Caps>, u32, u32, u32)> {
        unsafe {
            let mut caps = ptr::null_mut();
            let mut size = mem::MaybeUninit::uninit();
            let mut min_buffers = mem::MaybeUninit::uninit();
            let mut max_buffers = mem::MaybeUninit::uninit();

            let ret: bool = from_glib(ffi::gst_buffer_pool_config_get_params(
                self.0.as_mut_ptr(),
                &mut caps,
                size.as_mut_ptr(),
                min_buffers.as_mut_ptr(),
                max_buffers.as_mut_ptr(),
            ));
            if !ret {
                return None;
            }

            Some((
                from_glib_none(caps),
                size.assume_init(),
                min_buffers.assume_init(),
                max_buffers.assume_init(),
            ))
        }
    }

    #[doc(alias = "gst_buffer_pool_config_validate_params")]
    pub fn validate_params(
        &self,
        caps: Option<&crate::Caps>,
        size: u32,
        min_buffers: u32,
        max_buffers: u32,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_config_validate_params(
                    self.0.as_mut_ptr(),
                    caps.to_glib_none().0,
                    size,
                    min_buffers,
                    max_buffers,
                ),
                "Parameters are not valid in this context"
            )
        }
    }

    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_buffer_pool_config_get_allocator")]
    pub fn allocator(&self) -> Option<(Option<Allocator>, AllocationParams)> {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_buffer_pool_config_get_allocator(
                self.0.as_mut_ptr(),
                &mut allocator,
                params.as_mut_ptr(),
            ));
            if ret {
                Some((from_glib_none(allocator), params.assume_init().into()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_buffer_pool_config_set_allocator")]
    pub fn set_allocator(
        &self,
        allocator: Option<&impl IsA<Allocator>>,
        params: Option<&AllocationParams>,
    ) {
        assert!(allocator.is_some() || params.is_some());
        unsafe {
            ffi::gst_buffer_pool_config_set_allocator(
                self.0.as_mut_ptr(),
                allocator.to_glib_none().0 as *mut ffi::GstAllocator,
                match params {
                    Some(val) => val.as_ptr(),
                    None => ptr::null(),
                },
            )
        }
    }
}

crate::utils::define_fixed_size_iter!(
    OptionsIter,
    &'a BufferPoolConfigRef,
    &'a glib::GStr,
    |collection: &BufferPoolConfigRef| unsafe {
        ffi::gst_buffer_pool_config_n_options(collection.as_mut_ptr()) as usize
    },
    |collection: &BufferPoolConfigRef, idx: usize| unsafe {
        glib::GStr::from_ptr(ffi::gst_buffer_pool_config_get_option(
            collection.as_mut_ptr(),
            idx as u32,
        ))
    }
);

#[derive(Debug, Copy, Clone)]
#[doc(alias = "GstBufferPoolAcquireParams")]
pub struct BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams);

unsafe impl Send for BufferPoolAcquireParams {}
unsafe impl Sync for BufferPoolAcquireParams {}

impl BufferPoolAcquireParams {
    pub fn with_flags(flags: crate::BufferPoolAcquireFlags) -> Self {
        skip_assert_initialized!();
        BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
            format: ffi::GST_FORMAT_UNDEFINED,
            start: -1,
            stop: -1,
            flags: flags.into_glib(),
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }

    pub fn with_start_stop<T: crate::format::SpecificFormattedValue>(
        start: T,
        stop: T,
        flags: crate::BufferPoolAcquireFlags,
    ) -> Self {
        skip_assert_initialized!();
        unsafe {
            BufferPoolAcquireParams(ffi::GstBufferPoolAcquireParams {
                format: start.format().into_glib(),
                start: start.into_raw_value(),
                stop: stop.into_raw_value(),
                flags: flags.into_glib(),
                _gst_reserved: [ptr::null_mut(); 4],
            })
        }
    }

    pub fn flags(&self) -> crate::BufferPoolAcquireFlags {
        unsafe { from_glib(self.0.flags) }
    }

    pub fn format(&self) -> crate::Format {
        unsafe { from_glib(self.0.format) }
    }

    pub fn start(&self) -> crate::GenericFormattedValue {
        unsafe { crate::GenericFormattedValue::new(from_glib(self.0.format), self.0.start) }
    }

    pub fn stop(&self) -> crate::GenericFormattedValue {
        unsafe { crate::GenericFormattedValue::new(from_glib(self.0.format), self.0.stop) }
    }

    pub fn set_flags(&mut self, flags: crate::BufferPoolAcquireFlags) {
        self.0.flags = flags.into_glib();
    }

    pub fn set_format(&mut self, format: crate::Format) {
        self.0.format = format.into_glib();
    }

    pub fn set_start(&mut self, start: crate::GenericFormattedValue) {
        assert_eq!(self.format(), start.format());
        self.0.start = start.value();
    }

    pub fn set_stop(&mut self, stop: crate::GenericFormattedValue) {
        assert_eq!(self.format(), stop.format());
        self.0.stop = stop.value();
    }
}

impl PartialEq for BufferPoolAcquireParams {
    fn eq(&self, other: &Self) -> bool {
        self.flags() == other.flags()
            && self.format() == other.format()
            && self.start() == other.start()
            && self.stop() == other.stop()
    }
}

impl Eq for BufferPoolAcquireParams {}

impl Default for BufferPoolAcquireParams {
    fn default() -> Self {
        Self(ffi::GstBufferPoolAcquireParams {
            format: ffi::GST_FORMAT_UNDEFINED,
            start: -1,
            stop: -1,
            flags: ffi::GST_BUFFER_POOL_ACQUIRE_FLAG_NONE,
            _gst_reserved: [ptr::null_mut(); 4],
        })
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstBufferPoolAcquireParams> for BufferPoolAcquireParams {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(
        &'a self,
    ) -> glib::translate::Stash<'a, *const ffi::GstBufferPoolAcquireParams, Self> {
        glib::translate::Stash(&self.0, PhantomData)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GstBufferPoolAcquireParams> for BufferPoolAcquireParams {
    type Storage = PhantomData<&'a mut Self>;

    #[inline]
    fn to_glib_none_mut(
        &'a mut self,
    ) -> glib::translate::StashMut<'a, *mut ffi::GstBufferPoolAcquireParams, Self> {
        glib::translate::StashMut(&mut self.0, PhantomData)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GstBufferPoolAcquireParams> for BufferPoolAcquireParams {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstBufferPoolAcquireParams) -> Self {
        Self(*ptr)
    }
}

pub trait BufferPoolExtManual: IsA<BufferPool> + 'static {
    #[doc(alias = "get_config")]
    #[doc(alias = "gst_buffer_pool_get_config")]
    fn config(&self) -> BufferPoolConfig {
        unsafe {
            let ptr = ffi::gst_buffer_pool_get_config(self.as_ref().to_glib_none().0);
            BufferPoolConfig(from_glib_full(ptr))
        }
    }

    #[doc(alias = "gst_buffer_pool_set_config")]
    fn set_config(&self, config: BufferPoolConfig) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_buffer_pool_set_config(
                    self.as_ref().to_glib_none().0,
                    config.0.into_glib_ptr()
                ),
                "Failed to set config",
            )
        }
    }

    fn is_flushing(&self) -> bool {
        unsafe {
            let stash = self.as_ref().to_glib_none();
            let ptr: *mut ffi::GstBufferPool = stash.0;

            from_glib((*ptr).flushing)
        }
    }

    #[doc(alias = "gst_buffer_pool_acquire_buffer")]
    fn acquire_buffer(
        &self,
        params: Option<&BufferPoolAcquireParams>,
    ) -> Result<crate::Buffer, crate::FlowError> {
        let params_ptr = params.to_glib_none().0 as *mut _;

        unsafe {
            let mut buffer = ptr::null_mut();
            crate::FlowSuccess::try_from_glib(ffi::gst_buffer_pool_acquire_buffer(
                self.as_ref().to_glib_none().0,
                &mut buffer,
                params_ptr,
            ))
            .map(|_| from_glib_full(buffer))
        }
    }
}

impl<O: IsA<BufferPool>> BufferPoolExtManual for O {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn pool_with_params() {
        crate::init().unwrap();

        let pool = crate::BufferPool::new();
        let mut config = pool.config();
        config.set_params(Some(&crate::Caps::builder("foo/bar").build()), 1024, 0, 2);
        pool.set_config(config).unwrap();

        pool.set_active(true).unwrap();

        let params =
            crate::BufferPoolAcquireParams::with_flags(crate::BufferPoolAcquireFlags::DONTWAIT);

        let _buf1 = pool.acquire_buffer(Some(&params)).unwrap();
        let buf2 = pool.acquire_buffer(Some(&params)).unwrap();

        assert!(pool.acquire_buffer(Some(&params)).is_err());

        drop(buf2);
        let _buf2 = pool.acquire_buffer(Some(&params)).unwrap();

        pool.set_active(false).unwrap();
    }

    #[test]
    fn pool_no_params() {
        crate::init().unwrap();

        let pool = crate::BufferPool::new();
        let mut config = pool.config();
        config.set_params(None, 1024, 0, 2);
        pool.set_config(config).unwrap();

        pool.set_active(true).unwrap();
        let _buf1 = pool.acquire_buffer(None).unwrap();
        pool.set_active(false).unwrap();
    }
}
