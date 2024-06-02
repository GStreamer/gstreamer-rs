// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, mem, os::raw::c_char, ptr};

use crate::ffi;
use glib::translate::*;

#[repr(transparent)]
#[doc(alias = "GstSDPTime")]
pub struct SDPTime(pub(crate) ffi::GstSDPTime);

unsafe impl Send for SDPTime {}
unsafe impl Sync for SDPTime {}

impl SDPTime {
    pub fn new(start: &str, stop: &str, repeat: &[&str]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut time = mem::MaybeUninit::uninit();
            ffi::gst_sdp_time_set(
                time.as_mut_ptr(),
                start.to_glib_none().0,
                stop.to_glib_none().0,
                repeat.to_glib_none().0,
            );
            SDPTime(time.assume_init())
        }
    }

    pub fn start(&self) -> Option<&str> {
        unsafe {
            if self.0.start.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.start).to_str().unwrap())
            }
        }
    }

    pub fn stop(&self) -> Option<&str> {
        unsafe {
            if self.0.stop.is_null() {
                None
            } else {
                Some(CStr::from_ptr(self.0.stop).to_str().unwrap())
            }
        }
    }

    pub fn repeat(&self) -> Vec<&str> {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            if self.0.repeat.is_null() || (*self.0.repeat).data.is_null() {
                return vec![];
            }

            let arr = (*self.0.repeat).data as *const *const c_char;
            let len = (*self.0.repeat).len as usize;
            let mut vec = Vec::with_capacity(len);
            for i in 0..len {
                vec.push(CStr::from_ptr(*arr.add(i)).to_str().unwrap());
            }
            vec
        }
    }
}

impl Clone for SDPTime {
    fn clone(&self) -> Self {
        skip_assert_initialized!();
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let mut time = mem::MaybeUninit::uninit();
            ffi::gst_sdp_time_set(
                time.as_mut_ptr(),
                self.0.start,
                self.0.stop,
                if self.0.repeat.is_null() {
                    ptr::null_mut()
                } else {
                    (*self.0.repeat).data as *mut *const c_char
                },
            );
            SDPTime(time.assume_init())
        }
    }
}

impl Drop for SDPTime {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_sdp_time_clear(&mut self.0);
        }
    }
}

impl fmt::Debug for SDPTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SDPTime")
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("repeat", &self.repeat())
            .finish()
    }
}
