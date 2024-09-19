// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::Borrow, cmp, ffi::CStr, fmt, mem, num::NonZeroU32, ops::Deref, ops::DerefMut, ptr,
};

use glib::{
    translate::{FromGlibPtrContainer, *},
    value::ToSendValue,
};

use crate::{
    ffi,
    format::{
        CompatibleFormattedValue, FormattedValue, FormattedValueIntrinsic, GenericFormattedValue,
    },
    structure::*,
    ClockTime, EventType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seqnum(pub(crate) NonZeroU32);

impl Seqnum {
    #[doc(alias = "gst_util_seqnum_next")]
    #[inline]
    pub fn next() -> Self {
        unsafe {
            let v = ffi::gst_util_seqnum_next();
            if v == 0 {
                Seqnum::next()
            } else {
                Seqnum(NonZeroU32::new_unchecked(v))
            }
        }
    }
}

impl IntoGlib for Seqnum {
    type GlibType = u32;

    #[inline]
    fn into_glib(self) -> u32 {
        self.0.get()
    }
}

impl cmp::PartialOrd for Seqnum {
    #[inline]
    fn partial_cmp(&self, other: &Seqnum) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Seqnum {
    #[inline]
    fn cmp(&self, other: &Seqnum) -> cmp::Ordering {
        unsafe {
            let ret = ffi::gst_util_seqnum_compare(self.0.get(), other.0.get());
            ret.cmp(&0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupId(pub(crate) NonZeroU32);

impl GroupId {
    #[doc(alias = "gst_util_group_id_next")]
    #[inline]
    pub fn next() -> Self {
        unsafe {
            let v = ffi::gst_util_group_id_next();
            if v == 0 {
                GroupId::next()
            } else {
                GroupId(NonZeroU32::new_unchecked(v))
            }
        }
    }
}

impl EventType {
    #[doc(alias = "GST_EVENT_IS_UPSTREAM")]
    #[inline]
    pub fn is_upstream(self) -> bool {
        (self.into_glib() as u32) & ffi::GST_EVENT_TYPE_UPSTREAM != 0
    }

    #[doc(alias = "GST_EVENT_IS_DOWNSTREAM")]
    #[inline]
    pub fn is_downstream(self) -> bool {
        (self.into_glib() as u32) & ffi::GST_EVENT_TYPE_DOWNSTREAM != 0
    }

    #[doc(alias = "GST_EVENT_IS_SERIALIZED")]
    #[inline]
    pub fn is_serialized(self) -> bool {
        (self.into_glib() as u32) & ffi::GST_EVENT_TYPE_SERIALIZED != 0
    }

    #[doc(alias = "GST_EVENT_IS_STICKY")]
    #[inline]
    pub fn is_sticky(self) -> bool {
        (self.into_glib() as u32) & ffi::GST_EVENT_TYPE_STICKY != 0
    }

    #[doc(alias = "GST_EVENT_IS_STICKY_MULTI")]
    #[inline]
    pub fn is_sticky_multi(self) -> bool {
        (self.into_glib() as u32) & ffi::GST_EVENT_TYPE_STICKY_MULTI != 0
    }
}

impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if !self.is_serialized() || !other.is_serialized() {
            return None;
        }

        // See gst_event_type_to_sticky_ordering() from 1.22
        let fixup_event_ordering = |v| match v {
            ffi::GST_EVENT_INSTANT_RATE_CHANGE => ffi::GST_EVENT_SEGMENT as u32 + 1,
            _ => v as u32,
        };

        let v1 = fixup_event_ordering(self.into_glib());
        let v2 = fixup_event_ordering(other.into_glib());

        let stream_start = ffi::GST_EVENT_STREAM_START as u32;
        let segment = ffi::GST_EVENT_SEGMENT as u32;
        let eos = ffi::GST_EVENT_EOS as u32;

        // Strictly ordered range between stream_start and segment,
        // and EOS is bigger than everything else
        if v1 >= stream_start && v1 <= segment || v2 >= stream_start && v2 <= segment {
            Some(v1.cmp(&v2))
        // If one is EOS, the other is definitely less or equal
        } else if v1 == eos || v2 == eos {
            if v1 == v2 {
                Some(cmp::Ordering::Equal)
            } else if v1 == eos {
                Some(cmp::Ordering::Greater)
            } else {
                Some(cmp::Ordering::Less)
            }
        } else {
            None
        }
    }
}

mini_object_wrapper!(Event, EventRef, ffi::GstEvent, || {
    ffi::gst_event_get_type()
});

impl EventRef {
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_event_get_seqnum")]
    pub fn seqnum(&self) -> Seqnum {
        unsafe {
            let seqnum = ffi::gst_event_get_seqnum(self.as_mut_ptr());
            debug_assert_ne!(seqnum, 0);
            Seqnum(NonZeroU32::new_unchecked(seqnum))
        }
    }

    #[doc(alias = "gst_event_set_seqnum")]
    pub fn set_seqnum(&mut self, seqnum: Seqnum) {
        unsafe {
            ffi::gst_event_set_seqnum(self.as_mut_ptr(), seqnum.0.get());
        }
    }

    #[doc(alias = "get_running_time_offset")]
    #[doc(alias = "gst_event_get_running_time_offset")]
    pub fn running_time_offset(&self) -> i64 {
        unsafe { ffi::gst_event_get_running_time_offset(self.as_mut_ptr()) }
    }

    #[doc(alias = "gst_event_set_running_time_offset")]
    pub fn set_running_time_offset(&mut self, offset: i64) {
        unsafe { ffi::gst_event_set_running_time_offset(self.as_mut_ptr(), offset) }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_event_get_structure")]
    #[inline]
    pub fn structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_event_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    #[doc(alias = "gst_event_writable_structure")]
    #[inline]
    pub fn structure_mut(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(ffi::gst_event_writable_structure(self.as_mut_ptr()))
        }
    }

    #[doc(alias = "GST_EVENT_IS_UPSTREAM")]
    #[inline]
    pub fn is_upstream(&self) -> bool {
        self.type_().is_upstream()
    }

    #[doc(alias = "GST_EVENT_IS_DOWNSTREAM")]
    #[inline]
    pub fn is_downstream(&self) -> bool {
        self.type_().is_downstream()
    }

    #[doc(alias = "GST_EVENT_IS_SERIALIZED")]
    #[inline]
    pub fn is_serialized(&self) -> bool {
        self.type_().is_serialized()
    }

    #[doc(alias = "GST_EVENT_IS_STICKY")]
    #[inline]
    pub fn is_sticky(&self) -> bool {
        self.type_().is_sticky()
    }

    #[doc(alias = "GST_EVENT_IS_STICKY_MULTI")]
    #[inline]
    pub fn is_sticky_multi(&self) -> bool {
        self.type_().is_sticky_multi()
    }

    #[doc(alias = "get_type")]
    #[doc(alias = "GST_EVENT_TYPE")]
    #[inline]
    pub fn type_(&self) -> EventType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    #[doc(alias = "gst_event_has_name")]
    #[inline]
    pub fn has_name(&self, name: &str) -> bool {
        self.structure().is_some_and(|s| s.has_name(name))
    }

    pub fn view(&self) -> EventView {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_EVENT_FLUSH_START => FlushStart::view(self),
                ffi::GST_EVENT_FLUSH_STOP => FlushStop::view(self),
                ffi::GST_EVENT_STREAM_START => StreamStart::view(self),
                ffi::GST_EVENT_CAPS => Caps::view(self),
                ffi::GST_EVENT_SEGMENT => Segment::view(self),
                ffi::GST_EVENT_STREAM_COLLECTION => StreamCollection::view(self),
                ffi::GST_EVENT_TAG => Tag::view(self),
                ffi::GST_EVENT_BUFFERSIZE => Buffersize::view(self),
                ffi::GST_EVENT_SINK_MESSAGE => SinkMessage::view(self),
                ffi::GST_EVENT_STREAM_GROUP_DONE => StreamGroupDone::view(self),
                ffi::GST_EVENT_EOS => Eos::view(self),
                ffi::GST_EVENT_TOC => Toc::view(self),
                ffi::GST_EVENT_PROTECTION => Protection::view(self),
                ffi::GST_EVENT_SEGMENT_DONE => SegmentDone::view(self),
                ffi::GST_EVENT_GAP => Gap::view(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_EVENT_INSTANT_RATE_CHANGE => InstantRateChange::view(self),
                ffi::GST_EVENT_QOS => Qos::view(self),
                ffi::GST_EVENT_SEEK => Seek::view(self),
                ffi::GST_EVENT_NAVIGATION => Navigation::view(self),
                ffi::GST_EVENT_LATENCY => Latency::view(self),
                ffi::GST_EVENT_STEP => Step::view(self),
                ffi::GST_EVENT_RECONFIGURE => Reconfigure::view(self),
                ffi::GST_EVENT_TOC_SELECT => TocSelect::view(self),
                ffi::GST_EVENT_SELECT_STREAMS => SelectStreams::view(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_EVENT_INSTANT_RATE_SYNC_TIME => InstantRateSyncTime::view(self),
                ffi::GST_EVENT_CUSTOM_UPSTREAM => CustomUpstream::view(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM => CustomDownstream::view(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB => CustomDownstreamOob::view(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY => CustomDownstreamSticky::view(self),
                ffi::GST_EVENT_CUSTOM_BOTH => CustomBoth::view(self),
                ffi::GST_EVENT_CUSTOM_BOTH_OOB => CustomBothOob::view(self),
                _ => Other::view(self),
            }
        }
    }

    pub fn view_mut(&mut self) -> EventViewMut {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_EVENT_FLUSH_START => FlushStart::view_mut(self),
                ffi::GST_EVENT_FLUSH_STOP => FlushStop::view_mut(self),
                ffi::GST_EVENT_STREAM_START => StreamStart::view_mut(self),
                ffi::GST_EVENT_CAPS => Caps::view_mut(self),
                ffi::GST_EVENT_SEGMENT => Segment::view_mut(self),
                ffi::GST_EVENT_STREAM_COLLECTION => StreamCollection::view_mut(self),
                ffi::GST_EVENT_TAG => Tag::view_mut(self),
                ffi::GST_EVENT_BUFFERSIZE => Buffersize::view_mut(self),
                ffi::GST_EVENT_SINK_MESSAGE => SinkMessage::view_mut(self),
                ffi::GST_EVENT_STREAM_GROUP_DONE => StreamGroupDone::view_mut(self),
                ffi::GST_EVENT_EOS => Eos::view_mut(self),
                ffi::GST_EVENT_TOC => Toc::view_mut(self),
                ffi::GST_EVENT_PROTECTION => Protection::view_mut(self),
                ffi::GST_EVENT_SEGMENT_DONE => SegmentDone::view_mut(self),
                ffi::GST_EVENT_GAP => Gap::view_mut(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_EVENT_INSTANT_RATE_CHANGE => InstantRateChange::view_mut(self),
                ffi::GST_EVENT_QOS => Qos::view_mut(self),
                ffi::GST_EVENT_SEEK => Seek::view_mut(self),
                ffi::GST_EVENT_NAVIGATION => Navigation::view_mut(self),
                ffi::GST_EVENT_LATENCY => Latency::view_mut(self),
                ffi::GST_EVENT_STEP => Step::view_mut(self),
                ffi::GST_EVENT_RECONFIGURE => Reconfigure::view_mut(self),
                ffi::GST_EVENT_TOC_SELECT => TocSelect::view_mut(self),
                ffi::GST_EVENT_SELECT_STREAMS => SelectStreams::view_mut(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_EVENT_INSTANT_RATE_SYNC_TIME => InstantRateSyncTime::view_mut(self),
                ffi::GST_EVENT_CUSTOM_UPSTREAM => CustomUpstream::view_mut(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM => CustomDownstream::view_mut(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB => CustomDownstreamOob::view_mut(self),
                ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY => CustomDownstreamSticky::view_mut(self),
                ffi::GST_EVENT_CUSTOM_BOTH => CustomBoth::view_mut(self),
                ffi::GST_EVENT_CUSTOM_BOTH_OOB => CustomBothOob::view_mut(self),
                _ => Other::view_mut(self),
            }
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        EventRef::fmt(self, f)
    }
}

impl fmt::Debug for EventRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("ptr", &self.as_ptr())
            .field("type", &self.type_().name())
            .field("seqnum", &self.seqnum())
            .field("structure", &self.structure())
            .finish()
    }
}

pub trait StickyEventType: ToOwned {
    const TYPE: EventType;

    unsafe fn from_event(event: Event) -> Self::Owned;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EventView<'a> {
    FlushStart(&'a FlushStart),
    FlushStop(&'a FlushStop),
    StreamStart(&'a StreamStart),
    Caps(&'a Caps),
    Segment(&'a Segment),
    StreamCollection(&'a StreamCollection),
    Tag(&'a Tag),
    Buffersize(&'a Buffersize),
    SinkMessage(&'a SinkMessage),
    StreamGroupDone(&'a StreamGroupDone),
    Eos(&'a Eos),
    Toc(&'a Toc),
    Protection(&'a Protection),
    SegmentDone(&'a SegmentDone),
    Gap(&'a Gap),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateChange(&'a InstantRateChange),
    Qos(&'a Qos),
    Seek(&'a Seek),
    Navigation(&'a Navigation),
    Latency(&'a Latency),
    Step(&'a Step),
    Reconfigure(&'a Reconfigure),
    TocSelect(&'a TocSelect),
    SelectStreams(&'a SelectStreams),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateSyncTime(&'a InstantRateSyncTime),
    CustomUpstream(&'a CustomUpstream),
    CustomDownstream(&'a CustomDownstream),
    CustomDownstreamOob(&'a CustomDownstreamOob),
    CustomDownstreamSticky(&'a CustomDownstreamSticky),
    CustomBoth(&'a CustomBoth),
    CustomBothOob(&'a CustomBothOob),
    Other(&'a Other),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EventViewMut<'a> {
    FlushStart(&'a mut FlushStart),
    FlushStop(&'a mut FlushStop),
    StreamStart(&'a mut StreamStart),
    Caps(&'a mut Caps),
    Segment(&'a mut Segment),
    StreamCollection(&'a mut StreamCollection),
    Tag(&'a mut Tag),
    Buffersize(&'a mut Buffersize),
    SinkMessage(&'a mut SinkMessage),
    StreamGroupDone(&'a mut StreamGroupDone),
    Eos(&'a mut Eos),
    Toc(&'a mut Toc),
    Protection(&'a mut Protection),
    SegmentDone(&'a mut SegmentDone),
    Gap(&'a mut Gap),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateChange(&'a mut InstantRateChange),
    Qos(&'a mut Qos),
    Seek(&'a mut Seek),
    Navigation(&'a mut Navigation),
    Latency(&'a mut Latency),
    Step(&'a mut Step),
    Reconfigure(&'a mut Reconfigure),
    TocSelect(&'a mut TocSelect),
    SelectStreams(&'a mut SelectStreams),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateSyncTime(&'a mut InstantRateSyncTime),
    CustomUpstream(&'a mut CustomUpstream),
    CustomDownstream(&'a mut CustomDownstream),
    CustomDownstreamOob(&'a mut CustomDownstreamOob),
    CustomDownstreamSticky(&'a mut CustomDownstreamSticky),
    CustomBoth(&'a mut CustomBoth),
    CustomBothOob(&'a mut CustomBothOob),
    Other(&'a mut Other),
}

macro_rules! declare_concrete_event {
    (@sticky $name:ident, $param:ident) => {
        declare_concrete_event!($name, $param);

        impl StickyEventType for $name {
            const TYPE: EventType = EventType::$name;

            #[inline]
            unsafe fn from_event(event: Event) -> Self::Owned {
                $name::<Event>(event)
            }
        }
    };
    ($name:ident, $param:ident) => {
        #[repr(transparent)]
        pub struct $name<$param = EventRef>($param);

        impl $name {
            #[inline]
            pub fn event(&self) -> &EventRef {
                unsafe { &*(self as *const Self as *const EventRef) }
            }

            #[inline]
            pub fn event_mut(&mut self) -> &mut EventRef {
                unsafe { &mut *(self as *mut Self as *mut EventRef) }
            }

            #[inline]
            unsafe fn view(event: &EventRef) -> EventView<'_> {
                let event = &*(event as *const EventRef as *const Self);
                EventView::$name(event)
            }

            #[inline]
            unsafe fn view_mut(event: &mut EventRef) -> EventViewMut<'_> {
                let event = &mut *(event as *mut EventRef as *mut Self);
                EventViewMut::$name(event)
            }
        }

        impl Deref for $name {
            type Target = EventRef;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.event()
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.event_mut()
            }
        }

        impl ToOwned for $name {
            type Owned = $name<Event>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                $name::<Event>(self.copy())
            }
        }

        impl $name<Event> {
            #[inline]
            pub fn get_mut(&mut self) -> Option<&mut $name> {
                self.0
                    .get_mut()
                    .map(|event| unsafe { &mut *(event as *mut EventRef as *mut $name) })
            }
        }

        impl Deref for $name<Event> {
            type Target = $name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.0.as_ptr() as *const Self::Target) }
            }
        }

        impl DerefMut for $name<Event> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                debug_assert!(self.0.is_writable());
                unsafe { &mut *(self.0.as_mut_ptr() as *mut Self::Target) }
            }
        }

        impl Borrow<$name> for $name<Event> {
            #[inline]
            fn borrow(&self) -> &$name {
                &*self
            }
        }

        impl From<$name<Event>> for Event {
            #[inline]
            fn from(concrete: $name<Event>) -> Self {
                skip_assert_initialized!();
                concrete.0
            }
        }
    };
}

declare_concrete_event!(FlushStart, T);
impl FlushStart<Event> {
    #[doc(alias = "gst_event_new_flush_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> FlushStartBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStartBuilder::new()
    }
}

impl std::fmt::Debug for FlushStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlushStart")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for FlushStart<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FlushStart::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(FlushStop, T);
impl FlushStop<Event> {
    #[doc(alias = "gst_event_new_flush_stop")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(reset_time: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(reset_time).build()
    }

    pub fn builder<'a>(reset_time: bool) -> FlushStopBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStopBuilder::new(reset_time)
    }
}

impl FlushStop {
    #[doc(alias = "get_reset_time")]
    #[doc(alias = "gst_event_parse_flush_stop")]
    pub fn resets_time(&self) -> bool {
        unsafe {
            let mut reset_time = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_flush_stop(self.as_mut_ptr(), reset_time.as_mut_ptr());

            from_glib(reset_time.assume_init())
        }
    }
}

impl std::fmt::Debug for FlushStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlushStop")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("resets-time", &self.resets_time())
            .finish()
    }
}

impl std::fmt::Debug for FlushStop<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FlushStop::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky StreamStart, T);
impl StreamStart<Event> {
    #[doc(alias = "gst_event_new_stream_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream_id: &str) -> Event {
        skip_assert_initialized!();
        Self::builder(stream_id).build()
    }

    pub fn builder(stream_id: &str) -> StreamStartBuilder {
        assert_initialized_main_thread!();
        StreamStartBuilder::new(stream_id)
    }
}

impl StreamStart {
    #[doc(alias = "get_stream_id")]
    #[doc(alias = "gst_event_parse_stream_start")]
    pub fn stream_id(&self) -> &str {
        unsafe {
            let mut stream_id = ptr::null();

            ffi::gst_event_parse_stream_start(self.as_mut_ptr(), &mut stream_id);
            CStr::from_ptr(stream_id).to_str().unwrap()
        }
    }

    #[doc(alias = "get_stream_flags")]
    #[doc(alias = "gst_event_parse_stream_flags")]
    pub fn stream_flags(&self) -> crate::StreamFlags {
        unsafe {
            let mut stream_flags = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_stream_flags(self.as_mut_ptr(), stream_flags.as_mut_ptr());

            from_glib(stream_flags.assume_init())
        }
    }

    #[doc(alias = "get_group_id")]
    #[doc(alias = "gst_event_parse_group_id")]
    pub fn group_id(&self) -> Option<GroupId> {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_group_id(self.as_mut_ptr(), group_id.as_mut_ptr());

            let group_id = group_id.assume_init();
            if group_id == 0 {
                None
            } else {
                Some(GroupId(NonZeroU32::new_unchecked(group_id)))
            }
        }
    }

    #[doc(alias = "gst_event_set_group_id")]
    pub fn set_group_id(&mut self, group_id: GroupId) {
        unsafe {
            ffi::gst_event_set_group_id(self.as_mut_ptr(), group_id.0.get());
        }
    }

    #[doc(alias = "get_stream")]
    #[doc(alias = "gst_event_parse_stream")]
    pub fn stream(&self) -> Option<crate::Stream> {
        unsafe {
            let mut stream = ptr::null_mut();
            ffi::gst_event_parse_stream(self.as_mut_ptr(), &mut stream);
            from_glib_full(stream)
        }
    }
}

impl std::fmt::Debug for StreamStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamStart")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("stream-id", &self.stream_id())
            .field("stream-flags", &self.stream_flags())
            .field("group-id", &self.group_id())
            .field("stream", &self.stream())
            .finish()
    }
}

impl std::fmt::Debug for StreamStart<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamStart::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Caps, T);
impl Caps<Event> {
    #[doc(alias = "gst_event_new_caps")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(caps: &crate::Caps) -> Event {
        skip_assert_initialized!();
        Self::builder(caps).build()
    }

    pub fn builder(caps: &crate::Caps) -> CapsBuilder {
        assert_initialized_main_thread!();
        CapsBuilder::new(caps)
    }
}

impl Caps {
    #[doc(alias = "get_caps")]
    #[doc(alias = "gst_event_parse_caps")]
    pub fn caps(&self) -> &crate::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();

            ffi::gst_event_parse_caps(self.as_mut_ptr(), &mut caps);
            crate::CapsRef::from_ptr(caps)
        }
    }

    #[doc(alias = "get_caps_owned")]
    #[doc(alias = "gst_event_parse_caps")]
    pub fn caps_owned(&self) -> crate::Caps {
        unsafe { from_glib_none(self.caps().as_ptr()) }
    }
}

impl std::fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Caps")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("caps", &self.caps())
            .finish()
    }
}

impl std::fmt::Debug for Caps<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Caps::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Segment, T);
impl Segment<Event> {
    #[doc(alias = "gst_event_new_segment")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<F: FormattedValueIntrinsic>(segment: &crate::FormattedSegment<F>) -> Event {
        skip_assert_initialized!();
        Self::builder(segment).build()
    }

    pub fn builder<F: FormattedValueIntrinsic>(
        segment: &crate::FormattedSegment<F>,
    ) -> SegmentBuilder {
        assert_initialized_main_thread!();
        SegmentBuilder::new(segment.as_ref())
    }
}

impl Segment {
    #[doc(alias = "get_segment")]
    #[doc(alias = "gst_event_parse_segment")]
    pub fn segment(&self) -> &crate::Segment {
        unsafe {
            let mut segment = ptr::null();

            ffi::gst_event_parse_segment(self.as_mut_ptr(), &mut segment);
            &*(segment as *mut ffi::GstSegment as *mut crate::Segment)
        }
    }
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Segment")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("segment", &self.segment())
            .finish()
    }
}

impl std::fmt::Debug for Segment<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Segment::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky StreamCollection, T);
impl StreamCollection<Event> {
    #[doc(alias = "gst_event_new_stream_collection")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream_collection: &crate::StreamCollection) -> Event {
        skip_assert_initialized!();
        Self::builder(stream_collection).build()
    }

    pub fn builder(stream_collection: &crate::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(stream_collection)
    }
}

impl StreamCollection {
    #[doc(alias = "get_stream_collection")]
    #[doc(alias = "gst_event_parse_stream_collection")]
    pub fn stream_collection(&self) -> crate::StreamCollection {
        unsafe {
            let mut stream_collection = ptr::null_mut();

            ffi::gst_event_parse_stream_collection(self.as_mut_ptr(), &mut stream_collection);
            from_glib_full(stream_collection)
        }
    }
}

impl std::fmt::Debug for StreamCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamCollection")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("stream-collection", &self.stream_collection())
            .finish()
    }
}

impl std::fmt::Debug for StreamCollection<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamCollection::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Tag, T);
impl Tag<Event> {
    #[doc(alias = "gst_event_new_tag")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(tags: crate::TagList) -> Event {
        skip_assert_initialized!();
        Self::builder(tags).build()
    }

    pub fn builder<'a>(tags: crate::TagList) -> TagBuilder<'a> {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }
}

impl Tag {
    #[doc(alias = "get_tag")]
    #[doc(alias = "gst_event_parse_tag")]
    pub fn tag(&self) -> &crate::TagListRef {
        unsafe {
            let mut tags = ptr::null_mut();

            ffi::gst_event_parse_tag(self.as_mut_ptr(), &mut tags);
            crate::TagListRef::from_ptr(tags)
        }
    }

    #[doc(alias = "get_tag_owned")]
    #[doc(alias = "gst_event_parse_tag")]
    pub fn tag_owned(&self) -> crate::TagList {
        unsafe { from_glib_none(self.tag().as_ptr()) }
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tag")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("tag", &self.tag())
            .finish()
    }
}

impl std::fmt::Debug for Tag<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Tag::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Buffersize, T);
impl Buffersize<Event> {
    #[doc(alias = "gst_event_new_buffer_size")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: FormattedValue>(
        minsize: V,
        maxsize: impl CompatibleFormattedValue<V>,
        r#async: bool,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(minsize, maxsize, r#async).build()
    }

    pub fn builder<'a, V: FormattedValue>(
        minsize: V,
        maxsize: impl CompatibleFormattedValue<V>,
        r#async: bool,
    ) -> BuffersizeBuilder<'a> {
        assert_initialized_main_thread!();
        let maxsize = maxsize.try_into_checked(minsize).unwrap();

        BuffersizeBuilder::new(minsize.into(), maxsize.into(), r#async)
    }
}

impl Buffersize {
    #[doc(alias = "gst_event_parse_buffer_size")]
    pub fn get(&self) -> (GenericFormattedValue, GenericFormattedValue, bool) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut minsize = mem::MaybeUninit::uninit();
            let mut maxsize = mem::MaybeUninit::uninit();
            let mut async_ = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_buffer_size(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                minsize.as_mut_ptr(),
                maxsize.as_mut_ptr(),
                async_.as_mut_ptr(),
            );
            (
                GenericFormattedValue::new(from_glib(fmt.assume_init()), minsize.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), maxsize.assume_init()),
                from_glib(async_.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for Buffersize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (minsize, maxsize, async_) = self.get();
        f.debug_struct("Buffersize")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("min-size", &minsize)
            .field("max-size", &maxsize)
            .field("async", &async_)
            .finish()
    }
}

impl std::fmt::Debug for Buffersize<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Buffersize::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky SinkMessage, T);
impl SinkMessage<Event> {
    #[doc(alias = "gst_event_new_sink_message")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(name: &str, msg: &crate::Message) -> Event {
        skip_assert_initialized!();
        Self::builder(name, msg).build()
    }

    pub fn builder<'a>(name: &'a str, msg: &'a crate::Message) -> SinkMessageBuilder<'a> {
        assert_initialized_main_thread!();
        SinkMessageBuilder::new(name, msg)
    }
}

impl SinkMessage {
    #[doc(alias = "get_message")]
    #[doc(alias = "gst_event_parse_sink_message")]
    pub fn message(&self) -> crate::Message {
        unsafe {
            let mut msg = ptr::null_mut();

            ffi::gst_event_parse_sink_message(self.as_mut_ptr(), &mut msg);
            from_glib_full(msg)
        }
    }
}

impl std::fmt::Debug for SinkMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SinkMessage")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("message", &self.message())
            .finish()
    }
}

impl std::fmt::Debug for SinkMessage<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SinkMessage::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky StreamGroupDone, T);
impl StreamGroupDone<Event> {
    #[doc(alias = "gst_event_new_stream_group_done")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(group_id: GroupId) -> Event {
        skip_assert_initialized!();
        Self::builder(group_id).build()
    }

    pub fn builder<'a>(group_id: GroupId) -> StreamGroupDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StreamGroupDoneBuilder::new(group_id)
    }
}

impl StreamGroupDone {
    #[doc(alias = "get_group_id")]
    #[doc(alias = "gst_event_parse_stream_group_done")]
    pub fn group_id(&self) -> GroupId {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_stream_group_done(self.as_mut_ptr(), group_id.as_mut_ptr());

            let group_id = group_id.assume_init();
            debug_assert_ne!(group_id, 0);
            GroupId(NonZeroU32::new_unchecked(group_id))
        }
    }
}

impl std::fmt::Debug for StreamGroupDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamGroupDone")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("group-id", &self.group_id())
            .finish()
    }
}

impl std::fmt::Debug for StreamGroupDone<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamGroupDone::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Eos, T);
impl Eos<Event> {
    #[doc(alias = "gst_event_new_eos")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }
}

impl std::fmt::Debug for Eos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Eos")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for Eos<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Eos::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Toc, T);
impl Toc<Event> {
    // FIXME could use false for updated as default
    // Even better: use an enum for updated so that it is more explicit than true / false
    #[doc(alias = "gst_event_new_toc")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(toc: &crate::Toc, updated: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(toc, updated).build()
    }

    pub fn builder(toc: &crate::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }
}

impl Toc {
    #[doc(alias = "get_toc")]
    #[doc(alias = "gst_event_parse_toc")]
    pub fn toc(&self) -> (&crate::TocRef, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_toc(self.as_mut_ptr(), &mut toc, updated.as_mut_ptr());
            (
                crate::TocRef::from_ptr(toc),
                from_glib(updated.assume_init()),
            )
        }
    }

    #[doc(alias = "get_toc_owned")]
    #[doc(alias = "gst_event_parse_toc")]
    pub fn toc_owned(&self) -> (crate::Toc, bool) {
        unsafe {
            let (toc, updated) = self.toc();
            (from_glib_none(toc.as_ptr()), updated)
        }
    }
}

impl std::fmt::Debug for Toc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Toc")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("toc", &self.toc())
            .finish()
    }
}

impl std::fmt::Debug for Toc<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Toc::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky Protection, T);
impl Protection<Event> {
    #[doc(alias = "gst_event_new_protection")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(system_id: &str, data: &crate::Buffer) -> Event {
        skip_assert_initialized!();
        Self::builder(system_id, data).build()
    }

    pub fn builder<'a>(system_id: &'a str, data: &'a crate::Buffer) -> ProtectionBuilder<'a> {
        assert_initialized_main_thread!();
        ProtectionBuilder::new(system_id, data)
    }
}

impl Protection {
    #[doc(alias = "gst_event_parse_protection")]
    pub fn get(&self) -> (&str, &crate::BufferRef, Option<&str>) {
        unsafe {
            let mut system_id = ptr::null();
            let mut buffer = ptr::null_mut();
            let mut origin = ptr::null();

            ffi::gst_event_parse_protection(
                self.as_mut_ptr(),
                &mut system_id,
                &mut buffer,
                &mut origin,
            );

            (
                CStr::from_ptr(system_id).to_str().unwrap(),
                crate::BufferRef::from_ptr(buffer),
                if origin.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(origin).to_str().unwrap())
                },
            )
        }
    }

    #[doc(alias = "gst_event_parse_protection")]
    pub fn get_owned(&self) -> (&str, crate::Buffer, Option<&str>) {
        unsafe {
            let (system_id, buffer, origin) = self.get();
            (system_id, from_glib_none(buffer.as_ptr()), origin)
        }
    }
}

impl std::fmt::Debug for Protection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (system_id, buffer, origin) = self.get();
        f.debug_struct("Protection")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("system-id", &system_id)
            .field("buffer", &buffer)
            .field("origin", &origin)
            .finish()
    }
}

impl std::fmt::Debug for Protection<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Protection::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(SegmentDone, T);
impl SegmentDone<Event> {
    #[doc(alias = "gst_event_new_segment_done")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(position: impl FormattedValue) -> Event {
        skip_assert_initialized!();
        Self::builder(position).build()
    }

    pub fn builder<'a>(position: impl FormattedValue) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        SegmentDoneBuilder::new(position.into())
    }
}

impl SegmentDone {
    #[doc(alias = "gst_event_parse_segment_done")]
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_segment_done(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(fmt.assume_init()), position.assume_init())
        }
    }
}

impl std::fmt::Debug for SegmentDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SegmentDone")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("segment", &self.get())
            .finish()
    }
}

impl std::fmt::Debug for SegmentDone<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SegmentDone::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Gap, T);
impl Gap<Event> {
    #[doc(alias = "gst_event_new_gap")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(timestamp: ClockTime, duration: impl Into<Option<ClockTime>>) -> Event {
        skip_assert_initialized!();
        Self::builder(timestamp).duration(duration).build()
    }

    pub fn builder<'a>(timestamp: ClockTime) -> GapBuilder<'a> {
        assert_initialized_main_thread!();
        GapBuilder::new(timestamp)
    }
}

