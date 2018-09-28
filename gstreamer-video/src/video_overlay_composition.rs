// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use ffi;
use gst;
use gst::miniobject::*;

use glib;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, ToGlib, ToGlibPtr,
};
use glib_ffi;

gst_define_mini_object_wrapper!(
    VideoOverlayRectangle,
    VideoOverlayRectangleRef,
    ffi::GstVideoOverlayRectangle,
    [Debug,],
    || ffi::gst_video_overlay_rectangle_get_type()
);

impl fmt::Debug for VideoOverlayRectangleRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayRectangle")
            .field("flags", &self.get_flags())
            .field("global_alpha", &self.get_global_alpha())
            .field("render_rectangle", &self.get_render_rectangle())
            .finish()
    }
}

impl VideoOverlayRectangle {
    pub fn new_raw(
        buffer: &gst::BufferRef,
        render_x: i32,
        render_y: i32,
        render_width: u32,
        render_height: u32,
        flags: ::VideoOverlayFormatFlags,
    ) -> Self {
        unsafe {
            from_glib_full(ffi::gst_video_overlay_rectangle_new_raw(
                buffer.as_mut_ptr(),
                render_x,
                render_y,
                render_width,
                render_height,
                flags.to_glib(),
            ))
        }
    }
}

impl VideoOverlayRectangleRef {
    pub fn get_flags(&self) -> ::VideoOverlayFormatFlags {
        unsafe {
            from_glib(ffi::gst_video_overlay_rectangle_get_flags(
                self.as_mut_ptr(),
            ))
        }
    }

    pub fn get_global_alpha(&self) -> f32 {
        unsafe { ffi::gst_video_overlay_rectangle_get_global_alpha(self.as_mut_ptr()) }
    }

    pub fn set_global_alpha(&mut self, alpha: f32) {
        unsafe { ffi::gst_video_overlay_rectangle_set_global_alpha(self.as_mut_ptr(), alpha) }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_rectangle_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn get_render_rectangle(&self) -> (i32, i32, u32, u32) {
        unsafe {
            let mut render_x = 0;
            let mut render_y = 0;
            let mut render_width = 0;
            let mut render_height = 0;

            ffi::gst_video_overlay_rectangle_get_render_rectangle(
                self.as_mut_ptr(),
                &mut render_x,
                &mut render_y,
                &mut render_width,
                &mut render_height,
            );

            (render_x, render_y, render_width, render_height)
        }
    }

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

    pub fn get_pixels_unscaled_raw(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_raw(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_unscaled_ayuv(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_ayuv(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_unscaled_argb(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_unscaled_argb(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_raw(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_raw(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_ayuv(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_ayuv(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }

    pub fn get_pixels_argb(&self, flags: ::VideoOverlayFormatFlags) -> gst::Buffer {
        unsafe {
            from_glib_none(ffi::gst_video_overlay_rectangle_get_pixels_argb(
                self.as_mut_ptr(),
                flags.to_glib(),
            ))
        }
    }
}

gst_define_mini_object_wrapper!(
    VideoOverlayComposition,
    VideoOverlayCompositionRef,
    ffi::GstVideoOverlayComposition,
    [Debug,],
    || ffi::gst_video_overlay_composition_get_type()
);

impl fmt::Debug for VideoOverlayCompositionRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayComposition").finish()
    }
}

impl VideoOverlayComposition {
    pub fn new(rect: &VideoOverlayRectangle) -> Self {
        unsafe { from_glib_full(ffi::gst_video_overlay_composition_new(rect.as_mut_ptr())) }
    }
}

impl VideoOverlayCompositionRef {
    pub fn add_rectangle(&mut self, rect: &VideoOverlayRectangle) {
        unsafe {
            ffi::gst_video_overlay_composition_add_rectangle(self.as_mut_ptr(), rect.as_mut_ptr());
        }
    }

    pub fn n_rectangles(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_composition_n_rectangles(self.as_mut_ptr()) }
    }

    pub fn get_rectangle(&self, idx: u32) -> Option<VideoOverlayRectangle> {
        if idx >= self.n_rectangles() {
            return None;
        }

        unsafe {
            from_glib_none(ffi::gst_video_overlay_composition_get_rectangle(
                self.as_mut_ptr(),
                idx,
            ))
        }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe { ffi::gst_video_overlay_composition_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn blend(
        &self,
        frame: &mut ::VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::BoolError::from_glib(
                ffi::gst_video_overlay_composition_blend(self.as_mut_ptr(), frame.as_mut_ptr()),
                "Failed to blend overlay composition",
            )
        }
    }
}
