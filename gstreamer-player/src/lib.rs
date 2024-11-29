// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use gst;
pub use gst_video;
pub use gstreamer_player_sys as ffi;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

#[allow(clippy::needless_borrow)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::*;

pub mod subclass;

mod config;
mod player;
pub use crate::config::*;

mod player_video_info;

mod player_g_main_context_signal_dispatcher;
mod player_video_overlay_video_renderer;
mod player_visualization;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_player::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::auto::traits::*;
}
