// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlibPtr};
use glib_ffi;
use std::fmt;

use miniobject::*;
use Buffer;
use BufferRef;

gst_define_mini_object_wrapper!(
    BufferList,
    BufferListRef,
    ffi::GstBufferList,
    [Debug,],
    || ffi::gst_buffer_list_get_type()
);

impl BufferList {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new()) }
    }

    pub fn new_sized(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new_sized(size as u32)) }
    }
}

impl BufferListRef {
    pub fn insert(&mut self, idx: i32, buffer: Buffer) {
        unsafe {
            ffi::gst_buffer_list_insert(self.as_mut_ptr(), idx, buffer.into_ptr());
        }
    }

    pub fn add(&mut self, buffer: Buffer) {
        self.insert(-1, buffer);
    }

    pub fn copy_deep(&self) -> BufferList {
        unsafe { from_glib_full(ffi::gst_buffer_list_copy_deep(self.as_ptr())) }
    }

    pub fn remove(&mut self, idx: u32, len: u32) {
        unsafe { ffi::gst_buffer_list_remove(self.as_mut_ptr(), idx, len) }
    }

    pub fn get(&self, idx: u32) -> Option<&BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_writable(&mut self, idx: u32) -> Option<&mut BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get_writable(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::gst_buffer_list_length(self.as_mut_ptr()) as usize }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn calculate_size(&self) -> usize {
        unsafe { ffi::gst_buffer_list_calculate_size(self.as_mut_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl Default for BufferList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BufferListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.iter().map(|b| b.get_size()).sum::<usize>();
        let (pts, dts) = self
            .get(0)
            .map(|b| (b.get_pts(), b.get_dts()))
            .unwrap_or((::ClockTime::none(), ::ClockTime::none()));

        f.debug_struct("BufferList")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("buffers", &self.len())
            .field("pts", &pts.to_string())
            .field("dts", &dts.to_string())
            .field("size", &size)
            .finish()
    }
}

pub struct Iter<'a> {
    list: &'a BufferListRef,
    idx: u32,
    size: u32,
}

impl<'a> Iter<'a> {
    fn new(list: &'a BufferListRef) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            list,
            idx: 0,
            size: list.len() as u32,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a BufferRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.list.get(self.idx);
        self.idx += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.list.get(self.size)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}
