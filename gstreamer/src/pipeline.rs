// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

use std::ops;

use glib::IsClassFor;

#[repr(C)]
pub struct PipelineClass(ffi::GstPipelineClass);

unsafe impl IsClassFor for PipelineClass {
    type Instance = ::Pipeline;
}

unsafe impl Send for PipelineClass {}
unsafe impl Sync for PipelineClass {}

impl ops::Deref for PipelineClass {
    type Target = ::BinClass;

    fn deref(&self) -> &Self::Target {
        self.upcast_ref()
    }
}

impl ops::DerefMut for PipelineClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_ref_mut()
    }
}
