// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use gst;
use gst::prelude::*;
use libc::uintptr_t;
use VideoOverlay;

use glib::IsA;

pub trait VideoOverlayExtManual {
    unsafe fn set_window_handle(&self, handle: uintptr_t);
    unsafe fn got_window_handle(&self, handle: uintptr_t);
}

impl<O: IsA<VideoOverlay>> VideoOverlayExtManual for O {
    unsafe fn set_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_set_window_handle(self.to_glib_none().0, handle)
    }

    unsafe fn got_window_handle(&self, handle: uintptr_t) {
        ffi::gst_video_overlay_got_window_handle(self.to_glib_none().0, handle)
    }
}

pub fn is_video_overlay_prepare_window_handle_message(msg: &gst::MessageRef) -> bool {
    unsafe {
        from_glib(ffi::gst_is_video_overlay_prepare_window_handle_message(
            msg.as_mut_ptr(),
        ))
    }
}