impl Gap {
    #[doc(alias = "gst_event_parse_gap")]
    pub fn get(&self) -> (ClockTime, Option<ClockTime>) {
        unsafe {
            let mut timestamp = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_gap(
                self.as_mut_ptr(),
                timestamp.as_mut_ptr(),
                duration.as_mut_ptr(),
            );

            (
                try_from_glib(timestamp.assume_init()).expect("undefined timestamp"),
                from_glib(duration.assume_init()),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_event_parse_gap_flags")]
    pub fn gap_flags(&self) -> crate::GapFlags {
        unsafe {
            let mut flags = mem::MaybeUninit::uninit();
            ffi::gst_event_parse_gap_flags(self.as_mut_ptr(), flags.as_mut_ptr());
            from_glib(flags.assume_init())
        }
    }
}

impl std::fmt::Debug for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (timestamp, duration) = self.get();
        let mut f = f.debug_struct("Gap");
        f.field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("timestamp", &timestamp)
            .field("duration", &duration);
        #[cfg(feature = "v1_20")]
        f.field("flags", &self.gap_flags());
        f.finish()
    }
}

impl std::fmt::Debug for Gap<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Gap::<EventRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
declare_concrete_event!(@sticky InstantRateChange, T);
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl InstantRateChange<Event> {
    #[doc(alias = "gst_event_new_instant_rate_change")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(multiplier: f64, new_flags: crate::SegmentFlags) -> Event {
        skip_assert_initialized!();
        Self::builder(multiplier, new_flags).build()
    }

