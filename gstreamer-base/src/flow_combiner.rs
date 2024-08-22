// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstFlowCombiner")]
    pub struct FlowCombiner(Shared<ffi::GstFlowCombiner>);

    match fn {
        ref => |ptr| ffi::gst_flow_combiner_ref(ptr),
        unref => |ptr| ffi::gst_flow_combiner_unref(ptr),
        type_ => || ffi::gst_flow_combiner_get_type(),
    }
}

impl FlowCombiner {
    #[doc(alias = "gst_flow_combiner_new")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_flow_combiner_new()) }
    }

    #[doc(alias = "gst_flow_combiner_add_pad")]
    pub fn add_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_add_pad(self.to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_flow_combiner_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gst_flow_combiner_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_flow_combiner_remove_pad")]
    pub fn remove_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_remove_pad(self.to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_flow_combiner_reset")]
    pub fn reset(&self) {
        unsafe {
            ffi::gst_flow_combiner_reset(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_flow_combiner_update_flow")]
    pub fn update_flow<FRet: Into<gst::FlowReturn>>(
        &self,
        fret: FRet,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let fret: gst::FlowReturn = fret.into();
        unsafe {
            try_from_glib(ffi::gst_flow_combiner_update_flow(
                self.to_glib_none().0,
                fret.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_flow_combiner_update_pad_flow")]
    pub fn update_pad_flow<P: IsA<gst::Pad>, FRet: Into<gst::FlowReturn>>(
        &self,
        pad: &P,
        fret: FRet,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let fret: gst::FlowReturn = fret.into();
        unsafe {
            try_from_glib(ffi::gst_flow_combiner_update_pad_flow(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                fret.into_glib(),
            ))
        }
    }
}

impl Default for FlowCombiner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct UniqueFlowCombiner(FlowCombiner);

unsafe impl Sync for UniqueFlowCombiner {}
unsafe impl Send for UniqueFlowCombiner {}

impl UniqueFlowCombiner {
    pub fn new() -> Self {
        Self(FlowCombiner::new())
    }

    pub fn add_pad<P: IsA<gst::Pad>>(&mut self, pad: &P) {
        self.0.add_pad(pad);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn remove_pad<P: IsA<gst::Pad>>(&mut self, pad: &P) {
        self.0.remove_pad(pad);
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn update_flow(
        &mut self,
        fret: Result<gst::FlowSuccess, gst::FlowError>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.0.update_flow(fret)
    }

    pub fn update_pad_flow<P: IsA<gst::Pad>>(
        &mut self,
        pad: &P,
        fret: Result<gst::FlowSuccess, gst::FlowError>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.0.update_pad_flow(pad, fret)
    }
}

impl Default for UniqueFlowCombiner {
    fn default() -> Self {
        Self::new()
    }
}
