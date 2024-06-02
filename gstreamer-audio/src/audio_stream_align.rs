// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;

use crate::AudioStreamAlign;

impl AudioStreamAlign {
    #[doc(alias = "gst_audio_stream_align_process")]
    pub fn process(
        &mut self,
        discont: bool,
        timestamp: gst::ClockTime,
        n_samples: u32,
    ) -> (bool, gst::ClockTime, gst::ClockTime, u64) {
        unsafe {
            let mut out_timestamp = mem::MaybeUninit::uninit();
            let mut out_duration = mem::MaybeUninit::uninit();
            let mut out_sample_position = mem::MaybeUninit::uninit();
            let ret = from_glib(crate::ffi::gst_audio_stream_align_process(
                self.to_glib_none_mut().0,
                discont.into_glib(),
                timestamp.into_glib(),
                n_samples,
                out_timestamp.as_mut_ptr(),
                out_duration.as_mut_ptr(),
                out_sample_position.as_mut_ptr(),
            ));
            (
                ret,
                try_from_glib(out_timestamp.assume_init()).expect("undefined out_timestamp"),
                try_from_glib(out_duration.assume_init()).expect("undefined out_duration"),
                out_sample_position.assume_init(),
            )
        }
    }
}
