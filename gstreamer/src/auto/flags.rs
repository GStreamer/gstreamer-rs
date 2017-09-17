// This file was generated by gir (6a48033) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct BufferFlags: u32 {
        const BUFFER_FLAG_LIVE = 16;
        const BUFFER_FLAG_DECODE_ONLY = 32;
        const BUFFER_FLAG_DISCONT = 64;
        const BUFFER_FLAG_RESYNC = 128;
        const BUFFER_FLAG_CORRUPTED = 256;
        const BUFFER_FLAG_MARKER = 512;
        const BUFFER_FLAG_HEADER = 1024;
        const BUFFER_FLAG_GAP = 2048;
        const BUFFER_FLAG_DROPPABLE = 4096;
        const BUFFER_FLAG_DELTA_UNIT = 8192;
        const BUFFER_FLAG_TAG_MEMORY = 16384;
        const BUFFER_FLAG_SYNC_AFTER = 32768;
        const BUFFER_FLAG_LAST = 1048576;
    }
}

#[doc(hidden)]
impl ToGlib for BufferFlags {
    type GlibType = ffi::GstBufferFlags;

    fn to_glib(&self) -> ffi::GstBufferFlags {
        ffi::GstBufferFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstBufferFlags> for BufferFlags {
    fn from_glib(value: ffi::GstBufferFlags) -> BufferFlags {
        skip_assert_initialized!();
        BufferFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for BufferFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_buffer_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BufferFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BufferFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstBufferFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for BufferFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct DebugColorFlags: u32 {
        const DEBUG_FG_BLACK = 0;
        const DEBUG_FG_RED = 1;
        const DEBUG_FG_GREEN = 2;
        const DEBUG_FG_YELLOW = 3;
        const DEBUG_FG_BLUE = 4;
        const DEBUG_FG_MAGENTA = 5;
        const DEBUG_FG_CYAN = 6;
        const DEBUG_FG_WHITE = 7;
        const DEBUG_BG_BLACK = 0;
        const DEBUG_BG_RED = 16;
        const DEBUG_BG_GREEN = 32;
        const DEBUG_BG_YELLOW = 48;
        const DEBUG_BG_BLUE = 64;
        const DEBUG_BG_MAGENTA = 80;
        const DEBUG_BG_CYAN = 96;
        const DEBUG_BG_WHITE = 112;
        const DEBUG_BOLD = 256;
        const DEBUG_UNDERLINE = 512;
    }
}

#[doc(hidden)]
impl ToGlib for DebugColorFlags {
    type GlibType = ffi::GstDebugColorFlags;

    fn to_glib(&self) -> ffi::GstDebugColorFlags {
        ffi::GstDebugColorFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDebugColorFlags> for DebugColorFlags {
    fn from_glib(value: ffi::GstDebugColorFlags) -> DebugColorFlags {
        skip_assert_initialized!();
        DebugColorFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for DebugColorFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_debug_color_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DebugColorFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DebugColorFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstDebugColorFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for DebugColorFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct DebugGraphDetails: u32 {
        const DEBUG_GRAPH_SHOW_MEDIA_TYPE = 1;
        const DEBUG_GRAPH_SHOW_CAPS_DETAILS = 2;
        const DEBUG_GRAPH_SHOW_NON_DEFAULT_PARAMS = 4;
        const DEBUG_GRAPH_SHOW_STATES = 8;
        const DEBUG_GRAPH_SHOW_FULL_PARAMS = 16;
        const DEBUG_GRAPH_SHOW_ALL = 15;
        const DEBUG_GRAPH_SHOW_VERBOSE = 4294967295;
    }
}

#[doc(hidden)]
impl ToGlib for DebugGraphDetails {
    type GlibType = ffi::GstDebugGraphDetails;

    fn to_glib(&self) -> ffi::GstDebugGraphDetails {
        ffi::GstDebugGraphDetails::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstDebugGraphDetails> for DebugGraphDetails {
    fn from_glib(value: ffi::GstDebugGraphDetails) -> DebugGraphDetails {
        skip_assert_initialized!();
        DebugGraphDetails::from_bits_truncate(value.bits())
    }
}

impl StaticType for DebugGraphDetails {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_debug_graph_details_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DebugGraphDetails {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DebugGraphDetails {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstDebugGraphDetails::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for DebugGraphDetails {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct ElementFlags: u32 {
        const ELEMENT_FLAG_LOCKED_STATE = 16;
        const ELEMENT_FLAG_SINK = 32;
        const ELEMENT_FLAG_SOURCE = 64;
        const ELEMENT_FLAG_PROVIDE_CLOCK = 128;
        const ELEMENT_FLAG_REQUIRE_CLOCK = 256;
        const ELEMENT_FLAG_INDEXABLE = 512;
        const ELEMENT_FLAG_LAST = 16384;
    }
}

#[doc(hidden)]
impl ToGlib for ElementFlags {
    type GlibType = ffi::GstElementFlags;

    fn to_glib(&self) -> ffi::GstElementFlags {
        ffi::GstElementFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstElementFlags> for ElementFlags {
    fn from_glib(value: ffi::GstElementFlags) -> ElementFlags {
        skip_assert_initialized!();
        ElementFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for ElementFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_element_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ElementFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ElementFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstElementFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for ElementFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct PadLinkCheck: u32 {
        const PAD_LINK_CHECK_NOTHING = 0;
        const PAD_LINK_CHECK_HIERARCHY = 1;
        const PAD_LINK_CHECK_TEMPLATE_CAPS = 2;
        const PAD_LINK_CHECK_CAPS = 4;
        const PAD_LINK_CHECK_NO_RECONFIGURE = 8;
        const PAD_LINK_CHECK_DEFAULT = 5;
    }
}

#[doc(hidden)]
impl ToGlib for PadLinkCheck {
    type GlibType = ffi::GstPadLinkCheck;

    fn to_glib(&self) -> ffi::GstPadLinkCheck {
        ffi::GstPadLinkCheck::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadLinkCheck> for PadLinkCheck {
    fn from_glib(value: ffi::GstPadLinkCheck) -> PadLinkCheck {
        skip_assert_initialized!();
        PadLinkCheck::from_bits_truncate(value.bits())
    }
}

impl StaticType for PadLinkCheck {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_pad_link_check_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadLinkCheck {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadLinkCheck {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstPadLinkCheck::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for PadLinkCheck {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct PadProbeType: u32 {
        const PAD_PROBE_TYPE_INVALID = 0;
        const PAD_PROBE_TYPE_IDLE = 1;
        const PAD_PROBE_TYPE_BLOCK = 2;
        const PAD_PROBE_TYPE_BUFFER = 16;
        const PAD_PROBE_TYPE_BUFFER_LIST = 32;
        const PAD_PROBE_TYPE_EVENT_DOWNSTREAM = 64;
        const PAD_PROBE_TYPE_EVENT_UPSTREAM = 128;
        const PAD_PROBE_TYPE_EVENT_FLUSH = 256;
        const PAD_PROBE_TYPE_QUERY_DOWNSTREAM = 512;
        const PAD_PROBE_TYPE_QUERY_UPSTREAM = 1024;
        const PAD_PROBE_TYPE_PUSH = 4096;
        const PAD_PROBE_TYPE_PULL = 8192;
        const PAD_PROBE_TYPE_BLOCKING = 3;
        const PAD_PROBE_TYPE_DATA_DOWNSTREAM = 112;
        const PAD_PROBE_TYPE_DATA_UPSTREAM = 128;
        const PAD_PROBE_TYPE_DATA_BOTH = 240;
        const PAD_PROBE_TYPE_BLOCK_DOWNSTREAM = 114;
        const PAD_PROBE_TYPE_BLOCK_UPSTREAM = 130;
        const PAD_PROBE_TYPE_EVENT_BOTH = 192;
        const PAD_PROBE_TYPE_QUERY_BOTH = 1536;
        const PAD_PROBE_TYPE_ALL_BOTH = 1776;
        const PAD_PROBE_TYPE_SCHEDULING = 12288;
    }
}

#[doc(hidden)]
impl ToGlib for PadProbeType {
    type GlibType = ffi::GstPadProbeType;

    fn to_glib(&self) -> ffi::GstPadProbeType {
        ffi::GstPadProbeType::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstPadProbeType> for PadProbeType {
    fn from_glib(value: ffi::GstPadProbeType) -> PadProbeType {
        skip_assert_initialized!();
        PadProbeType::from_bits_truncate(value.bits())
    }
}

impl StaticType for PadProbeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_pad_probe_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PadProbeType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PadProbeType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstPadProbeType::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for PadProbeType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct ParseFlags: u32 {
        const PARSE_FLAG_NONE = 0;
        const PARSE_FLAG_FATAL_ERRORS = 1;
        const PARSE_FLAG_NO_SINGLE_ELEMENT_BINS = 2;
        const PARSE_FLAG_PLACE_IN_BIN = 4;
    }
}

#[doc(hidden)]
impl ToGlib for ParseFlags {
    type GlibType = ffi::GstParseFlags;

    fn to_glib(&self) -> ffi::GstParseFlags {
        ffi::GstParseFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstParseFlags> for ParseFlags {
    fn from_glib(value: ffi::GstParseFlags) -> ParseFlags {
        skip_assert_initialized!();
        ParseFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for ParseFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_parse_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ParseFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ParseFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstParseFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for ParseFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SchedulingFlags: u32 {
        const SCHEDULING_FLAG_SEEKABLE = 1;
        const SCHEDULING_FLAG_SEQUENTIAL = 2;
        const SCHEDULING_FLAG_BANDWIDTH_LIMITED = 4;
    }
}

#[doc(hidden)]
impl ToGlib for SchedulingFlags {
    type GlibType = ffi::GstSchedulingFlags;

    fn to_glib(&self) -> ffi::GstSchedulingFlags {
        ffi::GstSchedulingFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSchedulingFlags> for SchedulingFlags {
    fn from_glib(value: ffi::GstSchedulingFlags) -> SchedulingFlags {
        skip_assert_initialized!();
        SchedulingFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for SchedulingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_scheduling_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SchedulingFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SchedulingFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstSchedulingFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SchedulingFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SeekFlags: u32 {
        const SEEK_FLAG_NONE = 0;
        const SEEK_FLAG_FLUSH = 1;
        const SEEK_FLAG_ACCURATE = 2;
        const SEEK_FLAG_KEY_UNIT = 4;
        const SEEK_FLAG_SEGMENT = 8;
        const SEEK_FLAG_TRICKMODE = 16;
        const SEEK_FLAG_SKIP = 16;
        const SEEK_FLAG_SNAP_BEFORE = 32;
        const SEEK_FLAG_SNAP_AFTER = 64;
        const SEEK_FLAG_SNAP_NEAREST = 96;
        const SEEK_FLAG_TRICKMODE_KEY_UNITS = 128;
        const SEEK_FLAG_TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SeekFlags {
    type GlibType = ffi::GstSeekFlags;

    fn to_glib(&self) -> ffi::GstSeekFlags {
        ffi::GstSeekFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSeekFlags> for SeekFlags {
    fn from_glib(value: ffi::GstSeekFlags) -> SeekFlags {
        skip_assert_initialized!();
        SeekFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for SeekFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_seek_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SeekFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SeekFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstSeekFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SeekFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SegmentFlags: u32 {
        const SEGMENT_FLAG_NONE = 0;
        const SEGMENT_FLAG_RESET = 1;
        const SEGMENT_FLAG_TRICKMODE = 16;
        const SEGMENT_FLAG_SKIP = 16;
        const SEGMENT_FLAG_SEGMENT = 8;
        const SEGMENT_FLAG_TRICKMODE_KEY_UNITS = 128;
        const SEGMENT_FLAG_TRICKMODE_NO_AUDIO = 256;
    }
}

#[doc(hidden)]
impl ToGlib for SegmentFlags {
    type GlibType = ffi::GstSegmentFlags;

    fn to_glib(&self) -> ffi::GstSegmentFlags {
        ffi::GstSegmentFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstSegmentFlags> for SegmentFlags {
    fn from_glib(value: ffi::GstSegmentFlags) -> SegmentFlags {
        skip_assert_initialized!();
        SegmentFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for SegmentFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_segment_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SegmentFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SegmentFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstSegmentFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SegmentFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

#[cfg(feature = "v1_12")]
bitflags! {
    pub struct StackTraceFlags: u32 {
        const STACK_TRACE_SHOW_FULL = 1;
    }
}

#[cfg(feature = "v1_12")]
#[doc(hidden)]
impl ToGlib for StackTraceFlags {
    type GlibType = ffi::GstStackTraceFlags;

    fn to_glib(&self) -> ffi::GstStackTraceFlags {
        ffi::GstStackTraceFlags::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v1_12")]
#[doc(hidden)]
impl FromGlib<ffi::GstStackTraceFlags> for StackTraceFlags {
    fn from_glib(value: ffi::GstStackTraceFlags) -> StackTraceFlags {
        skip_assert_initialized!();
        StackTraceFlags::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v1_12")]
impl StaticType for StackTraceFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_stack_trace_flags_get_type()) }
    }
}

#[cfg(feature = "v1_12")]
impl<'a> FromValueOptional<'a> for StackTraceFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(feature = "v1_12")]
impl<'a> FromValue<'a> for StackTraceFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstStackTraceFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

#[cfg(feature = "v1_12")]
impl SetValue for StackTraceFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct StreamFlags: u32 {
        const STREAM_FLAG_NONE = 0;
        const STREAM_FLAG_SPARSE = 1;
        const STREAM_FLAG_SELECT = 2;
        const STREAM_FLAG_UNSELECT = 4;
    }
}

#[doc(hidden)]
impl ToGlib for StreamFlags {
    type GlibType = ffi::GstStreamFlags;

    fn to_glib(&self) -> ffi::GstStreamFlags {
        ffi::GstStreamFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamFlags> for StreamFlags {
    fn from_glib(value: ffi::GstStreamFlags) -> StreamFlags {
        skip_assert_initialized!();
        StreamFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for StreamFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_stream_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StreamFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StreamFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstStreamFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for StreamFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct StreamType: u32 {
        const STREAM_TYPE_UNKNOWN = 1;
        const STREAM_TYPE_AUDIO = 2;
        const STREAM_TYPE_VIDEO = 4;
        const STREAM_TYPE_CONTAINER = 8;
        const STREAM_TYPE_TEXT = 16;
    }
}

#[doc(hidden)]
impl ToGlib for StreamType {
    type GlibType = ffi::GstStreamType;

    fn to_glib(&self) -> ffi::GstStreamType {
        ffi::GstStreamType::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstStreamType> for StreamType {
    fn from_glib(value: ffi::GstStreamType) -> StreamType {
        skip_assert_initialized!();
        StreamType::from_bits_truncate(value.bits())
    }
}

impl StaticType for StreamType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_stream_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StreamType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StreamType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GstStreamType::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for StreamType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

