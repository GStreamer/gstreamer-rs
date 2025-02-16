// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_base;
pub use gstreamer_audio_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::needless_borrow)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

mod caps;
pub use crate::caps::AudioCapsBuilder;

#[cfg(feature = "serde")]
mod flag_serde;

mod audio_format;
pub use crate::audio_format::*;
mod audio_format_info;
pub use crate::audio_format_info::*;
mod audio_ring_buffer_spec;
pub use crate::audio_ring_buffer_spec::*;
mod audio_info;
pub use crate::audio_info::*;
mod audio_meta;
pub use crate::audio_meta::*;
mod audio_channel_position;
pub use crate::audio_channel_position::*;
mod audio_aggregator;
mod audio_aggregator_convert_pad;
mod audio_aggregator_pad;
mod audio_stream_align;
mod functions;
pub use crate::functions::*;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub mod audio_buffer;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use audio_buffer::{AudioBuffer, AudioBufferRef};

mod audio_decoder;
mod audio_encoder;
mod audio_filter;

mod audio_converter;
pub use crate::audio_converter::AudioConverterConfig;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_audio::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;

    pub use super::{audio_decoder::AudioDecoderExtManual, audio_encoder::AudioEncoderExtManual};
    pub use crate::{
        audio_aggregator::AudioAggregatorExtManual,
        audio_aggregator_convert_pad::AudioAggregatorConvertPadExtManual,
        audio_aggregator_pad::AudioAggregatorPadExtManual, audio_filter::AudioFilterExtManual,
        audio_format::AudioFormatIteratorExt, auto::traits::*,
    };
}

pub mod subclass;