    pub fn builder<'a>(
        multiplier: f64,
        new_flags: crate::SegmentFlags,
    ) -> InstantRateChangeBuilder<'a> {
        assert_initialized_main_thread!();
        InstantRateChangeBuilder::new(multiplier, new_flags)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl InstantRateChange {
    #[doc(alias = "gst_event_parse_instant_rate_change")]
    pub fn get(&self) -> (f64, crate::SegmentFlags) {
        unsafe {
            let mut multiplier = mem::MaybeUninit::uninit();
            let mut new_flags = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_instant_rate_change(
                self.as_mut_ptr(),
                multiplier.as_mut_ptr(),
                new_flags.as_mut_ptr(),
            );

            (multiplier.assume_init(), from_glib(new_flags.assume_init()))
        }
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (multiplier, new_flags) = self.get();
        f.debug_struct("InstantRateChange")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("multiplier", &multiplier)
            .field("new-flags", &new_flags)
            .finish()
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateChange<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        InstantRateChange::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Qos, T);
impl Qos<Event> {
    #[doc(alias = "gst_event_new_qos")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        type_: crate::QOSType,
        proportion: f64,
        diff: i64,
        timestamp: impl Into<Option<ClockTime>>,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(type_, proportion, diff)
            .timestamp(timestamp)
            .build()
    }

    pub fn builder<'a>(type_: crate::QOSType, proportion: f64, diff: i64) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(type_, proportion, diff)
    }
}

