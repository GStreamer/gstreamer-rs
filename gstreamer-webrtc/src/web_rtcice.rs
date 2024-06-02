// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, WebRTCICE, WebRTCICEStream};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::WebRTCICE>> Sealed for T {}
}

pub trait WebRTCICEExtManual: sealed::Sealed + IsA<WebRTCICE> + 'static {
    #[doc(alias = "gst_webrtc_ice_add_candidate")]
    fn add_candidate(&self, stream: &impl IsA<WebRTCICEStream>, candidate: &str) {
        #[cfg(not(feature = "v1_24"))]
        unsafe {
            use std::{mem, ptr};

            if gst::version() < (1, 23, 0, 0) {
                let func = mem::transmute::<
                    unsafe extern "C" fn(
                        *mut ffi::GstWebRTCICE,
                        *mut ffi::GstWebRTCICEStream,
                        *const std::os::raw::c_char,
                        *mut gst::ffi::GstPromise,
                    ),
                    unsafe extern "C" fn(
                        *mut ffi::GstWebRTCICE,
                        *mut ffi::GstWebRTCICEStream,
                        *const std::os::raw::c_char,
                    ),
                >(ffi::gst_webrtc_ice_add_candidate);
                func(
                    self.as_ref().to_glib_none().0,
                    stream.as_ref().to_glib_none().0,
                    candidate.to_glib_none().0,
                );
            } else {
                ffi::gst_webrtc_ice_add_candidate(
                    self.as_ref().to_glib_none().0,
                    stream.as_ref().to_glib_none().0,
                    candidate.to_glib_none().0,
                    ptr::null_mut(),
                );
            }
        }
        #[cfg(feature = "v1_24")]
        {
            self.add_candidate_full(stream, candidate, None);
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "gst_webrtc_ice_add_candidate")]
    fn add_candidate_full(
        &self,
        stream: &impl IsA<WebRTCICEStream>,
        candidate: &str,
        promise: Option<&gst::Promise>,
    ) {
        unsafe {
            ffi::gst_webrtc_ice_add_candidate(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                candidate.to_glib_none().0,
                promise.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<WebRTCICE>> WebRTCICEExtManual for O {}
