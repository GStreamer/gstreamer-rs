// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gstreamer_base_sys as ffi;

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

#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

pub mod functions;
pub use crate::functions::*;

mod adapter;
pub use crate::adapter::*;
mod flow_combiner;
pub use crate::flow_combiner::*;
mod aggregator;
mod aggregator_pad;
mod base_parse;
mod base_sink;
mod base_src;
mod base_transform;

pub mod base_parse_frame;
pub use crate::base_parse_frame::BaseParseFrame;

pub const BASE_TRANSFORM_FLOW_DROPPED: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const BASE_PARSE_FLOW_DROPPED: gst::FlowSuccess = gst::FlowSuccess::CustomSuccess;
pub const AGGREGATOR_FLOW_NEED_DATA: gst::FlowError = gst::FlowError::CustomError;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_base::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::{
        aggregator::AggregatorExtManual, aggregator_pad::AggregatorPadExtManual, auto::traits::*,
        base_parse::BaseParseExtManual, base_sink::BaseSinkExtManual, base_src::BaseSrcExtManual,
        base_transform::BaseTransformExtManual,
    };
}

pub mod subclass;
