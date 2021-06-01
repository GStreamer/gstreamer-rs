// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;
use std::cmp;
use std::fmt;
use std::mem;
use std::ptr;
use std::str;

#[derive(Clone)]
#[doc(alias = "GstVideoTimeCodeInterval")]
pub struct VideoTimeCodeInterval(ffi::GstVideoTimeCodeInterval);

impl VideoTimeCodeInterval {
    pub fn new(hours: u32, minutes: u32, seconds: u32, frames: u32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut v = mem::MaybeUninit::zeroed();
            ffi::gst_video_time_code_interval_init(v.as_mut_ptr(), hours, minutes, seconds, frames);
            Self(v.assume_init())
        }
    }

    #[doc(alias = "get_hours")]
    pub fn hours(&self) -> u32 {
        self.0.hours
    }

    pub fn set_hours(&mut self, hours: u32) {
        self.0.hours = hours
    }

    #[doc(alias = "get_minutes")]
    pub fn minutes(&self) -> u32 {
        self.0.minutes
    }

    pub fn set_minutes(&mut self, minutes: u32) {
        assert!(minutes < 60);
        self.0.minutes = minutes
    }

    #[doc(alias = "get_seconds")]
    pub fn seconds(&self) -> u32 {
        self.0.seconds
    }

    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.0.seconds = seconds
    }

    #[doc(alias = "get_frames")]
    pub fn frames(&self) -> u32 {
        self.0.frames
    }

    pub fn set_frames(&mut self, frames: u32) {
        self.0.frames = frames
    }
}

unsafe impl Send for VideoTimeCodeInterval {}
unsafe impl Sync for VideoTimeCodeInterval {}

impl PartialEq for VideoTimeCodeInterval {
    fn eq(&self, other: &Self) -> bool {
        self.0.hours == other.0.hours
            && self.0.minutes == other.0.minutes
            && self.0.seconds == other.0.seconds
            && self.0.frames == other.0.frames
    }
}

impl Eq for VideoTimeCodeInterval {}

impl PartialOrd for VideoTimeCodeInterval {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VideoTimeCodeInterval {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0
            .hours
            .cmp(&other.0.hours)
            .then_with(|| self.0.minutes.cmp(&other.0.minutes))
            .then_with(|| self.0.seconds.cmp(&other.0.seconds))
            .then_with(|| self.0.frames.cmp(&other.0.frames))
    }
}

impl fmt::Debug for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoTimeCodeInterval")
            .field("hours", &self.0.hours)
            .field("minutes", &self.0.minutes)
            .field("seconds", &self.0.seconds)
            .field("frames", &self.0.frames)
            .finish()
    }
}

impl fmt::Display for VideoTimeCodeInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.0.hours, self.0.minutes, self.0.seconds, self.0.frames
        )
    }
}

impl str::FromStr for VideoTimeCodeInterval {
    type Err = glib::error::BoolError;

    #[doc(alias = "gst_video_time_code_interval_new_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<Self>::from_glib_full(ffi::gst_video_time_code_interval_new_from_string(
                s.to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create VideoTimeCodeInterval from string"))
        }
    }
}

#[doc(hidden)]
impl GlibPtrDefault for VideoTimeCodeInterval {
    type GlibType = *mut ffi::GstVideoTimeCodeInterval;
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstVideoTimeCodeInterval, Self> {
        Stash(&self.0 as *const _, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const ffi::GstVideoTimeCodeInterval {
        unsafe { ffi::gst_video_time_code_interval_copy(&self.0 as *const _) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GstVideoTimeCodeInterval, Self> {
        let ptr = &mut self.0 as *mut _;
        StashMut(ptr, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        Self(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        Self(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Self {
        assert!(!ptr.is_null());
        let res = Self(ptr::read(ptr));
        ffi::gst_video_time_code_interval_free(ptr);

        res
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GstVideoTimeCodeInterval> for VideoTimeCodeInterval {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstVideoTimeCodeInterval) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(Self(ptr::read(ptr)))
    }
}

impl StaticType for VideoTimeCodeInterval {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_interval_get_type()) }
    }
}

impl glib::value::ValueType for VideoTimeCodeInterval {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for VideoTimeCodeInterval {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstVideoTimeCodeInterval)
    }
}

#[doc(hidden)]
impl glib::value::ToValue for VideoTimeCodeInterval {
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

#[doc(hidden)]
impl glib::value::ToValueOptional for VideoTimeCodeInterval {
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
