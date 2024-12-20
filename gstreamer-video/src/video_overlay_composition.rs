// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, mem};

use crate::ffi;
use glib::translate::*;

gst::mini_object_wrapper!(
    VideoOverlayRectangle,
    VideoOverlayRectangleRef,
    ffi::GstVideoOverlayRectangle,
    || ffi::gst_video_overlay_rectangle_get_type()
);

impl fmt::Debug for VideoOverlayRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        VideoOverlayRectangleRef::fmt(self, f)
    }
}

impl fmt::Debug for VideoOverlayRectangleRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayRectangle")
            .field("flags", &self.flags())
            .field("global_alpha", &self.global_alpha())
            .field("render_rectangle", &self.render_rectangle())
            .finish()
    }
}

impl VideoOverlayRectangle {
    #[doc(alias = "gst_video_overlay_rectangle_new_raw")]
    pub fn new_raw(
        buffer: &gst::Buffer,
        render_x: i32,
        render_y: i32,
        render_width: u32,
        render_height: u32,
        flags: crate::VideoOverlayFormatFlags,
    ) -> Self {
        skip_assert_initialized!();
        assert!(buffer.meta::<crate::VideoMeta>().is_some());
        unsafe {
            from_glib_full(ffi::gst_video_overlay_rectangle_new_raw(
                buffer.to_glib_none().0,
                render_x,
                render_y,
                render_width,
                render_height,
                flags.into_glib(),
            ))
        }
    }
}

impl VideoOverlayRectangleRef {
    #[doc(alias = "get_flags")]
    #[doc(alias = "gst_video_overlay_rectangle_get_flags")]
    pub fn flags(&self) -> crate::VideoOverlayFormatFlags {
        unsafe {
            from_glib(ffi::gst_video_overlay_rectangle_get_flags(
                self.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "get_global_alpha")]
    #[doc(alias = "gst_video_overlay_rectangle_get_global_alpha")]
    pub fn global_alpha(&self) -> f32 {
        unsafe { ffi::gst_video_overlay_rectangle_get_global_alpha(self.as_mut_ptr()) }
    }

    #[doc(alias = "gst_video_overlay_rectangle_set_global_alpha")]
    pub fn set_global_alpha(&mut self, alpha: f32) {
        unsafe { ffi::gst_video_overlay_rectangle_set_global_alpha(self.as_mut_ptr(), alpha) }
    }

    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_video_overlay_rectangle_get_seqnum")]
    pub fn seqnum(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_rectangle_get_seqnum(self.as_mut_ptr()) }
    }

    #[doc(alias = "get_render_rectangle")]
    #[doc(alias = "gst_video_overlay_rectangle_get_render_rectangle")]
    pub fn render_rectangle(&self) -> (i32, i32, u32, u32) {
        unsafe {
            let mut render_x = mem::MaybeUninit::uninit();
            let mut render_y = mem::MaybeUninit::uninit();
            let mut render_width = mem::MaybeUninit::uninit();
            let mut render_height = mem::MaybeUninit::uninit();

            ffi::gst_video_overlay_rectangle_get_render_rectangle(
                self.as_mut_ptr(),
                render_x.as_mut_ptr(),
                render_y.as_mut_ptr(),
                render_width.as_mut_ptr(),
                render_height.as_mut_ptr(),
            );

            (
                render_x.assume_init(),
                render_y.assume_init(),
                render_width.assume_init(),
                render_height.assume_init(),
            )
        }
    }

    #[doc(alias = "gst_video_overlay_rectangle_set_render_rectangle")]
    pub fn set_render_rectangle(
        &mut self,
        render_x: i32,
        render_y: i32,
        render_width: u32,
        render_height: u32,
    ) {
        unsafe {
            ffi::gst_video_overlay_rectangle_set_render_rectangle(
                self.as_mut_ptr(),
                render_x,
                render_y,
                render_width,
                render_height,
            )
        }
    }

    #[doc(alias = "get_pixels_unscaled_raw")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_unscaled_raw")]
    pub fn pixels_unscaled_raw(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_raw(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_pixels_unscaled_ayuv")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_unscaled_ayuv")]
    pub fn pixels_unscaled_ayuv(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_ayuv(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_pixels_unscaled_argb")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_unscaled_argb")]
    pub fn pixels_unscaled_argb(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_argb(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_pixels_raw")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_raw")]
    pub fn pixels_raw(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_raw(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_pixels_ayuv")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_ayuv")]
    pub fn pixels_ayuv(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_ayuv(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_pixels_argb")]
    #[doc(alias = "gst_video_overlay_rectangle_get_pixels_argb")]
    pub fn pixels_argb(&self, flags: crate::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_argb(
                self.as_mut_ptr(),
                flags.into_glib(),
            ))
        }
    }
}

gst::mini_object_wrapper!(
    VideoOverlayComposition,
    VideoOverlayCompositionRef,
    ffi::GstVideoOverlayComposition,
    || ffi::gst_video_overlay_composition_get_type()
);

impl fmt::Debug for VideoOverlayComposition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        VideoOverlayCompositionRef::fmt(self, f)
    }
}

