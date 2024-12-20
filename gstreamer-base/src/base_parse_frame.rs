// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, ptr};

use glib::translate::*;

use crate::{ffi, BaseParse, BaseParseFrameFlags};

pub struct BaseParseFrame<'a>(
    ptr::NonNull<ffi::GstBaseParseFrame>,
    PhantomData<&'a BaseParse>,
);

unsafe impl Send for BaseParseFrame<'_> {}
unsafe impl Sync for BaseParseFrame<'_> {}

#[derive(Debug)]
pub enum Overhead {
    None,
    Frame,
    Bytes(u32),
}

#[doc(hidden)]
impl IntoGlib for Overhead {
    type GlibType = i32;

    #[inline]
    fn into_glib(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Frame => -1,
            Self::Bytes(b) => i32::try_from(b).expect("overhead is higher than i32::MAX"),
        }
    }
}

impl FromGlib<i32> for Overhead {
    #[inline]
    unsafe fn from_glib(val: i32) -> Self {
        skip_assert_initialized!();
        match val {
            0 => Self::None,
            1 => Self::Frame,
            b if b > 0 => Self::Bytes(val as u32),
            _ => panic!("overheader is lower than -1"),
        }
    }
}

#[doc(hidden)]
impl<'a> ::glib::translate::ToGlibPtr<'a, *mut ffi::GstBaseParseFrame> for BaseParseFrame<'a> {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> ::glib::translate::Stash<'a, *mut ffi::GstBaseParseFrame, Self> {
        Stash(self.0.as_ptr(), PhantomData)
    }

    fn to_glib_full(&self) -> *mut ffi::GstBaseParseFrame {
        unimplemented!()
    }
}

impl fmt::Debug for BaseParseFrame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("BaseParseFrame");

        b.field("buffer", &self.buffer())
            .field("output_buffer", &self.output_buffer())
            .field("flags", &self.flags())
            .field("offset", &self.offset())
            .field("overhead", &self.overhead());

        b.finish()
    }
}

impl<'a> BaseParseFrame<'a> {
    #[inline]
    pub(crate) unsafe fn new(frame: *mut ffi::GstBaseParseFrame, _parse: &'a BaseParse) -> Self {
        skip_assert_initialized!();
        debug_assert!(!frame.is_null());
        Self(ptr::NonNull::new_unchecked(frame), PhantomData)
    }

    #[doc(alias = "get_buffer")]
    #[inline]
    pub fn buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_buffer_mut")]
    pub fn buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst::ffi::gst_mini_object_is_writable(
                    ptr as *const gst::ffi::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_output_buffer")]
    #[inline]
    pub fn output_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).out_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_output_buffer_mut")]
    pub fn output_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).out_buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst::ffi::gst_mini_object_is_writable(
                    ptr as *const gst::ffi::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn set_output_buffer(&mut self, output_buffer: gst::Buffer) {
        unsafe {
            assert!(output_buffer.is_writable());
            let prev = (*self.to_glib_none().0).out_buffer;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject);
            }

            (*self.to_glib_none().0).out_buffer = output_buffer.into_glib_ptr();
        }
    }

    #[doc(alias = "get_flags")]
    #[inline]
    pub fn flags(&self) -> BaseParseFrameFlags {
        let flags = unsafe { (*self.to_glib_none().0).flags };
        BaseParseFrameFlags::from_bits_truncate(flags)
    }

    #[inline]
    pub fn set_flags(&mut self, flags: BaseParseFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags |= flags.bits() }
    }

    #[inline]
    pub fn unset_flags(&mut self, flags: BaseParseFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags &= !flags.bits() }
    }

    #[doc(alias = "get_offset")]
    #[inline]
    pub fn offset(&self) -> u64 {
        unsafe { (*self.to_glib_none().0).offset }
    }

    #[doc(alias = "get_overhead")]
    #[inline]
    pub fn overhead(&self) -> Overhead {
        unsafe { from_glib((*self.to_glib_none().0).overhead) }
    }

    #[inline]
    pub fn set_overhead(&mut self, overhead: Overhead) {
        unsafe {
            (*self.to_glib_none().0).overhead = overhead.into_glib();
        }
    }
}
