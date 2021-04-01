// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod web_rtcdtls_transport;
pub use self::web_rtcdtls_transport::WebRTCDTLSTransport;

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
mod web_rtc_data_channel;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
pub use self::web_rtc_data_channel::WebRTCDataChannel;

mod web_rtcice_transport;
pub use self::web_rtcice_transport::WebRTCICETransport;

mod web_rtcrtp_receiver;
pub use self::web_rtcrtp_receiver::WebRTCRTPReceiver;

mod web_rtcrtp_sender;
pub use self::web_rtcrtp_sender::WebRTCRTPSender;

mod web_rtcrtp_transceiver;
pub use self::web_rtcrtp_transceiver::WebRTCRTPTransceiver;

mod web_rtc_session_description;
pub use self::web_rtc_session_description::WebRTCSessionDescription;

mod enums;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use self::enums::WebRTCBundlePolicy;
pub use self::enums::WebRTCDTLSSetup;
pub use self::enums::WebRTCDTLSTransportState;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use self::enums::WebRTCDataChannelState;
#[cfg(any(feature = "v1_14_1", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14_1")))]
pub use self::enums::WebRTCFECType;
pub use self::enums::WebRTCICEComponent;
pub use self::enums::WebRTCICEConnectionState;
pub use self::enums::WebRTCICEGatheringState;
pub use self::enums::WebRTCICERole;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use self::enums::WebRTCICETransportPolicy;
pub use self::enums::WebRTCPeerConnectionState;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use self::enums::WebRTCPriorityType;
pub use self::enums::WebRTCRTPTransceiverDirection;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use self::enums::WebRTCSCTPTransportState;
pub use self::enums::WebRTCSDPType;
pub use self::enums::WebRTCSignalingState;
pub use self::enums::WebRTCStatsType;

#[doc(hidden)]
pub mod traits {}