impl fmt::Debug for VideoOverlayCompositionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayComposition").finish()
    }
}

impl VideoOverlayComposition {
    #[doc(alias = "gst_video_overlay_composition_new")]
    pub fn new<'a>(
        rects: impl IntoIterator<Item = &'a VideoOverlayRectangle>,
    ) -> Result<Self, glib::error::BoolError> {
        assert_initialized_main_thread!();

        #[cfg(feature = "v1_20")]
        unsafe {
            use std::ptr;

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut()));

            rects.into_iter().for_each(|rect| {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.as_mut_ptr(),
                );
            });

            Ok(composition)
        }
        #[cfg(not(feature = "v1_20"))]
        unsafe {
            let mut iter = rects.into_iter();

            let first = match iter.next() {
                None => {
                    return Err(glib::bool_error!(
                        "Failed to create VideoOverlayComposition"
                    ))
                }
                Some(first) => first,
            };

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(first.as_mut_ptr()));

            for rect in iter {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.as_mut_ptr(),
                );
            }

            Ok(composition)
        }
    }
}

#[cfg(feature = "v1_20")]
impl Default for VideoOverlayComposition {
    fn default() -> Self {
        assert_initialized_main_thread!();

        use std::ptr;

        unsafe { from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut())) }
    }
}

impl VideoOverlayCompositionRef {
    #[doc(alias = "gst_video_overlay_composition_n_rectangles")]
    pub fn n_rectangles(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_composition_n_rectangles(self.as_mut_ptr()) }
    }

    #[doc(alias = "get_rectangle")]
    #[doc(alias = "gst_video_overlay_composition_get_rectangle")]
    pub fn rectangle(&self, idx: u32) -> Result<VideoOverlayRectangle, glib::error::BoolError> {
        if idx >= self.n_rectangles() {
            return Err(glib::bool_error!("Invalid index"));
        }

        unsafe {
            match from_glib_none(ffi::gst_video_overlay_composition_get_rectangle(
                self.as_mut_ptr(),
                idx,
            )) {
                Some(r) => Ok(r),
                None => Err(glib::bool_error!("Failed to get rectangle")),
            }
        }
    }

    #[doc(alias = "gst_video_overlay_composition_add_rectangle")]
    pub fn add_rectangle(&mut self, rect: &VideoOverlayRectangleRef) {
        unsafe {
            ffi::gst_video_overlay_composition_add_rectangle(self.as_mut_ptr(), rect.as_mut_ptr());
        }
    }

    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_video_overlay_composition_get_seqnum")]
    pub fn seqnum(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_composition_get_seqnum(self.as_mut_ptr()) }
    }

    #[doc(alias = "gst_video_overlay_composition_blend")]
    pub fn blend(
        &self,
        frame: &mut crate::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_overlay_composition_blend(self.as_mut_ptr(), frame.as_mut_ptr()),
                "Failed to blend overlay composition",
            )
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            composition: self,
            idx: 0,
            len: self.n_rectangles() as usize,
        }
    }
}