impl Qos {
    #[doc(alias = "gst_event_parse_qos")]
    pub fn get(&self) -> (crate::QOSType, f64, i64, Option<ClockTime>) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut proportion = mem::MaybeUninit::uninit();
            let mut diff = mem::MaybeUninit::uninit();
            let mut timestamp = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_qos(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                proportion.as_mut_ptr(),
                diff.as_mut_ptr(),
                timestamp.as_mut_ptr(),
            );

            (
                from_glib(type_.assume_init()),
                proportion.assume_init(),
                diff.assume_init(),
                from_glib(timestamp.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for Qos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (type_, proportion, diff, timestamp) = self.get();
        f.debug_struct("Qos")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("type", &type_)
            .field("proportion", &proportion)
            .field("diff", &diff)
            .field("timestamp", &timestamp)
            .finish()
    }
}

impl std::fmt::Debug for Qos<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Qos::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Seek, T);
impl Seek<Event> {
    #[doc(alias = "gst_event_new_seek")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: FormattedValue>(
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: impl CompatibleFormattedValue<V>,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(rate, flags, start_type, start, stop_type, stop).build()
    }

    pub fn builder<'a, V: FormattedValue>(
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: V,
        stop_type: crate::SeekType,
        stop: impl CompatibleFormattedValue<V>,
    ) -> SeekBuilder<'a> {
        assert_initialized_main_thread!();
        let stop = stop.try_into_checked(start).unwrap();

        SeekBuilder::new(
            rate,
            flags,
            start_type,
            start.into(),
            stop_type,
            stop.into(),
        )
    }
}

