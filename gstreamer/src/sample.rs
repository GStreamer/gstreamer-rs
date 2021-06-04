// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::ptr;

use glib::translate::{from_glib_full, from_glib_none, mut_override, ToGlibPtr};

use crate::Buffer;
use crate::BufferList;
use crate::BufferListRef;
use crate::BufferRef;
use crate::Caps;
use crate::CapsRef;
use crate::FormattedSegment;
use crate::FormattedValueIntrinsic;
use crate::Segment;
use crate::Structure;
use crate::StructureRef;

mini_object_wrapper!(Sample, SampleRef, ffi::GstSample, || {
    ffi::gst_sample_get_type()
});

#[derive(Debug, Clone)]
pub struct SampleBuilder<'a> {
    buffer: Option<&'a Buffer>,
    buffer_list: Option<&'a BufferList>,
    caps: Option<&'a Caps>,
    segment: Option<&'a Segment>,
    info: Option<Structure>,
}

impl<'a> SampleBuilder<'a> {
    pub fn buffer(self, buffer: &'a Buffer) -> Self {
        Self {
            buffer: Some(buffer),
            buffer_list: None,
            ..self
        }
    }

    pub fn buffer_list(self, buffer_list: &'a BufferList) -> Self {
        Self {
            buffer: None,
            buffer_list: Some(buffer_list),
            ..self
        }
    }

    pub fn caps(self, caps: &'a Caps) -> Self {
        Self {
            caps: Some(caps),
            ..self
        }
    }

    pub fn segment<F: FormattedValueIntrinsic>(self, segment: &'a FormattedSegment<F>) -> Self {
        Self {
            segment: Some(segment.upcast_ref()),
            ..self
        }
    }

    pub fn info(self, info: Structure) -> Self {
        Self {
            info: Some(info),
            ..self
        }
    }

    pub fn build(self) -> Sample {
        assert_initialized_main_thread!();

        unsafe {
            let info = self.info.map(|i| i.into_ptr()).unwrap_or(ptr::null_mut());

            let sample: Sample = from_glib_full(ffi::gst_sample_new(
                self.buffer.to_glib_none().0,
                self.caps.to_glib_none().0,
                self.segment.to_glib_none().0,
                mut_override(info),
            ));

            if let Some(buffer_list) = self.buffer_list {
                ffi::gst_sample_set_buffer_list(
                    sample.to_glib_none().0,
                    buffer_list.to_glib_none().0,
                );
            }

            sample
        }
    }
}

impl Sample {
    pub fn builder<'a>() -> SampleBuilder<'a> {
        SampleBuilder {
            buffer: None,
            buffer_list: None,
            caps: None,
            segment: None,
            info: None,
        }
    }
}

impl SampleRef {
    #[doc(alias = "get_buffer")]
    #[doc(alias = "gst_sample_get_buffer")]
    pub fn buffer(&self) -> Option<&BufferRef> {
        unsafe {
            let ptr = ffi::gst_sample_get_buffer(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_buffer_owned")]
    pub fn buffer_owned(&self) -> Option<Buffer> {
        unsafe { self.buffer().map(|buffer| from_glib_none(buffer.as_ptr())) }
    }

    #[doc(alias = "get_buffer_list")]
    #[doc(alias = "gst_sample_get_buffer_list")]
    pub fn buffer_list(&self) -> Option<&BufferListRef> {
        unsafe {
            let ptr = ffi::gst_sample_get_buffer_list(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(BufferListRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_buffer_list_owned")]
    pub fn buffer_list_owned(&self) -> Option<BufferList> {
        unsafe { self.buffer_list().map(|list| from_glib_none(list.as_ptr())) }
    }

    #[doc(alias = "get_caps")]
    #[doc(alias = "gst_sample_get_caps")]
    pub fn caps(&self) -> Option<&CapsRef> {
        unsafe {
            let ptr = ffi::gst_sample_get_caps(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CapsRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_caps_owned")]
    pub fn caps_owned(&self) -> Option<Caps> {
        unsafe { self.caps().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    #[doc(alias = "get_segment")]
    #[doc(alias = "gst_sample_get_segment")]
    pub fn segment(&self) -> Option<Segment> {
        unsafe { from_glib_none(ffi::gst_sample_get_segment(self.as_mut_ptr())) }
    }

    #[doc(alias = "get_info")]
    #[doc(alias = "gst_sample_get_info")]
    pub fn info(&self) -> Option<&StructureRef> {
        unsafe {
            let ptr = ffi::gst_sample_get_info(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(ptr))
            }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_sample_set_buffer")]
    pub fn set_buffer(&mut self, buffer: Option<&Buffer>) {
        unsafe { ffi::gst_sample_set_buffer(self.as_mut_ptr(), buffer.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_sample_set_buffer_list")]
    pub fn set_buffer_list(&mut self, buffer_list: Option<&BufferList>) {
        unsafe { ffi::gst_sample_set_buffer_list(self.as_mut_ptr(), buffer_list.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_sample_set_caps")]
    pub fn set_caps(&mut self, caps: Option<&Caps>) {
        unsafe { ffi::gst_sample_set_caps(self.as_mut_ptr(), caps.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_sample_set_segment")]
    pub fn set_segment(&mut self, segment: Option<&Segment>) {
        unsafe { ffi::gst_sample_set_segment(self.as_mut_ptr(), segment.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_sample_set_info")]
    pub fn set_info(&mut self, info: Option<Structure>) {
        unsafe {
            ffi::gst_sample_set_info(
                self.as_mut_ptr(),
                info.map(|i| i.into_ptr()).unwrap_or(ptr::null_mut()),
            );
        }
    }
}

impl fmt::Debug for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        SampleRef::fmt(self, f)
    }
}

impl fmt::Debug for SampleRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sample")
            .field("buffer", &self.buffer())
            .field("caps", &self.caps())
            .field("segment", &self.segment())
            .field("info", &self.info())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_new_with_info() {
        use crate::Sample;
        use crate::Structure;

        crate::init().unwrap();

        let info = Structure::builder("sample.info")
            .field("f3", &123i32)
            .build();
        let sample = Sample::builder().info(info).build();

        assert!(sample.info().is_some());
    }
}