impl<'a> IntoIterator for &'a VideoOverlayComposition {
    type IntoIter = Iter<'a>;
    type Item = VideoOverlayRectangle;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<VideoOverlayRectangle> for VideoOverlayComposition {
    fn from(value: VideoOverlayRectangle) -> Self {
        skip_assert_initialized!();

        unsafe {
            Self::from_glib_full(ffi::gst_video_overlay_composition_new(
                value.into_glib_ptr(),
            ))
        }
    }
}

impl<'a> From<&'a VideoOverlayRectangle> for VideoOverlayComposition {
    fn from(value: &'a VideoOverlayRectangle) -> Self {
        skip_assert_initialized!();

        unsafe { Self::from_glib_full(ffi::gst_video_overlay_composition_new(value.as_mut_ptr())) }
    }
}

#[cfg(feature = "v1_20")]
impl<const N: usize> From<[VideoOverlayRectangle; N]> for VideoOverlayComposition {
    fn from(value: [VideoOverlayRectangle; N]) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            use std::ptr;

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut()));

            value.into_iter().for_each(|rect| {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.into_glib_ptr(),
                );
            });

            composition
        }
    }
}

#[cfg(feature = "v1_20")]
impl<'a, const N: usize> From<[&'a VideoOverlayRectangle; N]> for VideoOverlayComposition {
    fn from(value: [&'a VideoOverlayRectangle; N]) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            use std::ptr;

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut()));

            value.into_iter().for_each(|rect| {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.as_mut_ptr(),
                );
            });

            composition
        }
    }
}

#[cfg(feature = "v1_20")]
impl std::iter::FromIterator<VideoOverlayRectangle> for VideoOverlayComposition {
    fn from_iter<T: IntoIterator<Item = VideoOverlayRectangle>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            use std::ptr;

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut()));

            iter.into_iter().for_each(|rect| {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.into_glib_ptr(),
                );
            });

            composition
        }
    }
}

#[cfg(feature = "v1_20")]
impl<'a> std::iter::FromIterator<&'a VideoOverlayRectangle> for VideoOverlayComposition {
    fn from_iter<T: IntoIterator<Item = &'a VideoOverlayRectangle>>(iter: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            use std::ptr;

            let composition =
                Self::from_glib_full(ffi::gst_video_overlay_composition_new(ptr::null_mut()));

            iter.into_iter().for_each(|rect| {
                ffi::gst_video_overlay_composition_add_rectangle(
                    composition.as_mut_ptr(),
                    rect.as_mut_ptr(),
                );
            });

            composition
        }
    }
}

pub struct Iter<'a> {
    composition: &'a VideoOverlayCompositionRef,
    idx: usize,
    len: usize,
}

impl Iterator for Iter<'_> {
    type Item = VideoOverlayRectangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            return None;
        }

        let rect = self.composition.rectangle(self.idx as u32).unwrap();
        self.idx += 1;

        Some(rect)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.len - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.len || overflow {
            self.idx = self.len;
            None
        } else {
            self.idx = end + 1;
            Some(self.composition.rectangle(end as u32).unwrap())
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.len {
            None
        } else {
            Some(self.composition.rectangle(self.len as u32 - 1).unwrap())
        }
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.len {
            return None;
        }

        self.len -= 1;

        Some(self.composition.rectangle(self.len as u32).unwrap())
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.len.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.len;
            None
        } else {
            self.len = end - 1;
            Some(self.composition.rectangle(self.len as u32).unwrap())
        }
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl std::iter::FusedIterator for Iter<'_> {}
