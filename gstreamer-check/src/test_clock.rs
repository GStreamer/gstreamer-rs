// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::translate::*;

use crate::{ffi, TestClock};

impl TestClock {
    #[doc(alias = "gst_test_clock_has_id")]
    pub fn has_id(&self, id: &gst::ClockId) -> bool {
        unsafe {
            from_glib(ffi::gst_test_clock_has_id(
                self.to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_test_clock_peek_next_pending_id")]
    pub fn peek_next_pending_id(&self) -> Option<gst::ClockId> {
        unsafe {
            let mut id = ptr::null_mut();
            let ret: bool = from_glib(ffi::gst_test_clock_peek_next_pending_id(
                self.to_glib_none().0,
                &mut id,
            ));
            if ret {
                from_glib_full(id)
            } else {
                None
            }
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_test_clock_process_id")]
    pub fn process_id(&self, pending_id: &gst::ClockId) -> bool {
        unsafe {
            from_glib(ffi::gst_test_clock_process_id(
                self.to_glib_none().0,
                pending_id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_test_clock_process_id_list")]
    pub fn process_id_list(&self, pending_list: &[&gst::ClockId]) -> u32 {
        unsafe {
            ffi::gst_test_clock_process_id_list(
                self.to_glib_none().0,
                pending_list.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gst_test_clock_process_next_clock_id")]
    pub fn process_next_clock_id(&self) -> Option<gst::ClockId> {
        unsafe {
            from_glib_full(ffi::gst_test_clock_process_next_clock_id(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_test_clock_wait_for_multiple_pending_ids")]
    pub fn wait_for_multiple_pending_ids(&self, count: u32) -> Vec<gst::ClockId> {
        unsafe {
            let mut pending_list = ptr::null_mut();
            ffi::gst_test_clock_wait_for_multiple_pending_ids(
                self.to_glib_none().0,
                count,
                &mut pending_list,
            );
            FromGlibPtrContainer::from_glib_full(pending_list)
        }
    }

    #[doc(alias = "gst_test_clock_wait_for_next_pending_id")]
    pub fn wait_for_next_pending_id(&self) -> gst::ClockId {
        unsafe {
            let mut id = ptr::null_mut();
            ffi::gst_test_clock_wait_for_next_pending_id(self.to_glib_none().0, &mut id);
            from_glib_full(id)
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_test_clock_timed_wait_for_multiple_pending_ids")]
    pub fn timed_wait_for_multiple_pending_ids(
        &self,
        count: u32,
        timeout_ms: u32,
    ) -> (bool, Vec<gst::ClockId>) {
        unsafe {
            let mut pending_list = ptr::null_mut();
            let res = ffi::gst_test_clock_timed_wait_for_multiple_pending_ids(
                self.to_glib_none().0,
                count,
                timeout_ms,
                &mut pending_list,
            );
            (
                from_glib(res),
                FromGlibPtrContainer::from_glib_full(pending_list),
            )
        }
    }
}
