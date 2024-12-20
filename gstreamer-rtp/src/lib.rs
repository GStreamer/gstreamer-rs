// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use glib;
pub use gst;
pub use gstreamer_rtp_sys as ffi;

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

mod auto;
pub use crate::auto::*;

#[cfg(feature = "serde")]
mod flag_serde;

pub mod subclass;

pub mod rtp_buffer;
pub use crate::rtp_buffer::{
    calc_header_len, calc_packet_len, calc_payload_len, compare_seqnum, ext_timestamp, RTPBuffer,
};
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub mod rtp_header_extension;

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
pub mod rtp_base_payload;

pub mod rtp_base_depayload;

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
mod rtp_meta;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
pub use crate::rtp_meta::*;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_rtp::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub use crate::rtp_base_payload::RTPBasePayloadExtManual;
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub use crate::rtp_header_extension::RTPHeaderExtensionExtManual;
    pub use crate::{
        auto::traits::*, rtp_base_depayload::RTPBaseDepayloadExtManual, rtp_buffer::RTPBufferExt,
    };
}

pub mod functions {
    pub use super::auto::functions::*;
}
