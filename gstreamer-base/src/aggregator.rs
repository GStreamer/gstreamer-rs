// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib::{IsA, IsClassFor};
use gst;
use Aggregator;

use std::ops;

pub trait AggregatorExtManual: 'static {
    fn finish_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn;
}

impl<O: IsA<Aggregator>> AggregatorExtManual for O {
    fn finish_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_aggregator_finish_buffer(
                self.to_glib_none().0,
                buffer.into_ptr(),
            ))
        }
    }
}

#[repr(C)]
pub struct AggregatorClass(ffi::GstAggregatorClass);

unsafe impl IsClassFor for AggregatorClass {
    type Instance = Aggregator;
}

unsafe impl Send for AggregatorClass {}
unsafe impl Sync for AggregatorClass {}

impl ops::Deref for AggregatorClass {
    type Target = gst::ElementClass;

    fn deref(&self) -> &Self::Target {
        self.upcast_ref()
    }
}

impl ops::DerefMut for AggregatorClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_ref_mut()
    }
}
