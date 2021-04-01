// Generated by gir (https://github.com/gtk-rs/gir @ 28780fd)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6088bb6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 208138a)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_WEBRTC_BUNDLE_POLICY_BALANCED);
    PRINT_CONSTANT((gint) GST_WEBRTC_BUNDLE_POLICY_MAX_BUNDLE);
    PRINT_CONSTANT((gint) GST_WEBRTC_BUNDLE_POLICY_MAX_COMPAT);
    PRINT_CONSTANT((gint) GST_WEBRTC_BUNDLE_POLICY_NONE);
    PRINT_CONSTANT((gint) GST_WEBRTC_DATA_CHANNEL_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_DATA_CHANNEL_STATE_CLOSING);
    PRINT_CONSTANT((gint) GST_WEBRTC_DATA_CHANNEL_STATE_CONNECTING);
    PRINT_CONSTANT((gint) GST_WEBRTC_DATA_CHANNEL_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_DATA_CHANNEL_STATE_OPEN);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_SETUP_ACTIVE);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_SETUP_ACTPASS);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_SETUP_NONE);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_SETUP_PASSIVE);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_CONNECTING);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_FAILED);
    PRINT_CONSTANT((gint) GST_WEBRTC_DTLS_TRANSPORT_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_FEC_TYPE_NONE);
    PRINT_CONSTANT((gint) GST_WEBRTC_FEC_TYPE_ULP_RED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_COMPONENT_RTCP);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_COMPONENT_RTP);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_CHECKING);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_COMPLETED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_CONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_DISCONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_FAILED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_CONNECTION_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_GATHERING_STATE_COMPLETE);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_GATHERING_STATE_GATHERING);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_GATHERING_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_ROLE_CONTROLLED);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_ROLE_CONTROLLING);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_TRANSPORT_POLICY_ALL);
    PRINT_CONSTANT((gint) GST_WEBRTC_ICE_TRANSPORT_POLICY_RELAY);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_CONNECTING);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_DISCONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_FAILED);
    PRINT_CONSTANT((gint) GST_WEBRTC_PEER_CONNECTION_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_PRIORITY_TYPE_HIGH);
    PRINT_CONSTANT((gint) GST_WEBRTC_PRIORITY_TYPE_LOW);
    PRINT_CONSTANT((gint) GST_WEBRTC_PRIORITY_TYPE_MEDIUM);
    PRINT_CONSTANT((gint) GST_WEBRTC_PRIORITY_TYPE_VERY_LOW);
    PRINT_CONSTANT((gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_INACTIVE);
    PRINT_CONSTANT((gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_NONE);
    PRINT_CONSTANT((gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_RECVONLY);
    PRINT_CONSTANT((gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDONLY);
    PRINT_CONSTANT((gint) GST_WEBRTC_RTP_TRANSCEIVER_DIRECTION_SENDRECV);
    PRINT_CONSTANT((gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTED);
    PRINT_CONSTANT((gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_CONNECTING);
    PRINT_CONSTANT((gint) GST_WEBRTC_SCTP_TRANSPORT_STATE_NEW);
    PRINT_CONSTANT((gint) GST_WEBRTC_SDP_TYPE_ANSWER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SDP_TYPE_OFFER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SDP_TYPE_PRANSWER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SDP_TYPE_ROLLBACK);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_CLOSED);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_OFFER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_HAVE_LOCAL_PRANSWER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_OFFER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_HAVE_REMOTE_PRANSWER);
    PRINT_CONSTANT((gint) GST_WEBRTC_SIGNALING_STATE_STABLE);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_CANDIDATE_PAIR);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_CERTIFICATE);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_CODEC);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_CSRC);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_DATA_CHANNEL);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_INBOUND_RTP);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_LOCAL_CANDIDATE);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_OUTBOUND_RTP);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_PEER_CONNECTION);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_REMOTE_CANDIDATE);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_REMOTE_INBOUND_RTP);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_REMOTE_OUTBOUND_RTP);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_STREAM);
    PRINT_CONSTANT((gint) GST_WEBRTC_STATS_TRANSPORT);
    return 0;
}
