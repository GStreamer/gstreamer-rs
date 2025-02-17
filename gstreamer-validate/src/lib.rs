// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use gstreamer_validate_sys as ffi;

#[doc(hidden)]
pub static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cold]
#[inline(never)]
#[track_caller]
pub fn assert_initialized() {
    if unsafe { ffi::gst_validate_is_initialized() } != glib::ffi::GTRUE {
        panic!("GStreamer Validate has not been initialized. Call `gst_validate::init` first.");
    } else {
        crate::INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }

        if !crate::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            $crate::assert_initialized();
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

mod functions;
pub use functions::*;

mod action_type;
pub use action_type::*;

mod enums;
pub use enums::*;

mod action;
pub use action::{Action, ActionRef};

mod reporter;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_validate::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst::prelude::*;

    pub use crate::auto::traits::*;
}
