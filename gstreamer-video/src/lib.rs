// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gst_base;
pub use gstreamer_video_sys as ffi;

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

mod enums;

#[cfg(feature = "serde")]
mod flag_serde;

mod caps;
pub use crate::caps::VideoCapsBuilder;

mod caps_features;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use crate::caps_features::{CAPS_FEATURES_FORMAT_INTERLACED, CAPS_FEATURE_FORMAT_INTERLACED};
pub use crate::caps_features::{
    CAPS_FEATURES_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURES_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURES_META_GST_VIDEO_META,
    CAPS_FEATURES_META_GST_VIDEO_OVERLAY_COMPOSITION,
    CAPS_FEATURE_META_GST_VIDEO_AFFINE_TRANSFORMATION_META,
    CAPS_FEATURE_META_GST_VIDEO_GL_TEXTURE_UPLOAD_META, CAPS_FEATURE_META_GST_VIDEO_META,
    CAPS_FEATURE_META_GST_VIDEO_OVERLAY_COMPOSITION,
};
mod video_color_matrix;
mod video_format;
pub use crate::video_format::*;
mod video_format_info;
pub use crate::video_format_info::*;
mod video_info;
pub use crate::video_info::*;
#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
mod video_info_dma_drm;
#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
pub use crate::video_info_dma_drm::*;
pub mod video_frame;
pub use crate::video_frame::{VideoFrame, VideoFrameExt, VideoFrameRef};
mod video_overlay;
pub use crate::video_overlay::is_video_overlay_prepare_window_handle_message;

pub mod video_event;
pub use crate::video_event::{
    DownstreamForceKeyUnitEvent, ForceKeyUnitEvent, NavigationEvent, StillFrameEvent,
    UpstreamForceKeyUnitEvent,
};

pub mod video_message;
pub use crate::video_message::{NavigationEventMessage, NavigationMessage};

mod functions;
pub use crate::functions::*;
mod video_rectangle;
pub use crate::video_rectangle::*;
pub mod video_overlay_composition;
pub use crate::video_overlay_composition::{
    VideoOverlayComposition, VideoOverlayCompositionRef, VideoOverlayRectangle,
    VideoOverlayRectangleRef,
};
pub mod video_meta;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use crate::video_meta::VideoCaptionMeta;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
pub use crate::video_meta::{VideoAFDMeta, VideoBarMeta};
pub use crate::video_meta::{
    VideoAffineTransformationMeta, VideoCropMeta, VideoMeta, VideoOverlayCompositionMeta,
    VideoRegionOfInterestMeta,
};
mod video_time_code;
pub use crate::video_time_code::{ValidVideoTimeCode, VideoTimeCode, VideoTimeCodeMeta};
mod video_time_code_interval;
pub use crate::video_time_code_interval::VideoTimeCodeInterval;
mod video_buffer_pool;
pub use crate::video_buffer_pool::{
    VideoAlignment, VideoBufferPoolConfig, BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META,
    BUFFER_POOL_OPTION_VIDEO_ALIGNMENT, BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META,
    BUFFER_POOL_OPTION_VIDEO_META,
};
pub mod video_converter;
pub use crate::video_converter::{VideoConverter, VideoConverterConfig};

mod video_codec_frame;
mod video_decoder;
mod video_encoder;
mod video_filter;
pub use crate::video_codec_frame::VideoCodecFrame;
pub mod video_codec_state;
pub use crate::video_codec_state::{VideoCodecState, VideoCodecStateContext};
mod utils;

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
mod video_hdr;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
pub use crate::video_hdr::*;

mod color_balance_channel;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator_convert_pad;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_aggregator_pad;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_vbi;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use video_vbi::*;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_vbi_encoder;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use video_vbi_encoder::*;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod video_vbi_parser;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use video_vbi_parser::*;

pub const VIDEO_ENCODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const VIDEO_DECODER_FLOW_NEED_DATA: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_video::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::prelude::*;

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use crate::video_aggregator::VideoAggregatorExtManual;
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use crate::video_aggregator_convert_pad::VideoAggregatorConvertPadExtManual;
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    pub use crate::video_aggregator_pad::VideoAggregatorPadExtManual;
    pub use crate::VideoFrameExt;
    pub use crate::{
        auto::traits::*, video_buffer_pool::VideoBufferPoolConfig,
        video_decoder::VideoDecoderExtManual, video_encoder::VideoEncoderExtManual,
        video_filter::VideoFilterExtManual, video_format::VideoFormatIteratorExt,
        video_frame::VideoBufferExt, video_overlay::VideoOverlayExtManual,
    };
}

pub mod subclass;
