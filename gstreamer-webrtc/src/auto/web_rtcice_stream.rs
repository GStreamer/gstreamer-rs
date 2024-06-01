// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, WebRTCICEComponent, WebRTCICETransport};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstWebRTCICEStream")]
    pub struct WebRTCICEStream(Object<ffi::GstWebRTCICEStream, ffi::GstWebRTCICEStreamClass>);

    match fn {
        type_ => || ffi::gst_webrtc_ice_stream_get_type(),
    }
}

impl WebRTCICEStream {
    pub const NONE: Option<&'static WebRTCICEStream> = None;
}

unsafe impl Send for WebRTCICEStream {}
unsafe impl Sync for WebRTCICEStream {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebRTCICEStream>> Sealed for T {}
}

pub trait WebRTCICEStreamExt: IsA<WebRTCICEStream> + sealed::Sealed + 'static {
    #[doc(alias = "gst_webrtc_ice_stream_find_transport")]
    fn find_transport(&self, component: WebRTCICEComponent) -> Option<WebRTCICETransport> {
        unsafe {
            from_glib_full(ffi::gst_webrtc_ice_stream_find_transport(
                self.as_ref().to_glib_none().0,
                component.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_webrtc_ice_stream_gather_candidates")]
    fn gather_candidates(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_webrtc_ice_stream_gather_candidates(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "stream-id")]
    fn stream_id(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "stream-id")
    }
}

impl<O: IsA<WebRTCICEStream>> WebRTCICEStreamExt for O {}