impl Seek {
    #[doc(alias = "gst_event_parse_seek")]
    pub fn get(
        &self,
    ) -> (
        f64,
        crate::SeekFlags,
        crate::SeekType,
        GenericFormattedValue,
        crate::SeekType,
        GenericFormattedValue,
    ) {
        unsafe {
            let mut rate = mem::MaybeUninit::uninit();
            let mut fmt = mem::MaybeUninit::uninit();
            let mut flags = mem::MaybeUninit::uninit();
            let mut start_type = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut stop_type = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_seek(
                self.as_mut_ptr(),
                rate.as_mut_ptr(),
                fmt.as_mut_ptr(),
                flags.as_mut_ptr(),
                start_type.as_mut_ptr(),
                start.as_mut_ptr(),
                stop_type.as_mut_ptr(),
                stop.as_mut_ptr(),
            );

            (
                rate.assume_init(),
                from_glib(flags.assume_init()),
                from_glib(start_type.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), start.assume_init()),
                from_glib(stop_type.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), stop.assume_init()),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_trickmode_interval")]
    #[doc(alias = "gst_event_parse_seek_trickmode_interval")]
    pub fn trickmode_interval(&self) -> Option<ClockTime> {
        unsafe {
            let mut trickmode_interval = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_seek_trickmode_interval(
                self.as_mut_ptr(),
                trickmode_interval.as_mut_ptr(),
            );

            from_glib(trickmode_interval.assume_init())
        }
    }
}

impl std::fmt::Debug for Seek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rate, flags, start_type, start, stop_type, stop) = self.get();
        f.debug_struct("Seek")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("rate", &rate)
            .field("flags", &flags)
            .field("start-type", &start_type)
            .field("start", &start)
            .field("stop-type", &stop_type)
            .field("stop", &stop)
            .finish()
    }
}

impl std::fmt::Debug for Seek<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Seek::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Navigation, T);
impl Navigation<Event> {
    #[doc(alias = "gst_event_new_navigation")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> NavigationBuilder<'a> {
        assert_initialized_main_thread!();
        NavigationBuilder::new(structure)
    }
}

impl std::fmt::Debug for Navigation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Navigation")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for Navigation<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Navigation::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Latency, T);
impl Latency<Event> {
    #[doc(alias = "gst_event_new_latency")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(latency: ClockTime) -> Event {
        skip_assert_initialized!();
        Self::builder(latency).build()
    }

    pub fn builder<'a>(latency: ClockTime) -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new(latency)
    }
}

impl Latency {
    #[doc(alias = "get_latency")]
    #[doc(alias = "gst_event_parse_latency")]
    pub fn latency(&self) -> ClockTime {
        unsafe {
            let mut latency = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_latency(self.as_mut_ptr(), latency.as_mut_ptr());

            try_from_glib(latency.assume_init()).expect("undefined latency")
        }
    }
}

impl std::fmt::Debug for Latency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latency")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("latency", &self.latency())
            .finish()
    }
}

impl std::fmt::Debug for Latency<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Latency::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Step, T);
impl Step<Event> {
    #[doc(alias = "gst_event_new_step")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(amount: impl FormattedValue, rate: f64, flush: bool, intermediate: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(amount, rate, flush, intermediate).build()
    }

    pub fn builder<'a>(
        amount: impl FormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepBuilder<'a> {
        assert_initialized_main_thread!();
        StepBuilder::new(amount.into(), rate, flush, intermediate)
    }
}

impl Step {
    #[doc(alias = "gst_event_parse_step")]
    pub fn get(&self) -> (GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_step(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(fmt.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (amount, rate, flush, intermediate) = self.get();
        f.debug_struct("Step")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("amount", &amount)
            .field("rate", &rate)
            .field("flush", &flush)
            .field("intermediate", &intermediate)
            .finish()
    }
}

impl std::fmt::Debug for Step<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Step::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Reconfigure, T);
impl Reconfigure<Event> {
    #[doc(alias = "gst_event_new_reconfigure")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> ReconfigureBuilder<'a> {
        assert_initialized_main_thread!();
        ReconfigureBuilder::new()
    }
}

