// Copyright (C) 2019 Philippe Normand <philn@igalia.com>
// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_video_sys;
use std::mem;
use std::ptr;
use utils::HasStreamLock;
use video_codec_state::{InNegotiation, Readable, VideoCodecState, VideoCodecStateContext};
use VideoCodecFrame;
use VideoDecoder;
use VideoFormat;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use VideoInterlaceMode;

extern "C" {
    fn _gst_video_decoder_error(
        dec: *mut gst_video_sys::GstVideoDecoder,
        weight: i32,
        domain: glib_sys::GQuark,
        code: i32,
        txt: *mut libc::c_char,
        debug: *mut libc::c_char,
        file: *const libc::c_char,
        function: *const libc::c_char,
        line: i32,
    ) -> gst_sys::GstFlowReturn;
}

pub trait VideoDecoderExtManual: 'static {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn allocate_output_frame(
        &self,
        frame: &mut VideoCodecFrame,
        params: Option<&gst::BufferPoolAcquireParams>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn get_frame(&self, frame_number: i32) -> Option<VideoCodecFrame>;
    fn get_frames(&self) -> Vec<VideoCodecFrame>;
    fn get_oldest_frame(&self) -> Option<VideoCodecFrame>;

    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn have_frame(&self) -> Result<gst::FlowSuccess, gst::FlowError>;
    fn finish_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;
    fn release_frame(&self, frame: VideoCodecFrame);
    fn drop_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime);
    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime);

    fn get_output_state(&self) -> Option<VideoCodecState<'static, Readable>>;
    fn set_output_state(
        &self,
        fmt: VideoFormat,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError>;
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_interlaced_output_state(
        &self,
        fmt: VideoFormat,
        mode: VideoInterlaceMode,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError>;

    fn negotiate<'a>(
        &'a self,
        output_state: VideoCodecState<'a, InNegotiation<'a>>,
    ) -> Result<(), gst::FlowError>;

    #[allow(clippy::too_many_arguments)]
    fn error<T: gst::MessageErrorDomain>(
        &self,
        weight: i32,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<VideoDecoder>> VideoDecoderExtManual for O {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn allocate_output_frame(
        &self,
        frame: &mut VideoCodecFrame,
        params: Option<&gst::BufferPoolAcquireParams>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            let params_ptr = params.to_glib_none().0 as *mut _;
            from_glib(
                gst_video_sys::gst_video_decoder_allocate_output_frame_with_params(
                    self.as_ref().to_glib_none().0,
                    frame.to_glib_none().0,
                    params_ptr,
                ),
            )
        };
        ret.into_result()
    }

    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            gst_video_sys::gst_video_decoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn have_frame(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_video_sys::gst_video_decoder_have_frame(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn finish_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_video_sys::gst_video_decoder_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.into_ptr(),
            ))
        };
        ret.into_result()
    }

    fn release_frame(&self, frame: VideoCodecFrame) {
        unsafe {
            gst_video_sys::gst_video_decoder_release_frame(
                self.as_ref().to_glib_none().0,
                frame.into_ptr(),
            )
        }
    }

    fn drop_frame(&self, frame: VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_video_sys::gst_video_decoder_drop_frame(
                self.as_ref().to_glib_none().0,
                frame.into_ptr(),
            ))
        };
        ret.into_result()
    }

    fn get_latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        let mut min_latency = gst_sys::GST_CLOCK_TIME_NONE;
        let mut max_latency = gst_sys::GST_CLOCK_TIME_NONE;

        unsafe {
            gst_video_sys::gst_video_decoder_get_latency(
                self.as_ref().to_glib_none().0,
                &mut min_latency,
                &mut max_latency,
            );

            (from_glib(min_latency), from_glib(max_latency))
        }
    }

    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime) {
        unsafe {
            gst_video_sys::gst_video_decoder_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.to_glib(),
                max_latency.to_glib(),
            );
        }
    }

    fn get_frame(&self, frame_number: i32) -> Option<VideoCodecFrame> {
        let frame = unsafe {
            gst_video_sys::gst_video_decoder_get_frame(self.as_ref().to_glib_none().0, frame_number)
        };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    fn get_frames(&self) -> Vec<VideoCodecFrame> {
        unsafe {
            let frames =
                gst_video_sys::gst_video_decoder_get_frames(self.as_ref().to_glib_none().0);
            let mut iter: *const glib_sys::GList = frames;
            let mut vec = Vec::new();

            while !iter.is_null() {
                let frame_ptr = Ptr::from((*iter).data);
                /* transfer ownership of the frame */
                let frame = VideoCodecFrame::new(frame_ptr, self.as_ref());
                vec.push(frame);
                iter = (*iter).next;
            }

            glib_sys::g_list_free(frames);
            vec
        }
    }

    fn get_oldest_frame(&self) -> Option<VideoCodecFrame> {
        let frame = unsafe {
            gst_video_sys::gst_video_decoder_get_oldest_frame(self.as_ref().to_glib_none().0)
        };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    fn get_output_state(&self) -> Option<VideoCodecState<'static, Readable>> {
        let state = unsafe {
            gst_video_sys::gst_video_decoder_get_output_state(self.as_ref().to_glib_none().0)
        };

        if state.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecState::<Readable>::new(state)) }
        }
    }

    fn set_output_state(
        &self,
        fmt: VideoFormat,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError> {
        let state = unsafe {
            let reference = match reference {
                Some(reference) => reference.as_mut_ptr(),
                None => ptr::null_mut(),
            };
            gst_video_sys::gst_video_decoder_set_output_state(
                self.as_ref().to_glib_none().0,
                fmt.to_glib(),
                width,
                height,
                reference,
            )
        };

        if state.is_null() {
            Err(gst::FlowError::NotNegotiated)
        } else {
            unsafe { Ok(VideoCodecState::<InNegotiation>::new(state, self.as_ref())) }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_interlaced_output_state(
        &self,
        fmt: VideoFormat,
        mode: VideoInterlaceMode,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError> {
        let state = unsafe {
            let reference = match reference {
                Some(reference) => reference.as_mut_ptr(),
                None => ptr::null_mut(),
            };
            gst_video_sys::gst_video_decoder_set_interlaced_output_state(
                self.as_ref().to_glib_none().0,
                fmt.to_glib(),
                mode.to_glib(),
                width,
                height,
                reference,
            )
        };

        if state.is_null() {
            Err(gst::FlowError::NotNegotiated)
        } else {
            unsafe { Ok(VideoCodecState::<InNegotiation>::new(state, self.as_ref())) }
        }
    }

    fn negotiate<'a>(
        &'a self,
        output_state: VideoCodecState<'a, InNegotiation<'a>>,
    ) -> Result<(), gst::FlowError> {
        // Consume output_state so user won't be able to modify it anymore
        let self_ptr = self.to_glib_none().0 as *const gst_sys::GstElement;
        assert_eq!(output_state.context.get_element_as_ptr(), self_ptr);

        let ret = unsafe {
            from_glib(gst_video_sys::gst_video_decoder_negotiate(
                self.as_ref().to_glib_none().0,
            ))
        };
        if ret {
            Ok(())
        } else {
            Err(gst::FlowError::NotNegotiated)
        }
    }
    fn error<T: gst::MessageErrorDomain>(
        &self,
        weight: i32,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(_gst_video_decoder_error(
                self.as_ref().to_glib_none().0,
                weight,
                T::domain().to_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            ))
        };
        ret.into_result()
    }
}

impl HasStreamLock for VideoDecoder {
    fn get_stream_lock(&self) -> *mut glib_sys::GRecMutex {
        let decoder_sys: *const gstreamer_video_sys::GstVideoDecoder = self.to_glib_none().0;
        unsafe { &(*decoder_sys).stream_lock as *const _ as usize as *mut _ }
    }

    fn get_element_as_ptr(&self) -> *const gst_sys::GstElement {
        let decoder_sys: *const gstreamer_video_sys::GstVideoDecoder = self.to_glib_none().0;
        decoder_sys as *const gst_sys::GstElement
    }
}

#[macro_export]
macro_rules! gst_video_decoder_error(
    ($obj:expr, $weight:expr, $err:expr, ($msg:expr), [$debug:expr]) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some($msg),
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($msg:expr)) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some($msg),
            None,
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, [$debug:expr]) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            None,
            Some($debug),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            module_path!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            module_path!(),
            line!(),
        )
    }};
);
