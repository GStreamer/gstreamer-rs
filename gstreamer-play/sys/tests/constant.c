// Generated by gir (https://github.com/gtk-rs/gir @ b3147f2b6043)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7fa401e3ee5d)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 2860909848fa)
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
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_PLAY_COLOR_BALANCE_BRIGHTNESS);
    PRINT_CONSTANT((gint) GST_PLAY_COLOR_BALANCE_CONTRAST);
    PRINT_CONSTANT((gint) GST_PLAY_COLOR_BALANCE_HUE);
    PRINT_CONSTANT((gint) GST_PLAY_COLOR_BALANCE_SATURATION);
    PRINT_CONSTANT((gint) GST_PLAY_ERROR_FAILED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_BUFFERING);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_DURATION_CHANGED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_END_OF_STREAM);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_ERROR);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_MEDIA_INFO_UPDATED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_MUTE_CHANGED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_POSITION_UPDATED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_SEEK_DONE);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_STATE_CHANGED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_URI_LOADED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_VIDEO_DIMENSIONS_CHANGED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_VOLUME_CHANGED);
    PRINT_CONSTANT((gint) GST_PLAY_MESSAGE_WARNING);
    PRINT_CONSTANT((gint) GST_PLAY_STATE_BUFFERING);
    PRINT_CONSTANT((gint) GST_PLAY_STATE_PAUSED);
    PRINT_CONSTANT((gint) GST_PLAY_STATE_PLAYING);
    PRINT_CONSTANT((gint) GST_PLAY_STATE_STOPPED);
    PRINT_CONSTANT((gint) GST_PLAY_THUMBNAIL_JPG);
    PRINT_CONSTANT((gint) GST_PLAY_THUMBNAIL_PNG);
    PRINT_CONSTANT((gint) GST_PLAY_THUMBNAIL_RAW_BGRx);
    PRINT_CONSTANT((gint) GST_PLAY_THUMBNAIL_RAW_NATIVE);
    PRINT_CONSTANT((gint) GST_PLAY_THUMBNAIL_RAW_xRGB);
    return 0;
}