impl std::fmt::Debug for Reconfigure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reconfigure")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for Reconfigure<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Reconfigure::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(TocSelect, T);
impl TocSelect<Event> {
    #[doc(alias = "gst_event_new_toc_select")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(uid: &str) -> Event {
        skip_assert_initialized!();
        Self::builder(uid).build()
    }

    pub fn builder(uid: &str) -> TocSelectBuilder {
        assert_initialized_main_thread!();
        TocSelectBuilder::new(uid)
    }
}

impl TocSelect {
    #[doc(alias = "get_uid")]
    pub fn uid(&self) -> &str {
        unsafe {
            let mut uid = ptr::null_mut();

            ffi::gst_event_parse_toc_select(self.as_mut_ptr(), &mut uid);

            CStr::from_ptr(uid).to_str().unwrap()
        }
    }
}

impl std::fmt::Debug for TocSelect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TocSelect")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("uid", &self.uid())
            .finish()
    }
}

impl std::fmt::Debug for TocSelect<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        TocSelect::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(SelectStreams, T);
impl SelectStreams<Event> {
    #[doc(alias = "gst_event_new_select_streams")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(streams: &[&str]) -> Event {
        skip_assert_initialized!();
        Self::builder(streams).build()
    }

    pub fn builder<'a>(streams: &'a [&'a str]) -> SelectStreamsBuilder<'a> {
        assert_initialized_main_thread!();
        SelectStreamsBuilder::new(streams)
    }
}

impl SelectStreams {
    #[doc(alias = "get_streams")]
    #[doc(alias = "gst_event_parse_select_streams")]
    pub fn streams(&self) -> Vec<String> {
        unsafe {
            let mut streams = ptr::null_mut();

            ffi::gst_event_parse_select_streams(self.as_mut_ptr(), &mut streams);

            FromGlibPtrContainer::from_glib_full(streams)
        }
    }
}

impl std::fmt::Debug for SelectStreams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SelectStreams")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("streams", &self.streams())
            .finish()
    }
}

impl std::fmt::Debug for SelectStreams<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SelectStreams::<EventRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
declare_concrete_event!(InstantRateSyncTime, T);
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl InstantRateSyncTime<Event> {
    #[doc(alias = "gst_event_new_instant_rate_sync_time")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        rate_multiplier: f64,
        running_time: ClockTime,
        upstream_running_time: ClockTime,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(rate_multiplier, running_time, upstream_running_time).build()
    }

    pub fn builder<'a>(
        rate_multiplier: f64,
        running_time: ClockTime,
        upstream_running_time: ClockTime,
    ) -> InstantRateSyncTimeBuilder<'a> {
        assert_initialized_main_thread!();
        InstantRateSyncTimeBuilder::new(rate_multiplier, running_time, upstream_running_time)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl InstantRateSyncTime {
    #[doc(alias = "parse_instant_rate_sync_time")]
    #[doc(alias = "gst_event_parse_instant_rate_sync_time")]
    pub fn get(&self) -> (f64, ClockTime, ClockTime) {
        unsafe {
            let mut rate_multiplier = mem::MaybeUninit::uninit();
            let mut running_time = mem::MaybeUninit::uninit();
            let mut upstream_running_time = mem::MaybeUninit::uninit();

            ffi::gst_event_parse_instant_rate_sync_time(
                self.as_mut_ptr(),
                rate_multiplier.as_mut_ptr(),
                running_time.as_mut_ptr(),
                upstream_running_time.as_mut_ptr(),
            );

            (
                rate_multiplier.assume_init(),
                try_from_glib(running_time.assume_init()).expect("undefined timestamp"),
                try_from_glib(upstream_running_time.assume_init()).expect("undefined timestamp"),
            )
        }
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateSyncTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rate_multiplier, running_time, upstream_running_time) = self.get();
        f.debug_struct("InstantRateSyncTime")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .field("rate-multiplier", &rate_multiplier)
            .field("running-time", &running_time)
            .field("upstream-running-time", &upstream_running_time)
            .finish()
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateSyncTime<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        InstantRateSyncTime::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(CustomUpstream, T);
impl CustomUpstream<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomUpstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomUpstreamBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomUpstream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomUpstream")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomUpstream<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomUpstream::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(CustomDownstream, T);
impl CustomDownstream<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomDownstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomDownstream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomDownstream")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomDownstream<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomDownstream::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(CustomDownstreamOob, T);
impl CustomDownstreamOob<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomDownstreamOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamOobBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomDownstreamOob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomDownstreamOob")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomDownstreamOob<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomDownstreamOob::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(@sticky CustomDownstreamSticky, T);
impl CustomDownstreamSticky<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomDownstreamStickyBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamStickyBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomDownstreamSticky {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomDownstreamSticky")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomDownstreamSticky<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomDownstreamSticky::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(CustomBoth, T);
impl CustomBoth<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomBothBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomBoth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomBoth")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomBoth<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomBoth::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(CustomBothOob, T);
impl CustomBothOob<Event> {
    #[doc(alias = "gst_event_new_custom")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> CustomBothOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothOobBuilder::new(structure)
    }
}

impl std::fmt::Debug for CustomBothOob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomBothOob")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for CustomBothOob<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        CustomBothOob::<EventRef>::fmt(self, f)
    }
}

declare_concrete_event!(Other, T);

impl std::fmt::Debug for Other {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Other")
            .field("seqnum", &self.event().seqnum())
            .field("running-time-offset", &self.event().running_time_offset())
            .field("structure", &self.event().structure())
            .finish()
    }
}

impl std::fmt::Debug for Other<Event> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Other::<EventRef>::fmt(self, f)
    }
}

struct EventBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, glib::SendValue)>,
}

impl<'a> EventBuilder<'a> {
    fn new() -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
        }
    }

    fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    fn running_time_offset(self, running_time_offset: i64) -> Self {
        Self {
            running_time_offset: Some(running_time_offset),
            ..self
        }
    }

    fn other_field(self, name: &'a str, value: impl ToSendValue) -> Self {
        let mut other_fields = self.other_fields;
        other_fields.push((name, value.to_send_value()));

        Self {
            other_fields,
            ..self
        }
    }

    fn other_fields(self, other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))]) -> Self {
        let mut s = self;

        for (name, value) in other_fields {
            s = s.other_field(name, value.to_send_value());
        }

        s
    }
}

macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        #[doc(alias = "gst_event_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[doc(alias = "gst_event_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if(self, seqnum: Seqnum, predicate: bool) -> Self {
            if predicate {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[doc(alias = "gst_event_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if_some(self, seqnum: Option<Seqnum>) -> Self {
            if let Some(seqnum) = seqnum {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[doc(alias = "gst_event_set_running_time_offset")]
        #[allow(clippy::needless_update)]
        pub fn running_time_offset(self, running_time_offset: i64) -> Self {
            Self {
                builder: self.builder.running_time_offset(running_time_offset),
                ..self
            }
        }

        #[doc(alias = "gst_event_set_running_time_offset")]
        #[allow(clippy::needless_update)]
        pub fn running_time_offset_if(self, running_time_offset: i64, predicate: bool) -> Self {
            if predicate {
                self.running_time_offset(running_time_offset)
            } else {
                self
            }
        }

        #[doc(alias = "gst_event_set_running_time_offset")]
        #[allow(clippy::needless_update)]
        pub fn running_time_offset_if_some(self, running_time_offset: Option<i64>) -> Self {
            if let Some(running_time_offset) = running_time_offset {
                self.running_time_offset(running_time_offset)
            } else {
                self
            }
        }

        // rustdoc-stripper-ignore-next
        /// Sets field `name` to the given value `value`.
        ///
        /// Overrides any default or previously defined value for `name`.
        #[allow(clippy::needless_update)]
        pub fn other_field(self, name: &'a str, value: impl ToSendValue) -> Self {
            Self {
                builder: self.builder.other_field(name, value),
                ..self
            }
        }

        impl_builder_gvalue_extra_setters!(other_field);

        #[deprecated = "use build.other_field() instead"]
        #[allow(clippy::needless_update)]
        pub fn other_fields(
            self,
            other_fields: &[(&'a str, &'a (dyn ToSendValue + Sync))],
        ) -> Self {
            Self {
                builder: self.builder.other_fields(other_fields),
                ..self
            }
        }

        #[must_use = "Building the event without using it has no effect"]
        #[allow(clippy::redundant_closure_call)]
        pub fn build(mut self) -> Event {
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.builder.seqnum {
                    ffi::gst_event_set_seqnum(event, seqnum.0.get());
                }

                if let Some(running_time_offset) = self.builder.running_time_offset {
                    ffi::gst_event_set_running_time_offset(event, running_time_offset);
                }

                if !self.builder.other_fields.is_empty() {
                    let s = StructureRef::from_glib_borrow_mut(ffi::gst_event_writable_structure(
                        event,
                    ));

                    for (k, v) in self.builder.other_fields {
                        s.set_value(k, v);
                    }
                }

                from_glib_full(event)
            }
        }
    };
}

#[must_use = "The builder must be built to be used"]
pub struct FlushStartBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> FlushStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| { ffi::gst_event_new_flush_start() });
}

