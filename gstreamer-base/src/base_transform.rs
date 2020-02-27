// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_base_sys;
use BaseTransform;

pub trait BaseTransformExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<BaseTransform>> BaseTransformExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let trans: &gst_base_sys::GstBaseTransform = &*(self.as_ptr() as *const _);
            let _guard = ::utils::MutexGuard::lock(&trans.element.object.lock);
            from_glib_none(&trans.segment as *const _)
        }
    }
}
