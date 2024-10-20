// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, RTSPMedia};

pub trait RTSPMediaExtManual: IsA<RTSPMedia> + 'static {
    #[doc(alias = "gst_rtsp_media_take_pipeline")]
    fn take_pipeline(&self, pipeline: &impl IsA<gst::Pipeline>) {
        unsafe {
            #[cfg(feature = "v1_18")]
            {
                ffi::gst_rtsp_media_take_pipeline(
                    self.as_ref().to_glib_none().0,
                    pipeline.upcast_ref().to_glib_none().0,
                );
            }
            #[cfg(not(feature = "v1_18"))]
            {
                let pipeline = pipeline.upcast_ref().to_glib_full();
                // See https://gitlab.freedesktop.org/gstreamer/gst-rtsp-server/merge_requests/109
                glib::gobject_ffi::g_object_force_floating(pipeline as *mut _);
                ffi::gst_rtsp_media_take_pipeline(self.as_ref().to_glib_none().0, pipeline);
                if glib::gobject_ffi::g_object_is_floating(pipeline as *mut _) != glib::ffi::GFALSE
                {
                    glib::gobject_ffi::g_object_ref_sink(pipeline as *mut _);
                }
            }
        }
    }
}

impl<O: IsA<RTSPMedia>> RTSPMediaExtManual for O {}