#[must_use = "The builder must be built to be used"]
pub struct FlushStopBuilder<'a> {
    builder: EventBuilder<'a>,
    reset_time: bool,
}
impl<'a> FlushStopBuilder<'a> {
    fn new(reset_time: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            reset_time,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_flush_stop(s.reset_time.into_glib())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StreamStartBuilder<'a> {
    builder: EventBuilder<'a>,
    stream_id: &'a str,
    flags: Option<crate::StreamFlags>,
    group_id: Option<GroupId>,
    stream: Option<crate::Stream>,
}

impl<'a> StreamStartBuilder<'a> {
    fn new(stream_id: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            stream_id,
            flags: None,
            group_id: None,
            stream: None,
        }
    }

    pub fn flags(self, flags: crate::StreamFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn flags_if(self, flags: crate::StreamFlags, predicate: bool) -> Self {
        if predicate {
            self.flags(flags)
        } else {
            self
        }
    }

    pub fn flags_if_some(self, flags: Option<crate::StreamFlags>) -> Self {
        if let Some(flags) = flags {
            self.flags(flags)
        } else {
            self
        }
    }

    pub fn group_id(self, group_id: GroupId) -> Self {
        Self {
            group_id: Some(group_id),
            ..self
        }
    }

    pub fn group_id_if(self, group_id: GroupId, predicate: bool) -> Self {
        if predicate {
            self.group_id(group_id)
        } else {
            self
        }
    }

    pub fn group_id_if_some(self, group_id: Option<GroupId>) -> Self {
        if let Some(group_id) = group_id {
            self.group_id(group_id)
        } else {
            self
        }
    }

    pub fn stream(self, stream: crate::Stream) -> Self {
        Self {
            stream: Some(stream),
            ..self
        }
    }

    pub fn stream_if(self, stream: crate::Stream, predicate: bool) -> Self {
        if predicate {
            self.stream(stream)
        } else {
            self
        }
    }

    pub fn stream_if_some(self, stream: Option<crate::Stream>) -> Self {
        if let Some(stream) = stream {
            self.stream(stream)
        } else {
            self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        let ev = ffi::gst_event_new_stream_start(s.stream_id.to_glib_none().0);
        if let Some(flags) = s.flags {
            ffi::gst_event_set_stream_flags(ev, flags.into_glib());
        }
        if let Some(group_id) = s.group_id {
            ffi::gst_event_set_group_id(ev, group_id.0.get());
        }

        if let Some(ref stream) = s.stream {
            ffi::gst_event_set_stream(ev, stream.to_glib_none().0);
        }

        ev
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CapsBuilder<'a> {
    builder: EventBuilder<'a>,
    caps: &'a crate::Caps,
}

impl<'a> CapsBuilder<'a> {
    fn new(caps: &'a crate::Caps) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            caps,
        }
    }

    event_builder_generic_impl!(|s: &Self| { ffi::gst_event_new_caps(s.caps.as_mut_ptr()) });
}

#[must_use = "The builder must be built to be used"]
pub struct SegmentBuilder<'a> {
    builder: EventBuilder<'a>,
    segment: &'a crate::Segment,
}

impl<'a> SegmentBuilder<'a> {
    fn new(segment: &'a crate::Segment) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            segment,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_segment(s.segment.to_glib_none().0)
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StreamCollectionBuilder<'a> {
    builder: EventBuilder<'a>,
    stream_collection: &'a crate::StreamCollection,
}

impl<'a> StreamCollectionBuilder<'a> {
    fn new(stream_collection: &'a crate::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            stream_collection,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_stream_collection(s.stream_collection.to_glib_none().0)
    });
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
#[must_use = "The builder must be built to be used"]
pub struct InstantRateSyncTimeBuilder<'a> {
    builder: EventBuilder<'a>,
    rate_multiplier: f64,
    running_time: ClockTime,
    upstream_running_time: ClockTime,
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl<'a> InstantRateSyncTimeBuilder<'a> {
    fn new(
        rate_multiplier: f64,
        running_time: ClockTime,
        upstream_running_time: ClockTime,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            rate_multiplier,
            running_time,
            upstream_running_time,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_instant_rate_sync_time(
            s.rate_multiplier,
            s.running_time.into_glib(),
            s.upstream_running_time.into_glib(),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct TagBuilder<'a> {
    builder: EventBuilder<'a>,
    tags: Option<crate::TagList>,
}

impl<'a> TagBuilder<'a> {
    fn new(tags: crate::TagList) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            tags: Some(tags),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let tags = s.tags.take().unwrap();
        ffi::gst_event_new_tag(tags.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct BuffersizeBuilder<'a> {
    builder: EventBuilder<'a>,
    minsize: GenericFormattedValue,
    maxsize: GenericFormattedValue,
    r#async: bool,
}

impl<'a> BuffersizeBuilder<'a> {
    fn new(minsize: GenericFormattedValue, maxsize: GenericFormattedValue, r#async: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            minsize,
            maxsize,
            r#async,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_buffer_size(
            s.minsize.format().into_glib(),
            s.minsize.value(),
            s.maxsize.value(),
            s.r#async.into_glib(),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct SinkMessageBuilder<'a> {
    builder: EventBuilder<'a>,
    name: &'a str,
    msg: &'a crate::Message,
}

impl<'a> SinkMessageBuilder<'a> {
    fn new(name: &'a str, msg: &'a crate::Message) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            name,
            msg,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_sink_message(s.name.to_glib_none().0, s.msg.as_mut_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StreamGroupDoneBuilder<'a> {
    builder: EventBuilder<'a>,
    group_id: GroupId,
}

impl<'a> StreamGroupDoneBuilder<'a> {
    fn new(group_id: GroupId) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            group_id,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_stream_group_done(s.group_id.0.get())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct EosBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_eos());
}

#[must_use = "The builder must be built to be used"]
pub struct TocBuilder<'a> {
    builder: EventBuilder<'a>,
    toc: &'a crate::Toc,
    updated: bool,
}

impl<'a> TocBuilder<'a> {
    fn new(toc: &'a crate::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            toc,
            updated,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_toc(
        s.toc.to_glib_none().0,
        s.updated.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct ProtectionBuilder<'a> {
    builder: EventBuilder<'a>,
    system_id: &'a str,
    data: &'a crate::Buffer,
    origin: Option<&'a str>,
}

impl<'a> ProtectionBuilder<'a> {
    fn new(system_id: &'a str, data: &'a crate::Buffer) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            system_id,
            data,
            origin: None,
        }
    }

    pub fn origin(self, origin: &'a str) -> Self {
        Self {
            origin: Some(origin),
            ..self
        }
    }

    pub fn origin_if(self, origin: &'a str, predicate: bool) -> Self {
        if predicate {
            self.origin(origin)
        } else {
            self
        }
    }

    pub fn origin_if_some(self, origin: Option<&'a str>) -> Self {
        if let Some(origin) = origin {
            self.origin(origin)
        } else {
            self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_protection(
            s.system_id.to_glib_none().0,
            s.data.as_mut_ptr(),
            s.origin.to_glib_none().0,
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct SegmentDoneBuilder<'a> {
    builder: EventBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            position,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_segment_done(s.position.format().into_glib(), s.position.value())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct GapBuilder<'a> {
    builder: EventBuilder<'a>,
    timestamp: ClockTime,
    duration: Option<ClockTime>,
    #[cfg(feature = "v1_20")]
    gap_flags: Option<crate::GapFlags>,
}

impl<'a> GapBuilder<'a> {
    fn new(timestamp: ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            timestamp,
            duration: None,
            #[cfg(feature = "v1_20")]
            gap_flags: None,
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn gap_flags(mut self, flags: crate::GapFlags) -> Self {
        self.gap_flags = Some(flags);
        self
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn gap_flags_if(self, flags: crate::GapFlags, predicate: bool) -> Self {
        if predicate {
            self.gap_flags(flags)
        } else {
            self
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    pub fn gap_flags_if_some(self, flags: Option<crate::GapFlags>) -> Self {
        if let Some(flags) = flags {
            self.gap_flags(flags)
        } else {
            self
        }
    }

    pub fn duration(mut self, duration: impl Into<Option<ClockTime>>) -> Self {
        self.duration = duration.into();
        self
    }

    pub fn duration_if(self, duration: ClockTime, predicate: bool) -> Self {
        if predicate {
            self.duration(duration)
        } else {
            self
        }
    }

    pub fn duration_if_some(self, duration: Option<ClockTime>) -> Self {
        if let Some(duration) = duration {
            self.duration(duration)
        } else {
            self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        #[allow(clippy::let_and_return)]
        let ev = ffi::gst_event_new_gap(s.timestamp.into_glib(), s.duration.into_glib());

        #[cfg(feature = "v1_20")]
        if let Some(ref flags) = s.gap_flags {
            ffi::gst_event_set_gap_flags(ev, flags.into_glib());
        }

        ev
    });
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
#[must_use = "The builder must be built to be used"]
pub struct InstantRateChangeBuilder<'a> {
    builder: EventBuilder<'a>,
    multiplier: f64,
    new_flags: crate::SegmentFlags,
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl<'a> InstantRateChangeBuilder<'a> {
    fn new(multiplier: f64, new_flags: crate::SegmentFlags) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            multiplier,
            new_flags,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_instant_rate_change(
        s.multiplier,
        s.new_flags.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct QosBuilder<'a> {
    builder: EventBuilder<'a>,
    type_: crate::QOSType,
    proportion: f64,
    diff: i64,
    timestamp: Option<ClockTime>,
}

impl<'a> QosBuilder<'a> {
    fn new(type_: crate::QOSType, proportion: f64, diff: i64) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            type_,
            proportion,
            diff,
            timestamp: None,
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<Option<ClockTime>>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn timestamp_if(self, timestamp: ClockTime, predicate: bool) -> Self {
        if predicate {
            self.timestamp(timestamp)
        } else {
            self
        }
    }

    pub fn timestamp_if_some(self, timestamp: Option<ClockTime>) -> Self {
        if let Some(timestamp) = timestamp {
            self.timestamp(timestamp)
        } else {
            self
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_qos(
        s.type_.into_glib(),
        s.proportion,
        s.diff,
        s.timestamp.into_glib(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct SeekBuilder<'a> {
    builder: EventBuilder<'a>,
    rate: f64,
    flags: crate::SeekFlags,
    start_type: crate::SeekType,
    start: GenericFormattedValue,
    stop_type: crate::SeekType,
    stop: GenericFormattedValue,
    #[allow(unused)]
    trickmode_interval: Option<ClockTime>,
}

impl<'a> SeekBuilder<'a> {
    fn new(
        rate: f64,
        flags: crate::SeekFlags,
        start_type: crate::SeekType,
        start: GenericFormattedValue,
        stop_type: crate::SeekType,
        stop: GenericFormattedValue,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            rate,
            flags,
            start_type,
            start,
            stop_type,
            stop,
            trickmode_interval: None,
        }
    }

    pub fn trickmode_interval(mut self, trickmode_interval: impl Into<Option<ClockTime>>) -> Self {
        self.trickmode_interval = trickmode_interval.into();
        self
    }

    event_builder_generic_impl!(|s: &Self| {
        #[allow(clippy::let_and_return)]
        {
            let ev = ffi::gst_event_new_seek(
                s.rate,
                s.start.format().into_glib(),
                s.flags.into_glib(),
                s.start_type.into_glib(),
                s.start.value(),
                s.stop_type.into_glib(),
                s.stop.value(),
            );

            #[cfg(feature = "v1_16")]
            if let Some(trickmode_interval) = s.trickmode_interval {
                ffi::gst_event_set_seek_trickmode_interval(ev, trickmode_interval.into_glib());
            }

            ev
        }
    });
}

#[must_use = "The builder must be built to be used"]
pub struct NavigationBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> NavigationBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_navigation(structure.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct LatencyBuilder<'a> {
    builder: EventBuilder<'a>,
    latency: ClockTime,
}

impl<'a> LatencyBuilder<'a> {
    fn new(latency: ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            latency,
        }
    }

    event_builder_generic_impl!(|s: &Self| { ffi::gst_event_new_latency(s.latency.into_glib()) });
}

#[must_use = "The builder must be built to be used"]
pub struct StepBuilder<'a> {
    builder: EventBuilder<'a>,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
}

impl<'a> StepBuilder<'a> {
    fn new(amount: GenericFormattedValue, rate: f64, flush: bool, intermediate: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            amount,
            rate,
            flush,
            intermediate,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_step(
            s.amount.format().into_glib(),
            s.amount.value() as u64,
            s.rate,
            s.flush.into_glib(),
            s.intermediate.into_glib(),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct ReconfigureBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> ReconfigureBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| { ffi::gst_event_new_reconfigure() });
}

#[must_use = "The builder must be built to be used"]
pub struct TocSelectBuilder<'a> {
    builder: EventBuilder<'a>,
    uid: &'a str,
}

impl<'a> TocSelectBuilder<'a> {
    fn new(uid: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            uid,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_toc_select(s.uid.to_glib_none().0)
    });
}

#[must_use = "The builder must be built to be used"]
pub struct SelectStreamsBuilder<'a> {
    builder: EventBuilder<'a>,
    streams: &'a [&'a str],
}

impl<'a> SelectStreamsBuilder<'a> {
    fn new(streams: &'a [&'a str]) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            streams,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        ffi::gst_event_new_select_streams(s.streams.to_glib_none().0)
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomUpstreamBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomUpstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_UPSTREAM, structure.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomDownstreamBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_DOWNSTREAM, structure.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomDownstreamOobBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB,
            structure.into_glib_ptr(),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomDownstreamStickyBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamStickyBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY,
            structure.into_glib_ptr(),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomBothBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomBothBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_BOTH, structure.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct CustomBothOobBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomBothOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_BOTH_OOB, structure.into_glib_ptr())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_simple() {
        crate::init().unwrap();

        // Event without arguments
        let flush_start_evt = FlushStart::new();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(!flush_start_evt.is_sticky());
                assert!(flush_start_evt.structure().is_none());
            }
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        let flush_start_evt = FlushStart::builder()
            .other_fields(&[("extra-field", &true)])
            .build();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(flush_start_evt.structure().is_some());
                if let Some(other_fields) = flush_start_evt.structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        // Event with arguments
        let flush_stop_evt = FlushStop::builder(true)
            .other_field("extra-field", true)
            .build();
        match flush_stop_evt.view() {
            EventView::FlushStop(flush_stop_evt) => {
                assert!(flush_stop_evt.resets_time());
                assert!(flush_stop_evt.structure().is_some());
                if let Some(other_fields) = flush_stop_evt.structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("flush_stop_evt.view() is not an EventView::FlushStop(_)"),
        }
    }

    #[test]
    fn test_get_structure_mut() {
        crate::init().unwrap();

        let mut flush_start_evt = FlushStart::new();

        {
            let flush_start_evt = flush_start_evt.get_mut().unwrap();
            let structure = flush_start_evt.structure_mut();
            structure.set("test", 42u32);
        }

        let structure = flush_start_evt.structure().unwrap();
        assert_eq!(structure.get("test"), Ok(42u32));
    }

    #[test]
    fn test_view_lifetimes() {
        crate::init().unwrap();

        let caps = crate::Caps::builder("some/x-caps").build();
        let event = crate::event::Caps::new(&caps);

        let caps2 = match event.view() {
            EventView::Caps(caps) => caps.caps(),
            _ => unreachable!(),
        };

        assert_eq!(&*caps, caps2);
    }

    #[test]
    fn test_select_streams() {
        crate::init().unwrap();

        let s = ["foo", "bar"].to_vec();
        let event = crate::event::SelectStreams::new(&s);
        let streams = match event.view() {
            EventView::SelectStreams(streams) => streams.streams(),
            _ => unreachable!(),
        };
        assert_eq!(streams, s);
    }
}
