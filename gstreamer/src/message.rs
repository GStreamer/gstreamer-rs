// Take a look at the license at the top of the repository in the LICENSE file.

use std::{borrow::Borrow, ffi::CStr, fmt, mem, num::NonZeroU32, ops::Deref, ops::DerefMut, ptr};

use glib::{
    translate::*,
    value::{SendValue, ValueType},
};

use crate::{
    ffi,
    format::{CompatibleFormattedValue, FormattedValue},
    prelude::*,
    structure::*,
    GenericFormattedValue, GroupId, MessageType, Object, Seqnum, TagList,
};

mini_object_wrapper!(Message, MessageRef, ffi::GstMessage, || {
    ffi::gst_message_get_type()
});

impl MessageRef {
    #[doc(alias = "get_src")]
    #[inline]
    pub fn src(&self) -> Option<&Object> {
        unsafe {
            if (*self.as_ptr()).src.is_null() {
                None
            } else {
                Some(&*(&(*self.as_ptr()).src as *const *mut ffi::GstObject as *const Object))
            }
        }
    }

    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_message_get_seqnum")]
    pub fn seqnum(&self) -> Seqnum {
        unsafe {
            let seqnum = ffi::gst_message_get_seqnum(self.as_mut_ptr());

            if seqnum == 0 {
                // seqnum for this message is invalid. This can happen with buggy elements
                // overriding the seqnum with GST_SEQNUM_INVALID instead of the expected seqnum.
                // As a workaround, let's generate an unused valid seqnum.
                let next = Seqnum::next();

                crate::warning!(
                    crate::CAT_RUST,
                    "get_seqnum detected invalid seqnum, returning next {:?}",
                    next
                );

                return next;
            }

            Seqnum(NonZeroU32::new_unchecked(seqnum))
        }
    }

    #[doc(alias = "gst_message_set_seqnum")]
    pub fn set_seqnum(&self, seqnum: Seqnum) {
        unsafe {
            ffi::gst_message_set_seqnum(self.as_mut_ptr(), seqnum.0.get());
        }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_message_get_structure")]
    #[inline]
    pub fn structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_message_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    #[doc(alias = "gst_message_writable_structure")]
    #[inline]
    pub fn structure_mut(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(ffi::gst_message_writable_structure(
                self.as_mut_ptr(),
            ))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_writable_details")]
    #[inline]
    pub fn details(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_message_writable_details(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_writable_details")]
    #[inline]
    pub fn details_mut(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(ffi::gst_message_writable_details(self.as_mut_ptr()))
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_set_details")]
    #[inline]
    pub fn set_details(&mut self, structure: Structure) {
        unsafe {
            ffi::gst_message_set_details(self.as_mut_ptr(), structure.into_glib_ptr());
        }
    }

    #[doc(alias = "gst_message_has_name")]
    #[inline]
    pub fn has_name(&self, name: &str) -> bool {
        self.structure().is_some_and(|s| s.has_name(name))
    }

    pub fn view(&self) -> MessageView {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_MESSAGE_EOS => Eos::view(self),
                ffi::GST_MESSAGE_ERROR => Error::view(self),
                ffi::GST_MESSAGE_WARNING => Warning::view(self),
                ffi::GST_MESSAGE_INFO => Info::view(self),
                ffi::GST_MESSAGE_TAG => Tag::view(self),
                ffi::GST_MESSAGE_BUFFERING => Buffering::view(self),
                ffi::GST_MESSAGE_STATE_CHANGED => StateChanged::view(self),
                ffi::GST_MESSAGE_STATE_DIRTY => StateDirty::view(self),
                ffi::GST_MESSAGE_STEP_DONE => StepDone::view(self),
                ffi::GST_MESSAGE_CLOCK_PROVIDE => ClockProvide::view(self),
                ffi::GST_MESSAGE_CLOCK_LOST => ClockLost::view(self),
                ffi::GST_MESSAGE_NEW_CLOCK => NewClock::view(self),
                ffi::GST_MESSAGE_STRUCTURE_CHANGE => StructureChange::view(self),
                ffi::GST_MESSAGE_STREAM_STATUS => StreamStatus::view(self),
                ffi::GST_MESSAGE_APPLICATION => Application::view(self),
                ffi::GST_MESSAGE_ELEMENT => Element::view(self),
                ffi::GST_MESSAGE_SEGMENT_START => SegmentStart::view(self),
                ffi::GST_MESSAGE_SEGMENT_DONE => SegmentDone::view(self),
                ffi::GST_MESSAGE_DURATION_CHANGED => DurationChanged::view(self),
                ffi::GST_MESSAGE_LATENCY => Latency::view(self),
                ffi::GST_MESSAGE_ASYNC_START => AsyncStart::view(self),
                ffi::GST_MESSAGE_ASYNC_DONE => AsyncDone::view(self),
                ffi::GST_MESSAGE_REQUEST_STATE => RequestState::view(self),
                ffi::GST_MESSAGE_STEP_START => StepStart::view(self),
                ffi::GST_MESSAGE_QOS => Qos::view(self),
                ffi::GST_MESSAGE_PROGRESS => Progress::view(self),
                ffi::GST_MESSAGE_TOC => Toc::view(self),
                ffi::GST_MESSAGE_RESET_TIME => ResetTime::view(self),
                ffi::GST_MESSAGE_STREAM_START => StreamStart::view(self),
                ffi::GST_MESSAGE_NEED_CONTEXT => NeedContext::view(self),
                ffi::GST_MESSAGE_HAVE_CONTEXT => HaveContext::view(self),
                ffi::GST_MESSAGE_DEVICE_ADDED => DeviceAdded::view(self),
                ffi::GST_MESSAGE_DEVICE_REMOVED => DeviceRemoved::view(self),
                ffi::GST_MESSAGE_REDIRECT => Redirect::view(self),
                ffi::GST_MESSAGE_PROPERTY_NOTIFY => PropertyNotify::view(self),
                ffi::GST_MESSAGE_STREAM_COLLECTION => StreamCollection::view(self),
                ffi::GST_MESSAGE_STREAMS_SELECTED => StreamsSelected::view(self),
                #[cfg(feature = "v1_16")]
                ffi::GST_MESSAGE_DEVICE_CHANGED => DeviceChanged::view(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_MESSAGE_INSTANT_RATE_REQUEST => InstantRateRequest::view(self),
                _ => MessageView::Other,
            }
        }
    }

    pub fn view_mut(&mut self) -> MessageViewMut {
        unsafe {
            let type_ = (*self.as_ptr()).type_;

            match type_ {
                ffi::GST_MESSAGE_EOS => Eos::view_mut(self),
                ffi::GST_MESSAGE_ERROR => Error::view_mut(self),
                ffi::GST_MESSAGE_WARNING => Warning::view_mut(self),
                ffi::GST_MESSAGE_INFO => Info::view_mut(self),
                ffi::GST_MESSAGE_TAG => Tag::view_mut(self),
                ffi::GST_MESSAGE_BUFFERING => Buffering::view_mut(self),
                ffi::GST_MESSAGE_STATE_CHANGED => StateChanged::view_mut(self),
                ffi::GST_MESSAGE_STATE_DIRTY => StateDirty::view_mut(self),
                ffi::GST_MESSAGE_STEP_DONE => StepDone::view_mut(self),
                ffi::GST_MESSAGE_CLOCK_PROVIDE => ClockProvide::view_mut(self),
                ffi::GST_MESSAGE_CLOCK_LOST => ClockLost::view_mut(self),
                ffi::GST_MESSAGE_NEW_CLOCK => NewClock::view_mut(self),
                ffi::GST_MESSAGE_STRUCTURE_CHANGE => StructureChange::view_mut(self),
                ffi::GST_MESSAGE_STREAM_STATUS => StreamStatus::view_mut(self),
                ffi::GST_MESSAGE_APPLICATION => Application::view_mut(self),
                ffi::GST_MESSAGE_ELEMENT => Element::view_mut(self),
                ffi::GST_MESSAGE_SEGMENT_START => SegmentStart::view_mut(self),
                ffi::GST_MESSAGE_SEGMENT_DONE => SegmentDone::view_mut(self),
                ffi::GST_MESSAGE_DURATION_CHANGED => DurationChanged::view_mut(self),
                ffi::GST_MESSAGE_LATENCY => Latency::view_mut(self),
                ffi::GST_MESSAGE_ASYNC_START => AsyncStart::view_mut(self),
                ffi::GST_MESSAGE_ASYNC_DONE => AsyncDone::view_mut(self),
                ffi::GST_MESSAGE_REQUEST_STATE => RequestState::view_mut(self),
                ffi::GST_MESSAGE_STEP_START => StepStart::view_mut(self),
                ffi::GST_MESSAGE_QOS => Qos::view_mut(self),
                ffi::GST_MESSAGE_PROGRESS => Progress::view_mut(self),
                ffi::GST_MESSAGE_TOC => Toc::view_mut(self),
                ffi::GST_MESSAGE_RESET_TIME => ResetTime::view_mut(self),
                ffi::GST_MESSAGE_STREAM_START => StreamStart::view_mut(self),
                ffi::GST_MESSAGE_NEED_CONTEXT => NeedContext::view_mut(self),
                ffi::GST_MESSAGE_HAVE_CONTEXT => HaveContext::view_mut(self),
                ffi::GST_MESSAGE_DEVICE_ADDED => DeviceAdded::view_mut(self),
                ffi::GST_MESSAGE_DEVICE_REMOVED => DeviceRemoved::view_mut(self),
                ffi::GST_MESSAGE_REDIRECT => Redirect::view_mut(self),
                ffi::GST_MESSAGE_PROPERTY_NOTIFY => PropertyNotify::view_mut(self),
                ffi::GST_MESSAGE_STREAM_COLLECTION => StreamCollection::view_mut(self),
                ffi::GST_MESSAGE_STREAMS_SELECTED => StreamsSelected::view_mut(self),
                #[cfg(feature = "v1_16")]
                ffi::GST_MESSAGE_DEVICE_CHANGED => DeviceChanged::view_mut(self),
                #[cfg(feature = "v1_18")]
                ffi::GST_MESSAGE_INSTANT_RATE_REQUEST => InstantRateRequest::view_mut(self),
                _ => MessageViewMut::Other,
            }
        }
    }

    #[doc(alias = "get_type")]
    #[inline]
    pub fn type_(&self) -> MessageType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MessageRef::fmt(self, f)
    }
}

impl fmt::Debug for MessageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't retrieve `seqnum` using `MessageRef::get_seqnum`
        // because it would generate a new seqnum if a buggy `Element`
        // emitted a `Message` with an invalid `seqnum`.
        // We want to help the user find out there is something wrong here,
        // so they can investigate the origin.
        let seqnum = unsafe { ffi::gst_message_get_seqnum(self.as_mut_ptr()) };
        let seqnum = if seqnum != 0 {
            &seqnum as &dyn fmt::Debug
        } else {
            &"INVALID (0)" as &dyn fmt::Debug
        };

        f.debug_struct("Message")
            .field("ptr", &self.as_ptr())
            .field("type", &unsafe {
                let type_ = ffi::gst_message_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("seqnum", seqnum)
            .field(
                "src",
                &self
                    .src()
                    .map(|s| s.name())
                    .as_ref()
                    .map(glib::GString::as_str),
            )
            .field("structure", &self.structure())
            .finish()
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MessageView<'a> {
    Eos(&'a Eos),
    Error(&'a Error),
    Warning(&'a Warning),
    Info(&'a Info),
    Tag(&'a Tag),
    Buffering(&'a Buffering),
    StateChanged(&'a StateChanged),
    StateDirty(&'a StateDirty),
    StepDone(&'a StepDone),
    ClockProvide(&'a ClockProvide),
    ClockLost(&'a ClockLost),
    NewClock(&'a NewClock),
    StructureChange(&'a StructureChange),
    StreamStatus(&'a StreamStatus),
    Application(&'a Application),
    Element(&'a Element),
    SegmentStart(&'a SegmentStart),
    SegmentDone(&'a SegmentDone),
    DurationChanged(&'a DurationChanged),
    Latency(&'a Latency),
    AsyncStart(&'a AsyncStart),
    AsyncDone(&'a AsyncDone),
    RequestState(&'a RequestState),
    StepStart(&'a StepStart),
    Qos(&'a Qos),
    Progress(&'a Progress),
    Toc(&'a Toc),
    ResetTime(&'a ResetTime),
    StreamStart(&'a StreamStart),
    NeedContext(&'a NeedContext),
    HaveContext(&'a HaveContext),
    DeviceAdded(&'a DeviceAdded),
    DeviceRemoved(&'a DeviceRemoved),
    PropertyNotify(&'a PropertyNotify),
    StreamCollection(&'a StreamCollection),
    StreamsSelected(&'a StreamsSelected),
    Redirect(&'a Redirect),
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    DeviceChanged(&'a DeviceChanged),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateRequest(&'a InstantRateRequest),
    Other,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MessageViewMut<'a> {
    Eos(&'a mut Eos),
    Error(&'a mut Error),
    Warning(&'a mut Warning),
    Info(&'a mut Info),
    Tag(&'a mut Tag),
    Buffering(&'a mut Buffering),
    StateChanged(&'a mut StateChanged),
    StateDirty(&'a mut StateDirty),
    StepDone(&'a mut StepDone),
    ClockProvide(&'a mut ClockProvide),
    ClockLost(&'a mut ClockLost),
    NewClock(&'a mut NewClock),
    StructureChange(&'a mut StructureChange),
    StreamStatus(&'a mut StreamStatus),
    Application(&'a mut Application),
    Element(&'a mut Element),
    SegmentStart(&'a mut SegmentStart),
    SegmentDone(&'a mut SegmentDone),
    DurationChanged(&'a mut DurationChanged),
    Latency(&'a mut Latency),
    AsyncStart(&'a mut AsyncStart),
    AsyncDone(&'a mut AsyncDone),
    RequestState(&'a mut RequestState),
    StepStart(&'a mut StepStart),
    Qos(&'a mut Qos),
    Progress(&'a mut Progress),
    Toc(&'a mut Toc),
    ResetTime(&'a mut ResetTime),
    StreamStart(&'a mut StreamStart),
    NeedContext(&'a mut NeedContext),
    HaveContext(&'a mut HaveContext),
    DeviceAdded(&'a mut DeviceAdded),
    DeviceRemoved(&'a mut DeviceRemoved),
    PropertyNotify(&'a mut PropertyNotify),
    StreamCollection(&'a mut StreamCollection),
    StreamsSelected(&'a mut StreamsSelected),
    Redirect(&'a mut Redirect),
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    DeviceChanged(&'a mut DeviceChanged),
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    InstantRateRequest(&'a mut InstantRateRequest),
    Other,
}

macro_rules! declare_concrete_message(
    ($name:ident, $param:ident) => {
        #[repr(transparent)]
        pub struct $name<$param = MessageRef>($param);

        impl $name {
            #[inline]
            pub fn message(&self) -> &MessageRef {
                unsafe { &*(self as *const Self as *const MessageRef) }
            }

            #[inline]
            pub fn message_mut(&mut self) -> &mut MessageRef {
                unsafe { &mut *(self as *mut Self as *mut MessageRef) }
            }

            #[inline]
            unsafe fn view(message: &MessageRef) -> MessageView<'_> {
                let message = &*(message as *const MessageRef as *const Self);
                MessageView::$name(message)
            }

            #[inline]
            unsafe fn view_mut(message: &mut MessageRef) -> MessageViewMut<'_> {
                let message = &mut *(message as *mut MessageRef as *mut Self);
                MessageViewMut::$name(message)
            }
        }

        impl Deref for $name {
            type Target = MessageRef;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const Self as *const Self::Target)
                }
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.message_mut()
            }
        }

        impl ToOwned for $name {
            type Owned = $name<Message>;

            #[inline]
            fn to_owned(&self) -> Self::Owned {
                $name::<Message>(self.copy())
            }
        }

        impl $name<Message> {
            #[inline]
            pub fn get_mut(&mut self) -> Option<&mut $name> {
                self.0.get_mut().map(|message| unsafe {
                    &mut *(message as *mut MessageRef as *mut $name)
                })
            }
        }

        impl Deref for $name<Message> {
            type Target = $name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self.0.as_ptr() as *const Self::Target) }
            }
        }

        impl DerefMut for $name<Message> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                debug_assert!(self.0.is_writable());
                unsafe { &mut *(self.0.as_mut_ptr() as *mut Self::Target) }
            }
        }

        impl Borrow<$name> for $name<Message> {
            #[inline]
            fn borrow(&self) -> &$name {
                &*self
            }
        }

        impl From<$name<Message>> for Message {
            #[inline]
            fn from(concrete: $name<Message>) -> Self {
                skip_assert_initialized!();
                concrete.0
            }
        }
    }
);

declare_concrete_message!(Eos, T);
impl Eos {
    #[doc(alias = "gst_message_new_eos")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
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
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for Eos<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Eos::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Error, T);
impl Error {
    #[doc(alias = "gst_message_new_error")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> ErrorBuilder {
        assert_initialized_main_thread!();
        ErrorBuilder::new(glib::Error::new(error, message))
    }

    pub fn builder_from_error<'a>(error: glib::Error) -> ErrorBuilder<'a> {
        assert_initialized_main_thread!();

        assert!([
            crate::CoreError::domain(),
            crate::ResourceError::domain(),
            crate::StreamError::domain(),
            crate::LibraryError::domain(),
        ]
        .contains(&error.domain()));
        ErrorBuilder::new(error)
    }

    #[doc(alias = "get_error")]
    #[doc(alias = "gst_message_parse_error")]
    pub fn error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_error(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    #[doc(alias = "get_debug")]
    #[doc(alias = "gst_message_parse_error")]
    pub fn debug(&self) -> Option<glib::GString> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_error(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[doc(alias = "get_details")]
    #[doc(alias = "gst_message_parse_error_details")]
    pub fn details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_parse_error_writable_details")]
    pub fn writable_details(&mut self) -> &mut StructureRef {
        unsafe {
            let mut details = ptr::null_mut();

            ffi::gst_message_parse_error_writable_details(self.as_mut_ptr(), &mut details);

            StructureRef::from_glib_borrow_mut(details)
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("error", &self.error())
            .field("debug", &self.debug())
            .field("details", &self.details())
            .finish()
    }
}

impl std::fmt::Debug for Error<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Error::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Warning, T);
impl Warning {
    #[doc(alias = "gst_message_new_warning")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> WarningBuilder {
        assert_initialized_main_thread!();
        WarningBuilder::new(glib::Error::new(error, message))
    }

    pub fn builder_from_error<'a>(error: glib::Error) -> WarningBuilder<'a> {
        assert_initialized_main_thread!();

        assert!([
            crate::CoreError::domain(),
            crate::ResourceError::domain(),
            crate::StreamError::domain(),
            crate::LibraryError::domain(),
        ]
        .contains(&error.domain()));
        WarningBuilder::new(error)
    }

    #[doc(alias = "get_error")]
    #[doc(alias = "gst_message_parse_warning")]
    pub fn error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_warning(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    #[doc(alias = "get_debug")]
    #[doc(alias = "gst_message_parse_warning")]
    pub fn debug(&self) -> Option<glib::GString> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_warning(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[doc(alias = "get_details")]
    #[doc(alias = "gst_message_parse_warning_details")]
    pub fn details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_warning_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_parse_warning_writable_details")]
    pub fn writable_details(&mut self) -> &mut StructureRef {
        unsafe {
            let mut details = ptr::null_mut();

            ffi::gst_message_parse_warning_writable_details(self.as_mut_ptr(), &mut details);

            StructureRef::from_glib_borrow_mut(details)
        }
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error())
    }
}

impl std::fmt::Debug for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Warning")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("error", &self.error())
            .field("debug", &self.debug())
            .field("details", &self.details())
            .finish()
    }
}

impl std::fmt::Debug for Warning<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Warning::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Info, T);
impl Info {
    #[doc(alias = "gst_message_new_info")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> InfoBuilder {
        assert_initialized_main_thread!();
        InfoBuilder::new(glib::Error::new(error, message))
    }

    pub fn builder_from_error<'a>(error: glib::Error) -> InfoBuilder<'a> {
        assert_initialized_main_thread!();

        assert!([
            crate::CoreError::domain(),
            crate::ResourceError::domain(),
            crate::StreamError::domain(),
            crate::LibraryError::domain(),
        ]
        .contains(&error.domain()));
        InfoBuilder::new(error)
    }

    #[doc(alias = "get_error")]
    #[doc(alias = "gst_message_parse_info")]
    pub fn error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_info(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    #[doc(alias = "get_debug")]
    #[doc(alias = "gst_message_parse_info")]
    pub fn debug(&self) -> Option<glib::GString> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_info(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[doc(alias = "get_details")]
    #[doc(alias = "gst_message_parse_info_details")]
    pub fn details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_info_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }

    #[cfg(feature = "v1_26")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
    #[doc(alias = "gst_message_parse_info_writable_details")]
    pub fn writable_details(&mut self) -> &mut StructureRef {
        unsafe {
            let mut details = ptr::null_mut();

            ffi::gst_message_parse_info_writable_details(self.as_mut_ptr(), &mut details);

            StructureRef::from_glib_borrow_mut(details)
        }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error())
    }
}

impl std::fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Info")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("error", &self.error())
            .field("debug", &self.debug())
            .field("details", &self.details())
            .finish()
    }
}

impl std::fmt::Debug for Info<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Info::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Tag, T);
impl Tag {
    #[doc(alias = "gst_message_new_tag")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(tags: &TagList) -> Message {
        skip_assert_initialized!();
        Self::builder(tags).build()
    }

    pub fn builder(tags: &TagList) -> TagBuilder {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    #[doc(alias = "get_tags")]
    #[doc(alias = "gst_message_parse_tag")]
    pub fn tags(&self) -> TagList {
        unsafe {
            let mut tags = ptr::null_mut();
            ffi::gst_message_parse_tag(self.as_mut_ptr(), &mut tags);
            from_glib_full(tags)
        }
    }
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tag")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("tags", &self.tags())
            .finish()
    }
}

impl std::fmt::Debug for Tag<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Tag::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Buffering, T);
impl Buffering {
    #[doc(alias = "gst_message_new_buffering")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(percent: i32) -> Message {
        skip_assert_initialized!();
        Self::builder(percent).build()
    }

    pub fn builder<'a>(percent: i32) -> BufferingBuilder<'a> {
        assert_initialized_main_thread!();
        BufferingBuilder::new(percent)
    }

    #[doc(alias = "get_percent")]
    #[doc(alias = "gst_message_parse_buffering")]
    pub fn percent(&self) -> i32 {
        unsafe {
            let mut p = mem::MaybeUninit::uninit();
            ffi::gst_message_parse_buffering(self.as_mut_ptr(), p.as_mut_ptr());
            p.assume_init()
        }
    }

    #[doc(alias = "get_buffering_stats")]
    #[doc(alias = "gst_message_parse_buffering_stats")]
    pub fn buffering_stats(&self) -> (crate::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut avg_in = mem::MaybeUninit::uninit();
            let mut avg_out = mem::MaybeUninit::uninit();
            let mut buffering_left = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_buffering_stats(
                self.as_mut_ptr(),
                mode.as_mut_ptr(),
                avg_in.as_mut_ptr(),
                avg_out.as_mut_ptr(),
                buffering_left.as_mut_ptr(),
            );

            (
                from_glib(mode.assume_init()),
                avg_in.assume_init(),
                avg_out.assume_init(),
                buffering_left.assume_init(),
            )
        }
    }
}

impl std::fmt::Debug for Buffering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buffering")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("percent", &self.percent())
            .field("buffering-stats", &self.buffering_stats())
            .finish()
    }
}

impl std::fmt::Debug for Buffering<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Buffering::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StateChanged, T);
impl StateChanged {
    #[doc(alias = "gst_message_new_state_changed")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(old: crate::State, new: crate::State, pending: crate::State) -> Message {
        skip_assert_initialized!();
        Self::builder(old, new, pending).build()
    }

    pub fn builder<'a>(
        old: crate::State,
        new: crate::State,
        pending: crate::State,
    ) -> StateChangedBuilder<'a> {
        assert_initialized_main_thread!();
        StateChangedBuilder::new(old, new, pending)
    }

    #[doc(alias = "get_old")]
    #[doc(alias = "gst_message_parse_state_changed")]
    pub fn old(&self) -> crate::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                state.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(state.assume_init())
        }
    }

    #[doc(alias = "get_current")]
    #[doc(alias = "gst_message_parse_state_changed")]
    pub fn current(&self) -> crate::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                ptr::null_mut(),
                state.as_mut_ptr(),
                ptr::null_mut(),
            );

            from_glib(state.assume_init())
        }
    }

    #[doc(alias = "get_pending")]
    #[doc(alias = "gst_message_parse_state_changed")]
    pub fn pending(&self) -> crate::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                state.as_mut_ptr(),
            );

            from_glib(state.assume_init())
        }
    }
}

impl std::fmt::Debug for StateChanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateChanged")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("old", &self.old())
            .field("current", &self.current())
            .field("pending", &self.pending())
            .finish()
    }
}

impl std::fmt::Debug for StateChanged<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StateChanged::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StateDirty, T);
impl StateDirty {
    #[doc(alias = "gst_message_new_state_dirty")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> StateDirtyBuilder<'a> {
        assert_initialized_main_thread!();
        StateDirtyBuilder::new()
    }
}

impl std::fmt::Debug for StateDirty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StateDirty")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for StateDirty<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StateDirty::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StepDone, T);
impl StepDone {
    #[doc(alias = "gst_message_new_step_done")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        amount: impl FormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: impl Into<Option<crate::ClockTime>>,
        eos: bool,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(amount, rate, flush, intermediate, duration, eos).build()
    }

    pub fn builder<'a>(
        amount: impl FormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: impl Into<Option<crate::ClockTime>>,
        eos: bool,
    ) -> StepDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StepDoneBuilder::new(
            amount.into(),
            rate,
            flush,
            intermediate,
            duration.into(),
            eos,
        )
    }

    #[doc(alias = "gst_message_parse_step_done")]
    pub fn get(
        &self,
    ) -> (
        GenericFormattedValue,
        f64,
        bool,
        bool,
        Option<crate::ClockTime>,
        bool,
    ) {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();
            let mut eos = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_step_done(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
                duration.as_mut_ptr(),
                eos.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
                from_glib(duration.assume_init()),
                from_glib(eos.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for StepDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (format, rate, flush, intermediate, duration, eos) = self.get();
        f.debug_struct("StepDone")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("format", &format)
            .field("rate", &rate)
            .field("flush", &flush)
            .field("intermediate", &intermediate)
            .field("duration", &duration)
            .field("eos", &eos)
            .finish()
    }
}

impl std::fmt::Debug for StepDone<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StepDone::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(ClockProvide, T);
impl ClockProvide {
    #[doc(alias = "gst_message_new_clock_provide")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &crate::Clock, ready: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(clock, ready).build()
    }

    pub fn builder(clock: &crate::Clock, ready: bool) -> ClockProvideBuilder {
        assert_initialized_main_thread!();
        ClockProvideBuilder::new(clock, ready)
    }

    #[doc(alias = "get_clock")]
    #[doc(alias = "gst_message_parse_clock_provide")]
    pub fn clock(&self) -> Option<crate::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_provide(self.as_mut_ptr(), &mut clock, ptr::null_mut());

            from_glib_none(clock)
        }
    }

    #[doc(alias = "get_ready")]
    #[doc(alias = "gst_message_parse_clock_provide")]
    pub fn is_ready(&self) -> bool {
        unsafe {
            let mut ready = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_clock_provide(
                self.as_mut_ptr(),
                ptr::null_mut(),
                ready.as_mut_ptr(),
            );

            from_glib(ready.assume_init())
        }
    }
}

impl std::fmt::Debug for ClockProvide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClockProvide")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("clock", &self.clock())
            .field("is-ready", &self.is_ready())
            .finish()
    }
}

impl std::fmt::Debug for ClockProvide<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ClockProvide::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(ClockLost, T);
impl ClockLost {
    #[doc(alias = "gst_message_new_clock_lost")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &crate::Clock) -> Message {
        skip_assert_initialized!();
        Self::builder(clock).build()
    }

    pub fn builder(clock: &crate::Clock) -> ClockLostBuilder {
        assert_initialized_main_thread!();
        ClockLostBuilder::new(clock)
    }

    #[doc(alias = "get_clock")]
    #[doc(alias = "gst_message_parse_clock_lost")]
    pub fn clock(&self) -> Option<crate::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_lost(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

impl std::fmt::Debug for ClockLost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClockLost")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("clock", &self.clock())
            .finish()
    }
}

impl std::fmt::Debug for ClockLost<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ClockLost::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(NewClock, T);
impl NewClock {
    #[doc(alias = "gst_message_new_new_clock")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &crate::Clock) -> Message {
        skip_assert_initialized!();
        Self::builder(clock).build()
    }

    pub fn builder(clock: &crate::Clock) -> NewClockBuilder {
        assert_initialized_main_thread!();
        NewClockBuilder::new(clock)
    }

    #[doc(alias = "get_clock")]
    #[doc(alias = "gst_message_parse_new_clock")]
    pub fn clock(&self) -> Option<crate::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_new_clock(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

impl std::fmt::Debug for NewClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewClock")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("clock", &self.clock())
            .finish()
    }
}

impl std::fmt::Debug for NewClock<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        NewClock::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StructureChange, T);
impl StructureChange {
    #[doc(alias = "gst_message_new_structure_change")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: crate::StructureChangeType, owner: &crate::Element, busy: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, owner, busy).build()
    }

    pub fn builder(
        type_: crate::StructureChangeType,
        owner: &crate::Element,
        busy: bool,
    ) -> StructureChangeBuilder {
        assert_initialized_main_thread!();
        StructureChangeBuilder::new(type_, owner, busy)
    }

    #[doc(alias = "gst_message_parse_structure_change")]
    pub fn get(&self) -> (crate::StructureChangeType, crate::Element, bool) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut owner = ptr::null_mut();
            let mut busy = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_structure_change(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                &mut owner,
                busy.as_mut_ptr(),
            );

            (
                from_glib(type_.assume_init()),
                from_glib_none(owner),
                from_glib(busy.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for StructureChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (type_, owner, busy) = self.get();

        f.debug_struct("StructureChange")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("type", &type_)
            .field("owner", &owner)
            .field("busy", &busy)
            .finish()
    }
}

impl std::fmt::Debug for StructureChange<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StructureChange::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StreamStatus, T);
impl StreamStatus {
    #[doc(alias = "gst_message_new_stream_status")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: crate::StreamStatusType, owner: &crate::Element) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, owner).build()
    }

    pub fn builder(type_: crate::StreamStatusType, owner: &crate::Element) -> StreamStatusBuilder {
        assert_initialized_main_thread!();
        StreamStatusBuilder::new(type_, owner)
    }

    #[doc(alias = "gst_message_parse_stream_status")]
    pub fn get(&self) -> (crate::StreamStatusType, crate::Element) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut owner = ptr::null_mut();

            ffi::gst_message_parse_stream_status(self.as_mut_ptr(), type_.as_mut_ptr(), &mut owner);

            (from_glib(type_.assume_init()), from_glib_none(owner))
        }
    }

    #[doc(alias = "get_stream_status_object")]
    #[doc(alias = "gst_message_get_stream_status_object")]
    pub fn stream_status_object(&self) -> Option<glib::Value> {
        unsafe {
            let value = ffi::gst_message_get_stream_status_object(self.as_mut_ptr());

            from_glib_none(value)
        }
    }
}

impl std::fmt::Debug for StreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamStatus")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("status", &self.stream_status_object())
            .finish()
    }
}

impl std::fmt::Debug for StreamStatus<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamStatus::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Application, T);
impl Application {
    #[doc(alias = "gst_message_new_application")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Message {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> ApplicationBuilder<'a> {
        assert_initialized_main_thread!();
        ApplicationBuilder::new(structure)
    }
}

impl std::fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Application")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for Application<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Application::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Element, T);
impl Element {
    #[doc(alias = "gst_message_new_element")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: crate::Structure) -> Message {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder<'a>(structure: crate::Structure) -> ElementBuilder<'a> {
        assert_initialized_main_thread!();
        ElementBuilder::new(structure)
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Element")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for Element<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Element::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(SegmentStart, T);
impl SegmentStart {
    #[doc(alias = "gst_message_new_segment_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(position: impl FormattedValue) -> Message {
        skip_assert_initialized!();
        Self::builder(position).build()
    }

    pub fn builder<'a>(position: impl FormattedValue) -> SegmentStartBuilder<'a> {
        assert_initialized_main_thread!();
        SegmentStartBuilder::new(position.into())
    }

    #[doc(alias = "gst_message_parse_segment_start")]
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_segment_start(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(format.assume_init()), position.assume_init())
        }
    }
}

impl std::fmt::Debug for SegmentStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SegmentStart")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("segment", &self.get())
            .finish()
    }
}

impl std::fmt::Debug for SegmentStart<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SegmentStart::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(SegmentDone, T);
impl SegmentDone {
    #[doc(alias = "gst_message_new_segment_done")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(position: impl FormattedValue) -> Message {
        skip_assert_initialized!();
        Self::builder(position).build()
    }

    pub fn builder<'a>(position: impl FormattedValue) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        SegmentDoneBuilder::new(position.into())
    }

    #[doc(alias = "gst_message_parse_segment_done")]
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_segment_done(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(format.assume_init()), position.assume_init())
        }
    }
}

impl std::fmt::Debug for SegmentDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SegmentDone")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("segment", &self.get())
            .finish()
    }
}

impl std::fmt::Debug for SegmentDone<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SegmentDone::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(DurationChanged, T);
impl DurationChanged {
    #[doc(alias = "gst_message_new_duration_changed")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> DurationChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DurationChangedBuilder::new()
    }
}

impl std::fmt::Debug for DurationChanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DurationChanged")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for DurationChanged<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DurationChanged::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Latency, T);
impl Latency {
    #[doc(alias = "gst_message_new_latency")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new()
    }
}

impl std::fmt::Debug for Latency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latency")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for Latency<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Latency::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(AsyncStart, T);
impl AsyncStart {
    #[doc(alias = "gst_message_new_async_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> AsyncStartBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncStartBuilder::new()
    }
}

impl std::fmt::Debug for AsyncStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncStart")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .finish()
    }
}

impl std::fmt::Debug for AsyncStart<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AsyncStart::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(AsyncDone, T);
impl AsyncDone {
    #[doc(alias = "gst_message_new_async_done")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(running_time: impl Into<Option<crate::ClockTime>>) -> Message {
        skip_assert_initialized!();
        Self::builder().running_time(running_time).build()
    }

    pub fn builder<'a>() -> AsyncDoneBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncDoneBuilder::new()
    }

    #[doc(alias = "get_running_time")]
    #[doc(alias = "gst_message_parse_async_done")]
    pub fn running_time(&self) -> Option<crate::ClockTime> {
        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_async_done(self.as_mut_ptr(), running_time.as_mut_ptr());

            from_glib(running_time.assume_init())
        }
    }
}

impl std::fmt::Debug for AsyncDone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncDone")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("running-time", &self.running_time())
            .finish()
    }
}

impl std::fmt::Debug for AsyncDone<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AsyncDone::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(RequestState, T);
impl RequestState {
    #[doc(alias = "gst_message_new_request_state")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(state: crate::State) -> Message {
        skip_assert_initialized!();
        Self::builder(state).build()
    }

    pub fn builder<'a>(state: crate::State) -> RequestStateBuilder<'a> {
        assert_initialized_main_thread!();
        RequestStateBuilder::new(state)
    }

    #[doc(alias = "get_requested_state")]
    #[doc(alias = "gst_message_parse_request_state")]
    pub fn requested_state(&self) -> crate::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_request_state(self.as_mut_ptr(), state.as_mut_ptr());

            from_glib(state.assume_init())
        }
    }
}

impl std::fmt::Debug for RequestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestState")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("requested-state", &self.requested_state())
            .finish()
    }
}

impl std::fmt::Debug for RequestState<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        RequestState::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StepStart, T);
impl StepStart {
    #[doc(alias = "gst_message_new_step_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        active: bool,
        amount: impl FormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(active, amount, rate, flush, intermediate).build()
    }

    pub fn builder<'a>(
        active: bool,
        amount: impl FormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepStartBuilder<'a> {
        assert_initialized_main_thread!();
        StepStartBuilder::new(active, amount.into(), rate, flush, intermediate)
    }

    #[doc(alias = "gst_message_parse_step_start")]
    pub fn get(&self) -> (bool, GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut active = mem::MaybeUninit::uninit();
            let mut format = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_step_start(
                self.as_mut_ptr(),
                active.as_mut_ptr(),
                format.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
            );

            (
                from_glib(active.assume_init()),
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
            )
        }
    }
}

impl std::fmt::Debug for StepStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (active, amount, rate, flush, intermediate) = self.get();
        f.debug_struct("StepStart")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("active", &active)
            .field("amount", &amount)
            .field("rate", &rate)
            .field("flush", &flush)
            .field("intermediate", &intermediate)
            .finish()
    }
}

impl std::fmt::Debug for StepStart<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StepStart::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Qos, T);
impl Qos {
    #[doc(alias = "gst_message_new_qos")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        live: bool,
        running_time: impl Into<Option<crate::ClockTime>>,
        stream_time: impl Into<Option<crate::ClockTime>>,
        timestamp: impl Into<Option<crate::ClockTime>>,
        duration: impl Into<Option<crate::ClockTime>>,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(live)
            .running_time(running_time)
            .stream_time(stream_time)
            .timestamp(timestamp)
            .duration(duration)
            .build()
    }

    pub fn builder<'a>(live: bool) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(live)
    }

    #[doc(alias = "gst_message_parse_qos")]
    pub fn get(
        &self,
    ) -> (
        bool,
        Option<crate::ClockTime>,
        Option<crate::ClockTime>,
        Option<crate::ClockTime>,
        Option<crate::ClockTime>,
    ) {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut running_time = mem::MaybeUninit::uninit();
            let mut stream_time = mem::MaybeUninit::uninit();
            let mut timestamp = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_qos(
                self.as_mut_ptr(),
                live.as_mut_ptr(),
                running_time.as_mut_ptr(),
                stream_time.as_mut_ptr(),
                timestamp.as_mut_ptr(),
                duration.as_mut_ptr(),
            );

            (
                from_glib(live.assume_init()),
                from_glib(running_time.assume_init()),
                from_glib(stream_time.assume_init()),
                from_glib(timestamp.assume_init()),
                from_glib(duration.assume_init()),
            )
        }
    }

    #[doc(alias = "get_values")]
    #[doc(alias = "gst_message_parse_qos_values")]
    pub fn values(&self) -> (i64, f64, i32) {
        unsafe {
            let mut jitter = mem::MaybeUninit::uninit();
            let mut proportion = mem::MaybeUninit::uninit();
            let mut quality = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_qos_values(
                self.as_mut_ptr(),
                jitter.as_mut_ptr(),
                proportion.as_mut_ptr(),
                quality.as_mut_ptr(),
            );

            (
                jitter.assume_init(),
                proportion.assume_init(),
                quality.assume_init(),
            )
        }
    }

    #[doc(alias = "get_stats")]
    #[doc(alias = "gst_message_parse_qos_stats")]
    pub fn stats(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut processed = mem::MaybeUninit::uninit();
            let mut dropped = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_qos_stats(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                processed.as_mut_ptr(),
                dropped.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    processed.assume_init() as i64,
                ),
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    dropped.assume_init() as i64,
                ),
            )
        }
    }
}

impl std::fmt::Debug for Qos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (live, running_time, stream_time, timestamp, duration) = self.get();
        let (jitter, proportion, quality) = self.values();
        let (processed, dropped) = self.stats();

        f.debug_struct("Qos")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("live", &live)
            .field("running-time", &running_time)
            .field("stream-time", &stream_time)
            .field("timestamp", &timestamp)
            .field("duration", &duration)
            .field("jitter", &jitter)
            .field("proportion", &proportion)
            .field("quality", &quality)
            .field("processed", &processed)
            .field("dropped", &dropped)
            .finish()
    }
}

impl std::fmt::Debug for Qos<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Qos::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Progress, T);
impl Progress {
    #[doc(alias = "gst_message_new_progress")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: crate::ProgressType, code: &str, text: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, code, text).build()
    }

    pub fn builder<'a>(
        type_: crate::ProgressType,
        code: &'a str,
        text: &'a str,
    ) -> ProgressBuilder<'a> {
        assert_initialized_main_thread!();
        ProgressBuilder::new(type_, code, text)
    }

    #[doc(alias = "gst_message_parse_progress")]
    pub fn get(&self) -> (crate::ProgressType, &str, &str) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut code = ptr::null_mut();
            let mut text = ptr::null_mut();

            ffi::gst_message_parse_progress(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                &mut code,
                &mut text,
            );

            let code = CStr::from_ptr(code).to_str().unwrap();
            let text = CStr::from_ptr(text).to_str().unwrap();

            (from_glib(type_.assume_init()), code, text)
        }
    }
}

impl std::fmt::Debug for Progress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (type_, code, text) = self.get();
        f.debug_struct("Progress")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("type", &type_)
            .field("code", &code)
            .field("text", &text)
            .finish()
    }
}

impl std::fmt::Debug for Progress<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Progress::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Toc, T);
impl Toc {
    // FIXME could use false for updated as default
    // Even better: use an enum for updated so that it is more explicit than true / false
    #[doc(alias = "gst_message_new_toc")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(toc: &crate::Toc, updated: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(toc, updated).build()
    }

    pub fn builder(toc: &crate::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    #[doc(alias = "get_toc")]
    #[doc(alias = "gst_message_parse_toc")]
    pub fn toc(&self) -> (crate::Toc, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::MaybeUninit::uninit();
            ffi::gst_message_parse_toc(self.as_mut_ptr(), &mut toc, updated.as_mut_ptr());
            (from_glib_full(toc), from_glib(updated.assume_init()))
        }
    }
}

impl std::fmt::Debug for Toc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Toc")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("toc", &self.toc())
            .finish()
    }
}

impl std::fmt::Debug for Toc<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Toc::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(ResetTime, T);
impl ResetTime {
    #[doc(alias = "gst_message_new_reset_time")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(running_time: crate::ClockTime) -> Message {
        skip_assert_initialized!();
        Self::builder(running_time).build()
    }

    pub fn builder<'a>(running_time: crate::ClockTime) -> ResetTimeBuilder<'a> {
        assert_initialized_main_thread!();
        ResetTimeBuilder::new(running_time)
    }

    #[doc(alias = "get_running_time")]
    #[doc(alias = "gst_message_parse_reset_time")]
    pub fn running_time(&self) -> crate::ClockTime {
        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_reset_time(self.as_mut_ptr(), running_time.as_mut_ptr());

            try_from_glib(running_time.assume_init()).expect("undefined running_time")
        }
    }
}

impl std::fmt::Debug for ResetTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResetTime")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("running-time", &self.running_time())
            .finish()
    }
}

impl std::fmt::Debug for ResetTime<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ResetTime::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StreamStart, T);
impl StreamStart {
    #[doc(alias = "gst_message_new_stream_start")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder<'a>() -> StreamStartBuilder<'a> {
        assert_initialized_main_thread!();
        StreamStartBuilder::new()
    }

    #[doc(alias = "get_group_id")]
    #[doc(alias = "gst_message_parse_group_id")]
    pub fn group_id(&self) -> Option<GroupId> {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            if from_glib(ffi::gst_message_parse_group_id(
                self.as_mut_ptr(),
                group_id.as_mut_ptr(),
            )) {
                let group_id = group_id.assume_init();
                if group_id == 0 {
                    None
                } else {
                    Some(GroupId(NonZeroU32::new_unchecked(group_id)))
                }
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_message_set_group_id")]
    pub fn set_group_id(&mut self, group_id: GroupId) {
        unsafe {
            ffi::gst_message_set_group_id(self.as_mut_ptr(), group_id.0.get());
        }
    }
}

impl std::fmt::Debug for StreamStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamStart")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("group-id", &self.group_id())
            .finish()
    }
}

impl std::fmt::Debug for StreamStart<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamStart::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(NeedContext, T);
impl NeedContext {
    #[doc(alias = "gst_message_new_need_context")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(context_type: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(context_type).build()
    }

    pub fn builder(context_type: &str) -> NeedContextBuilder {
        assert_initialized_main_thread!();
        NeedContextBuilder::new(context_type)
    }

    #[doc(alias = "get_context_type")]
    #[doc(alias = "gst_message_parse_context_type")]
    pub fn context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();

            ffi::gst_message_parse_context_type(self.as_mut_ptr(), &mut context_type);

            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

impl std::fmt::Debug for NeedContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NeedContext")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("context-type", &self.context_type())
            .finish()
    }
}

impl std::fmt::Debug for NeedContext<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        NeedContext::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(HaveContext, T);
impl HaveContext {
    #[doc(alias = "gst_message_new_have_context")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(context: crate::Context) -> Message {
        skip_assert_initialized!();
        Self::builder(context).build()
    }

    pub fn builder<'a>(context: crate::Context) -> HaveContextBuilder<'a> {
        assert_initialized_main_thread!();
        HaveContextBuilder::new(context)
    }

    #[doc(alias = "get_context")]
    #[doc(alias = "gst_message_parse_have_context")]
    pub fn context(&self) -> crate::Context {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_message_parse_have_context(self.as_mut_ptr(), &mut context);
            from_glib_full(context)
        }
    }
}

impl std::fmt::Debug for HaveContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HaveContext")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("context", &self.context())
            .finish()
    }
}

impl std::fmt::Debug for HaveContext<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        HaveContext::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(DeviceAdded, T);
impl DeviceAdded {
    #[doc(alias = "gst_message_new_device_added")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &crate::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device).build()
    }

    pub fn builder(device: &crate::Device) -> DeviceAddedBuilder {
        assert_initialized_main_thread!();
        DeviceAddedBuilder::new(device)
    }

    #[doc(alias = "get_device")]
    #[doc(alias = "gst_message_parse_device_added")]
    pub fn device(&self) -> crate::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_added(self.as_mut_ptr(), &mut device);

            from_glib_full(device)
        }
    }
}

impl std::fmt::Debug for DeviceAdded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceAdded")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("device", &self.device())
            .finish()
    }
}

impl std::fmt::Debug for DeviceAdded<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DeviceAdded::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(DeviceRemoved, T);
impl DeviceRemoved {
    #[doc(alias = "gst_message_new_device_removed")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &crate::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device).build()
    }

    pub fn builder(device: &crate::Device) -> DeviceRemovedBuilder {
        assert_initialized_main_thread!();
        DeviceRemovedBuilder::new(device)
    }

    #[doc(alias = "get_device")]
    #[doc(alias = "gst_message_parse_device_removed")]
    pub fn device(&self) -> crate::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_removed(self.as_mut_ptr(), &mut device);

            from_glib_full(device)
        }
    }
}

impl std::fmt::Debug for DeviceRemoved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceRemoved")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("device", &self.device())
            .finish()
    }
}

impl std::fmt::Debug for DeviceRemoved<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DeviceRemoved::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(PropertyNotify, T);
impl PropertyNotify {
    #[doc(alias = "gst_message_new_property_notify")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(object: &impl IsA<crate::Object>, property_name: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(object, property_name).build()
    }

    pub fn builder<'a>(
        object: &'a impl IsA<crate::Object>,
        property_name: &'a str,
    ) -> PropertyNotifyBuilder<'a> {
        assert_initialized_main_thread!();
        PropertyNotifyBuilder::new(property_name).src(object)
    }

    #[doc(alias = "gst_message_parse_property_notify")]
    pub fn get(&self) -> (Object, &str, Option<&glib::Value>) {
        unsafe {
            let mut object = ptr::null_mut();
            let mut property_name = ptr::null();
            let mut value = ptr::null();

            ffi::gst_message_parse_property_notify(
                self.as_mut_ptr(),
                &mut object,
                &mut property_name,
                &mut value,
            );

            (
                from_glib_none(object),
                CStr::from_ptr(property_name).to_str().unwrap(),
                if value.is_null() {
                    None
                } else {
                    Some(&*(value as *const glib::Value))
                },
            )
        }
    }
}

impl std::fmt::Debug for PropertyNotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (object, property_name, value) = self.get();
        f.debug_struct("PropertyNotify")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("object", &object)
            .field("property-name", &property_name)
            .field("value", &value)
            .finish()
    }
}

impl std::fmt::Debug for PropertyNotify<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PropertyNotify::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StreamCollection, T);
impl StreamCollection {
    #[doc(alias = "gst_message_new_stream_collection")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(collection: &crate::StreamCollection) -> Message {
        skip_assert_initialized!();
        Self::builder(collection).build()
    }

    pub fn builder(collection: &crate::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(collection)
    }

    #[doc(alias = "get_stream_collection")]
    #[doc(alias = "gst_message_parse_stream_collection")]
    pub fn stream_collection(&self) -> crate::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_stream_collection(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }
}

impl std::fmt::Debug for StreamCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamCollection")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("stream-collection", &self.stream_collection())
            .finish()
    }
}

impl std::fmt::Debug for StreamCollection<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamCollection::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(StreamsSelected, T);
impl StreamsSelected {
    #[doc(alias = "gst_message_new_streams_selected")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(collection: &crate::StreamCollection) -> Message {
        skip_assert_initialized!();
        Self::builder(collection).build()
    }

    pub fn builder(collection: &crate::StreamCollection) -> StreamsSelectedBuilder {
        assert_initialized_main_thread!();
        StreamsSelectedBuilder::new(collection)
    }

    #[doc(alias = "get_stream_collection")]
    #[doc(alias = "gst_message_parse_streams_selected")]
    pub fn stream_collection(&self) -> crate::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_streams_selected(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }

    #[doc(alias = "get_streams")]
    #[doc(alias = "gst_message_streams_selected_get_size")]
    #[doc(alias = "gst_message_streams_selected_get_stream")]
    pub fn streams(&self) -> Vec<crate::Stream> {
        unsafe {
            let n = ffi::gst_message_streams_selected_get_size(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    from_glib_full(ffi::gst_message_streams_selected_get_stream(
                        self.as_mut_ptr(),
                        i,
                    ))
                })
                .collect()
        }
    }
}

impl std::fmt::Debug for StreamsSelected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamsSelected")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("stream-collection", &self.stream_collection())
            .field("streams", &self.streams())
            .finish()
    }
}

impl std::fmt::Debug for StreamsSelected<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        StreamsSelected::<MessageRef>::fmt(self, f)
    }
}

declare_concrete_message!(Redirect, T);
impl Redirect {
    #[doc(alias = "gst_message_new_redirect")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(location: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(location).build()
    }

    pub fn builder(location: &str) -> RedirectBuilder {
        assert_initialized_main_thread!();
        RedirectBuilder::new(location)
    }

    #[doc(alias = "get_entries")]
    #[doc(alias = "gst_message_get_num_redirect_entries")]
    #[doc(alias = "gst_message_parse_redirect_entry")]
    pub fn entries(&self) -> Vec<(&str, Option<TagList>, Option<&StructureRef>)> {
        unsafe {
            let n = ffi::gst_message_get_num_redirect_entries(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    let mut location = ptr::null();
                    let mut tags = ptr::null_mut();
                    let mut structure = ptr::null();

                    ffi::gst_message_parse_redirect_entry(
                        self.as_mut_ptr(),
                        i,
                        &mut location,
                        &mut tags,
                        &mut structure,
                    );

                    let structure = if structure.is_null() {
                        None
                    } else {
                        Some(StructureRef::from_glib_borrow(structure))
                    };

                    (
                        CStr::from_ptr(location).to_str().unwrap(),
                        from_glib_none(tags),
                        structure,
                    )
                })
                .collect()
        }
    }
}

impl std::fmt::Debug for Redirect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Redirect")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("entries", &self.entries())
            .finish()
    }
}

impl std::fmt::Debug for Redirect<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Redirect::<MessageRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
declare_concrete_message!(DeviceChanged, T);
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl DeviceChanged {
    #[doc(alias = "gst_message_new_device_changed")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &crate::Device, changed_device: &crate::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device, changed_device).build()
    }

    pub fn builder<'a>(
        device: &'a crate::Device,
        changed_device: &'a crate::Device,
    ) -> DeviceChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DeviceChangedBuilder::new(device, changed_device)
    }

    #[doc(alias = "get_device_changed")]
    #[doc(alias = "gst_message_parse_device_changed")]
    pub fn device_changed(&self) -> (crate::Device, crate::Device) {
        unsafe {
            let mut device = ptr::null_mut();
            let mut changed_device = ptr::null_mut();

            ffi::gst_message_parse_device_changed(
                self.as_mut_ptr(),
                &mut device,
                &mut changed_device,
            );

            (from_glib_full(device), from_glib_full(changed_device))
        }
    }
}

#[cfg(feature = "v1_16")]
impl std::fmt::Debug for DeviceChanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceChanged")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("device-changed", &self.device_changed())
            .finish()
    }
}

#[cfg(feature = "v1_16")]
impl std::fmt::Debug for DeviceChanged<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DeviceChanged::<MessageRef>::fmt(self, f)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
declare_concrete_message!(InstantRateRequest, T);
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl InstantRateRequest {
    #[doc(alias = "gst_message_new_instant_rate_request")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(rate_multiplier: f64) -> Message {
        skip_assert_initialized!();
        Self::builder(rate_multiplier).build()
    }

    pub fn builder<'a>(rate_multiplier: f64) -> InstantRateRequestBuilder<'a> {
        assert_initialized_main_thread!();
        InstantRateRequestBuilder::new(rate_multiplier)
    }

    #[doc(alias = "parse_instant_rate_request")]
    #[doc(alias = "gst_message_parse_instant_rate_request")]
    pub fn rate_multiplier(&self) -> f64 {
        unsafe {
            let mut rate_multiplier = mem::MaybeUninit::uninit();

            ffi::gst_message_parse_instant_rate_request(
                self.as_mut_ptr(),
                rate_multiplier.as_mut_ptr(),
            );

            rate_multiplier.assume_init()
        }
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InstantRateRequest")
            .field("structure", &self.message().structure())
            .field("source", &self.src().map(|obj| (obj, obj.name())))
            .field("rate-multiplier", &self.rate_multiplier())
            .finish()
    }
}

#[cfg(feature = "v1_18")]
impl std::fmt::Debug for InstantRateRequest<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        InstantRateRequest::<MessageRef>::fmt(self, f)
    }
}

struct MessageBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    #[cfg(feature = "v1_26")]
    details: Option<Structure>,
    other_fields: Vec<(&'a str, glib::SendValue)>,
}

impl<'a> MessageBuilder<'a> {
    fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
            #[cfg(feature = "v1_26")]
            details: None,
            other_fields: Vec::new(),
        }
    }

    pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
        Self {
            src: Some(src.clone().upcast::<Object>()),
            ..self
        }
    }

    fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    #[cfg(feature = "v1_26")]
    fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
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

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        #[allow(clippy::needless_update)]
        pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
            Self {
                builder: self.builder.src(src),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn src_if<O: IsA<Object> + Cast + Clone>(self, src: &O, predicate: bool) -> Self {
            if predicate {
                self.src(src)
            } else {
                self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn src_if_some<O: IsA<Object> + Cast + Clone>(self, src: Option<&O>) -> Self {
            if let Some(src) = src {
                self.src(src)
            } else {
                self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if(self, seqnum: Seqnum, predicate: bool) -> Self {
            if predicate {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[doc(alias = "gst_message_set_seqnum")]
        #[allow(clippy::needless_update)]
        pub fn seqnum_if_some(self, seqnum: Option<Seqnum>) -> Self {
            if let Some(seqnum) = seqnum {
                self.seqnum(seqnum)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        #[allow(clippy::needless_update)]
        pub fn details(self, details: Structure) -> Self {
            Self {
                builder: self.builder.details(details),
                ..self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        #[allow(clippy::needless_update)]
        pub fn details_if(self, details: Structure, predicate: bool) -> Self {
            if predicate {
                self.details(details)
            } else {
                self
            }
        }

        #[cfg(feature = "v1_26")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
        #[doc(alias = "gst_message_set_details")]
        #[allow(clippy::needless_update)]
        pub fn details_if_some(self, details: Option<Structure>) -> Self {
            if let Some(details) = details {
                self.details(details)
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

        #[must_use = "Building the message without using it has no effect"]
        #[allow(clippy::redundant_closure_call)]
        pub fn build(mut self) -> Message {
            unsafe {
                let src = self.builder.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.builder.seqnum {
                    ffi::gst_message_set_seqnum(msg, seqnum.0.get());
                }

                #[cfg(feature = "v1_26")]
                if let Some(details) = self.builder.details {
                    ffi::gst_message_set_details(msg, details.into_glib_ptr());
                }

                if !self.builder.other_fields.is_empty() {
                    let structure = ffi::gst_message_writable_structure(msg);

                    if !structure.is_null() {
                        let structure = StructureRef::from_glib_borrow_mut(structure as *mut _);

                        for (k, v) in self.builder.other_fields {
                            structure.set_value(k, v);
                        }
                    }
                }

                from_glib_full(msg)
            }
        }
    };
}

#[must_use = "The builder must be built to be used"]
pub struct EosBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_eos(src));
}

pub trait MessageErrorDomain: glib::error::ErrorDomain {}

impl MessageErrorDomain for crate::CoreError {}
impl MessageErrorDomain for crate::ResourceError {}
impl MessageErrorDomain for crate::StreamError {}
impl MessageErrorDomain for crate::LibraryError {}

#[must_use = "The builder must be built to be used"]
pub struct ErrorBuilder<'a> {
    builder: MessageBuilder<'a>,
    error: glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a> ErrorBuilder<'a> {
    fn new(error: glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    pub fn debug_if(self, debug: &'a str, predicate: bool) -> Self {
        if predicate {
            self.debug(debug)
        } else {
            self
        }
    }

    pub fn debug_if_some(self, debug: Option<&'a str>) -> Self {
        if let Some(debug) = debug {
            self.debug(debug)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if(self, details: Structure, predicate: bool) -> Self {
        if predicate {
            self.details(details)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if_some(self, details: Option<Structure>) -> Self {
        if let Some(details) = details {
            self.details(details)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let details = match s.details.take() {
            None => ptr::null_mut(),
            Some(details) => details.into_glib_ptr(),
        };

        ffi::gst_message_new_error_with_details(
            src,
            mut_override(s.error.to_glib_none().0),
            s.debug.to_glib_none().0,
            details,
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct WarningBuilder<'a> {
    builder: MessageBuilder<'a>,
    error: glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a> WarningBuilder<'a> {
    fn new(error: glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    pub fn debug_if(self, debug: &'a str, predicate: bool) -> Self {
        if predicate {
            self.debug(debug)
        } else {
            self
        }
    }

    pub fn debug_if_some(self, debug: Option<&'a str>) -> Self {
        if let Some(debug) = debug {
            self.debug(debug)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if(self, details: Structure, predicate: bool) -> Self {
        if predicate {
            self.details(details)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if_some(self, details: Option<Structure>) -> Self {
        if let Some(details) = details {
            self.details(details)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let details = match s.details.take() {
            None => ptr::null_mut(),
            Some(details) => details.into_glib_ptr(),
        };

        ffi::gst_message_new_warning_with_details(
            src,
            mut_override(s.error.to_glib_none().0),
            s.debug.to_glib_none().0,
            details,
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct InfoBuilder<'a> {
    builder: MessageBuilder<'a>,
    error: glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a> InfoBuilder<'a> {
    fn new(error: glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    pub fn debug_if(self, debug: &'a str, predicate: bool) -> Self {
        if predicate {
            self.debug(debug)
        } else {
            self
        }
    }

    pub fn debug_if_some(self, debug: Option<&'a str>) -> Self {
        if let Some(debug) = debug {
            self.debug(debug)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if(self, details: Structure, predicate: bool) -> Self {
        if predicate {
            self.details(details)
        } else {
            self
        }
    }

    #[cfg(not(feature = "v1_26"))]
    pub fn details_if_some(self, details: Option<Structure>) -> Self {
        if let Some(details) = details {
            self.details(details)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let details = match s.details.take() {
            None => ptr::null_mut(),
            Some(details) => details.into_glib_ptr(),
        };

        ffi::gst_message_new_info_with_details(
            src,
            mut_override(s.error.to_glib_none().0),
            s.debug.to_glib_none().0,
            details,
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct TagBuilder<'a> {
    builder: MessageBuilder<'a>,
    tags: &'a TagList,
}

impl<'a> TagBuilder<'a> {
    fn new(tags: &'a TagList) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            tags,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_message_new_tag(
        src,
        s.tags.to_glib_full()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct BufferingBuilder<'a> {
    builder: MessageBuilder<'a>,
    percent: i32,
    stats: Option<(crate::BufferingMode, i32, i32, i64)>,
}

impl<'a> BufferingBuilder<'a> {
    fn new(percent: i32) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            percent,
            stats: None,
        }
    }

    pub fn stats(
        self,
        mode: crate::BufferingMode,
        avg_in: i32,
        avg_out: i32,
        buffering_left: i64,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            stats: Some((mode, avg_in, avg_out, buffering_left)),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_buffering(src, s.percent);

        if let Some((mode, avg_in, avg_out, buffering_left)) = s.stats {
            ffi::gst_message_set_buffering_stats(
                msg,
                mode.into_glib(),
                avg_in,
                avg_out,
                buffering_left,
            );
        }

        msg
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StateChangedBuilder<'a> {
    builder: MessageBuilder<'a>,
    old: crate::State,
    new: crate::State,
    pending: crate::State,
}

impl<'a> StateChangedBuilder<'a> {
    fn new(old: crate::State, new: crate::State, pending: crate::State) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            old,
            new,
            pending,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_state_changed(
        src,
        s.old.into_glib(),
        s.new.into_glib(),
        s.pending.into_glib(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct StateDirtyBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> StateDirtyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_state_dirty(src));
}

#[must_use = "The builder must be built to be used"]
pub struct StepDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
    duration: Option<crate::ClockTime>,
    eos: bool,
}

impl<'a> StepDoneBuilder<'a> {
    fn new(
        amount: GenericFormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: Option<crate::ClockTime>,
        eos: bool,
    ) -> Self {
        skip_assert_initialized!();
        assert_eq!(amount.format(), duration.format());
        Self {
            builder: MessageBuilder::new(),
            amount,
            rate,
            flush,
            intermediate,
            duration,
            eos,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_done(
        src,
        s.amount.format().into_glib(),
        s.amount.value() as u64,
        s.rate,
        s.flush.into_glib(),
        s.intermediate.into_glib(),
        s.duration.into_raw_value() as u64,
        s.eos.into_glib(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct ClockProvideBuilder<'a> {
    builder: MessageBuilder<'a>,
    clock: &'a crate::Clock,
    ready: bool,
}

impl<'a> ClockProvideBuilder<'a> {
    fn new(clock: &'a crate::Clock, ready: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
            ready,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_provide(
        src,
        s.clock.to_glib_none().0,
        s.ready.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct ClockLostBuilder<'a> {
    builder: MessageBuilder<'a>,
    clock: &'a crate::Clock,
}

impl<'a> ClockLostBuilder<'a> {
    fn new(clock: &'a crate::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_lost(
        src,
        s.clock.to_glib_none().0
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct NewClockBuilder<'a> {
    builder: MessageBuilder<'a>,
    clock: &'a crate::Clock,
}

impl<'a> NewClockBuilder<'a> {
    fn new(clock: &'a crate::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_new_clock(
        src,
        s.clock.to_glib_none().0
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct StructureChangeBuilder<'a> {
    builder: MessageBuilder<'a>,
    type_: crate::StructureChangeType,
    owner: &'a crate::Element,
    busy: bool,
}

impl<'a> StructureChangeBuilder<'a> {
    fn new(type_: crate::StructureChangeType, owner: &'a crate::Element, busy: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            owner,
            busy,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_structure_change(
        src,
        s.type_.into_glib(),
        s.owner.to_glib_none().0,
        s.busy.into_glib(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct StreamStatusBuilder<'a> {
    builder: MessageBuilder<'a>,
    type_: crate::StreamStatusType,
    owner: &'a crate::Element,
    status_object: Option<glib::SendValue>,
}

impl<'a> StreamStatusBuilder<'a> {
    fn new(type_: crate::StreamStatusType, owner: &'a crate::Element) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            owner,
            status_object: None,
        }
    }

    pub fn status_object(self, status_object: impl ToSendValue) -> Self {
        Self {
            status_object: Some(status_object.to_send_value()),
            ..self
        }
    }

    pub fn status_object_if(self, status_object: impl ToSendValue, predicate: bool) -> Self {
        if predicate {
            self.status_object(status_object)
        } else {
            self
        }
    }

    pub fn status_object_if_some(self, status_object: Option<impl ToSendValue>) -> Self {
        if let Some(status_object) = status_object {
            self.status_object(status_object)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg =
            ffi::gst_message_new_stream_status(src, s.type_.into_glib(), s.owner.to_glib_none().0);
        if let Some(ref status_object) = s.status_object {
            ffi::gst_message_set_stream_status_object(msg, status_object.to_glib_none().0);
        }
        msg
    });
}

#[must_use = "The builder must be built to be used"]
pub struct ApplicationBuilder<'a> {
    builder: MessageBuilder<'a>,
    structure: Option<crate::Structure>,
}

impl<'a> ApplicationBuilder<'a> {
    fn new(structure: crate::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_application(
        src,
        s.structure.take().unwrap().into_glib_ptr()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct ElementBuilder<'a> {
    builder: MessageBuilder<'a>,
    structure: Option<crate::Structure>,
}

impl<'a> ElementBuilder<'a> {
    fn new(structure: crate::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_element(
        src,
        s.structure.take().unwrap().into_glib_ptr()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct SegmentStartBuilder<'a> {
    builder: MessageBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentStartBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_start(
        src,
        s.position.format().into_glib(),
        s.position.value(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct SegmentDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_done(
        src,
        s.position.format().into_glib(),
        s.position.value(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct DurationChangedBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> DurationChangedBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_duration_changed(src));
}

#[must_use = "The builder must be built to be used"]
pub struct LatencyBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> LatencyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_latency(src));
}

#[must_use = "The builder must be built to be used"]
pub struct AsyncStartBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> AsyncStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_async_start(src));
}

#[must_use = "The builder must be built to be used"]
pub struct AsyncDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
    running_time: Option<crate::ClockTime>,
}

impl<'a> AsyncDoneBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            running_time: None,
        }
    }

    pub fn running_time(mut self, running_time: impl Into<Option<crate::ClockTime>>) -> Self {
        self.running_time = running_time.into();
        self
    }

    pub fn running_time_if(self, running_time: crate::ClockTime, predicate: bool) -> Self {
        if predicate {
            self.running_time(running_time)
        } else {
            self
        }
    }

    pub fn running_time_if_some(self, running_time: Option<crate::ClockTime>) -> Self {
        if let Some(running_time) = running_time {
            self.running_time(running_time)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_async_done(
        src,
        s.running_time.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct RequestStateBuilder<'a> {
    builder: MessageBuilder<'a>,
    state: crate::State,
}

impl<'a> RequestStateBuilder<'a> {
    fn new(state: crate::State) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            state,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_request_state(
        src,
        s.state.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct StepStartBuilder<'a> {
    builder: MessageBuilder<'a>,
    active: bool,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
}

impl<'a> StepStartBuilder<'a> {
    fn new(
        active: bool,
        amount: GenericFormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            active,
            amount,
            rate,
            flush,
            intermediate,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_start(
        src,
        s.active.into_glib(),
        s.amount.format().into_glib(),
        s.amount.value() as u64,
        s.rate,
        s.flush.into_glib(),
        s.intermediate.into_glib(),
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct QosBuilder<'a> {
    builder: MessageBuilder<'a>,
    live: bool,
    running_time: Option<crate::ClockTime>,
    stream_time: Option<crate::ClockTime>,
    timestamp: Option<crate::ClockTime>,
    duration: Option<crate::ClockTime>,
    values: Option<(i64, f64, i32)>,
    stats: Option<(GenericFormattedValue, GenericFormattedValue)>,
}

impl<'a> QosBuilder<'a> {
    fn new(live: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            live,
            running_time: None,
            stream_time: None,
            timestamp: None,
            duration: None,
            values: None,
            stats: None,
        }
    }

    pub fn running_time(mut self, running_time: impl Into<Option<crate::ClockTime>>) -> Self {
        self.running_time = running_time.into();
        self
    }

    pub fn running_time_if(self, running_time: crate::ClockTime, predicate: bool) -> Self {
        if predicate {
            self.running_time(running_time)
        } else {
            self
        }
    }

    pub fn running_time_if_some(self, running_time: Option<crate::ClockTime>) -> Self {
        if let Some(running_time) = running_time {
            self.running_time(running_time)
        } else {
            self
        }
    }

    pub fn stream_time(mut self, stream_time: impl Into<Option<crate::ClockTime>>) -> Self {
        self.stream_time = stream_time.into();
        self
    }

    pub fn stream_time_if(self, stream_time: crate::ClockTime, predicate: bool) -> Self {
        if predicate {
            self.stream_time(stream_time)
        } else {
            self
        }
    }

    pub fn stream_time_if_some(self, stream_time: Option<crate::ClockTime>) -> Self {
        if let Some(stream_time) = stream_time {
            self.stream_time(stream_time)
        } else {
            self
        }
    }

    pub fn timestamp(mut self, timestamp: impl Into<Option<crate::ClockTime>>) -> Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn timestamp_if(self, timestamp: crate::ClockTime, predicate: bool) -> Self {
        if predicate {
            self.timestamp(timestamp)
        } else {
            self
        }
    }

    pub fn timestamp_if_some(self, timestamp: Option<crate::ClockTime>) -> Self {
        if let Some(timestamp) = timestamp {
            self.timestamp(timestamp)
        } else {
            self
        }
    }

    pub fn duration(mut self, duration: impl Into<Option<crate::ClockTime>>) -> Self {
        self.duration = duration.into();
        self
    }

    pub fn duration_if(self, duration: crate::ClockTime, predicate: bool) -> Self {
        if predicate {
            self.duration(duration)
        } else {
            self
        }
    }

    pub fn duration_if_some(self, duration: Option<crate::ClockTime>) -> Self {
        if let Some(duration) = duration {
            self.duration(duration)
        } else {
            self
        }
    }

    pub fn values(self, jitter: i64, proportion: f64, quality: i32) -> Self {
        Self {
            values: Some((jitter, proportion, quality)),
            ..self
        }
    }

    pub fn stats<V: FormattedValue>(
        self,
        processed: V,
        dropped: impl CompatibleFormattedValue<V>,
    ) -> Self {
        let dropped = dropped.try_into_checked(processed).unwrap();
        Self {
            stats: Some((processed.into(), dropped.into())),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_qos(
            src,
            s.live.into_glib(),
            s.running_time.into_glib(),
            s.stream_time.into_glib(),
            s.timestamp.into_glib(),
            s.duration.into_glib(),
        );
        if let Some((jitter, proportion, quality)) = s.values {
            ffi::gst_message_set_qos_values(msg, jitter, proportion, quality);
        }
        if let Some((processed, dropped)) = s.stats {
            ffi::gst_message_set_qos_stats(
                msg,
                processed.format().into_glib(),
                processed.value() as u64,
                dropped.value() as u64,
            );
        }
        msg
    });
}

#[must_use = "The builder must be built to be used"]
pub struct ProgressBuilder<'a> {
    builder: MessageBuilder<'a>,
    type_: crate::ProgressType,
    code: &'a str,
    text: &'a str,
}

impl<'a> ProgressBuilder<'a> {
    fn new(type_: crate::ProgressType, code: &'a str, text: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            code,
            text,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_progress(
        src,
        s.type_.into_glib(),
        s.code.to_glib_none().0,
        s.text.to_glib_none().0,
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct TocBuilder<'a> {
    builder: MessageBuilder<'a>,
    toc: &'a crate::Toc,
    updated: bool,
}

impl<'a> TocBuilder<'a> {
    fn new(toc: &'a crate::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            toc,
            updated,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_message_new_toc(
        src,
        s.toc.to_glib_none().0,
        s.updated.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct ResetTimeBuilder<'a> {
    builder: MessageBuilder<'a>,
    running_time: crate::ClockTime,
}

impl<'a> ResetTimeBuilder<'a> {
    fn new(running_time: crate::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_reset_time(
        src,
        s.running_time.into_glib()
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct StreamStartBuilder<'a> {
    builder: MessageBuilder<'a>,
    group_id: Option<GroupId>,
}

impl<'a> StreamStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            group_id: None,
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

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_stream_start(src);
        if let Some(group_id) = s.group_id {
            ffi::gst_message_set_group_id(msg, group_id.0.get());
        }
        msg
    });
}

#[must_use = "The builder must be built to be used"]
pub struct NeedContextBuilder<'a> {
    builder: MessageBuilder<'a>,
    context_type: &'a str,
}

impl<'a> NeedContextBuilder<'a> {
    fn new(context_type: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            context_type,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_need_context(
        src,
        s.context_type.to_glib_none().0
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct HaveContextBuilder<'a> {
    builder: MessageBuilder<'a>,
    context: Option<crate::Context>,
}

impl<'a> HaveContextBuilder<'a> {
    fn new(context: crate::Context) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            context: Some(context),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let context = s.context.take().unwrap();
        ffi::gst_message_new_have_context(src, context.into_glib_ptr())
    });
}

#[must_use = "The builder must be built to be used"]
pub struct DeviceAddedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a crate::Device,
}

impl<'a> DeviceAddedBuilder<'a> {
    fn new(device: &'a crate::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_added(
        src,
        s.device.to_glib_none().0
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct DeviceRemovedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a crate::Device,
}

impl<'a> DeviceRemovedBuilder<'a> {
    fn new(device: &'a crate::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_removed(
        src,
        s.device.to_glib_none().0
    ));
}

#[must_use = "The builder must be built to be used"]
pub struct PropertyNotifyBuilder<'a> {
    builder: MessageBuilder<'a>,
    property_name: &'a str,
    value: Option<glib::SendValue>,
}

impl<'a> PropertyNotifyBuilder<'a> {
    fn new(property_name: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            property_name,
            value: None,
        }
    }

    pub fn value(self, value: impl ToSendValue) -> Self {
        Self {
            value: Some(value.to_send_value()),
            ..self
        }
    }

    pub fn value_if(self, value: impl ToSendValue, predicate: bool) -> Self {
        if predicate {
            self.value(value)
        } else {
            self
        }
    }

    pub fn value_if_some(self, value: Option<impl ToSendValue>) -> Self {
        if let Some(value) = value {
            self.value(value)
        } else {
            self
        }
    }

    pub fn value_from_iter<V: ValueType + ToSendValue + FromIterator<SendValue>>(
        self,
        name: &'a str,
        iter: impl IntoIterator<Item = impl ToSendValue>,
    ) -> Self {
        let iter = iter.into_iter().map(|item| item.to_send_value());
        self.other_field(name, V::from_iter(iter))
    }

    pub fn value_field_if_not_empty<V: ValueType + ToSendValue + FromIterator<SendValue>>(
        self,
        name: &'a str,
        iter: impl IntoIterator<Item = impl ToSendValue>,
    ) -> Self {
        let mut iter = iter.into_iter().peekable();
        if iter.peek().is_some() {
            let iter = iter.map(|item| item.to_send_value());
            self.other_field(name, V::from_iter(iter))
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let v = s.value.take();
        ffi::gst_message_new_property_notify(
            src,
            s.property_name.to_glib_none().0,
            v.as_ref().map(|v| v.as_ptr()).unwrap_or(ptr::null_mut()),
        )
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StreamCollectionBuilder<'a> {
    builder: MessageBuilder<'a>,
    collection: &'a crate::StreamCollection,
}

impl<'a> StreamCollectionBuilder<'a> {
    fn new(collection: &'a crate::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            collection,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_stream_collection(src, s.collection.to_glib_none().0)
    });
}

#[must_use = "The builder must be built to be used"]
pub struct StreamsSelectedBuilder<'a> {
    builder: MessageBuilder<'a>,
    collection: &'a crate::StreamCollection,
    streams: Option<Vec<crate::Stream>>,
}

impl<'a> StreamsSelectedBuilder<'a> {
    fn new(collection: &'a crate::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            collection,
            streams: None,
        }
    }

    pub fn streams(
        self,
        streams: impl IntoIterator<Item = impl std::borrow::Borrow<crate::Stream>>,
    ) -> Self {
        Self {
            streams: Some(
                streams
                    .into_iter()
                    .map(|s| s.borrow().clone())
                    .collect::<Vec<_>>(),
            ),
            ..self
        }
    }

    pub fn streams_if(
        self,
        streams: impl IntoIterator<Item = impl std::borrow::Borrow<crate::Stream>>,
        predicate: bool,
    ) -> Self {
        if predicate {
            self.streams(streams)
        } else {
            self
        }
    }

    pub fn streams_if_some(
        self,
        streams: Option<impl IntoIterator<Item = impl std::borrow::Borrow<crate::Stream>>>,
    ) -> Self {
        if let Some(streams) = streams {
            self.streams(streams)
        } else {
            self
        }
    }

    pub fn streams_if_not_empty(
        self,
        streams: impl IntoIterator<Item = impl std::borrow::Borrow<crate::Stream>>,
    ) -> Self {
        let mut streams = streams.into_iter().peekable();
        if streams.peek().is_some() {
            self.streams(streams)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_streams_selected(src, s.collection.to_glib_none().0);
        if let Some(ref streams) = s.streams {
            for stream in streams {
                ffi::gst_message_streams_selected_add(msg, stream.to_glib_none().0);
            }
        }
        msg
    });
}

#[must_use = "The builder must be built to be used"]
pub struct RedirectBuilder<'a> {
    builder: MessageBuilder<'a>,
    location: &'a str,
    tag_list: Option<&'a TagList>,
    entry_struct: Option<Structure>,
    #[allow(clippy::type_complexity)]
    entries: Option<&'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)]>,
}

impl<'a> RedirectBuilder<'a> {
    fn new(location: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            location,
            tag_list: None,
            entry_struct: None,
            entries: None,
        }
    }

    pub fn tag_list(self, tag_list: &'a TagList) -> Self {
        Self {
            tag_list: Some(tag_list),
            ..self
        }
    }

    pub fn tag_list_if(self, tag_list: &'a TagList, predicate: bool) -> Self {
        if predicate {
            self.tag_list(tag_list)
        } else {
            self
        }
    }

    pub fn tag_list_if_some(self, tag_list: Option<&'a TagList>) -> Self {
        if let Some(tag_list) = tag_list {
            self.tag_list(tag_list)
        } else {
            self
        }
    }

    pub fn entry_struct(self, entry_struct: Structure) -> Self {
        Self {
            entry_struct: Some(entry_struct),
            ..self
        }
    }

    pub fn entry_struct_if(self, entry_struct: Structure, predicate: bool) -> Self {
        if predicate {
            self.entry_struct(entry_struct)
        } else {
            self
        }
    }

    pub fn entry_struct_if_some(self, entry_struct: Option<Structure>) -> Self {
        if let Some(entry_struct) = entry_struct {
            self.entry_struct(entry_struct)
        } else {
            self
        }
    }

    pub fn entries(
        self,
        entries: &'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)],
    ) -> Self {
        skip_assert_initialized!();
        Self {
            entries: Some(entries),
            ..self
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn entries_if(
        self,
        entries: &'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)],
        predicate: bool,
    ) -> Self {
        if predicate {
            self.entries(entries)
        } else {
            self
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn entries_if_some(
        self,
        entries: Option<&'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)]>,
    ) -> Self {
        if let Some(entries) = entries {
            self.entries(entries)
        } else {
            self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let entry_struct = s.entry_struct.take();
        let entry_struct_ptr = if let Some(entry_struct) = entry_struct {
            entry_struct.into_glib_ptr()
        } else {
            ptr::null_mut()
        };

        let msg = ffi::gst_message_new_redirect(
            src,
            s.location.to_glib_none().0,
            s.tag_list.to_glib_full(),
            entry_struct_ptr,
        );
        if let Some(entries) = s.entries {
            for &(location, tag_list, entry_struct) in entries {
                let entry_struct = entry_struct.cloned();
                let entry_struct_ptr = if let Some(entry_struct) = entry_struct {
                    entry_struct.into_glib_ptr()
                } else {
                    ptr::null_mut()
                };
                ffi::gst_message_add_redirect_entry(
                    msg,
                    location.to_glib_none().0,
                    tag_list.to_glib_full(),
                    entry_struct_ptr,
                );
            }
        }
        msg
    });
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
#[must_use = "The builder must be built to be used"]
pub struct DeviceChangedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a crate::Device,
    changed_device: &'a crate::Device,
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl<'a> DeviceChangedBuilder<'a> {
    fn new(device: &'a crate::Device, changed_device: &'a crate::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
            changed_device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_changed(
        src,
        s.device.to_glib_none().0,
        s.changed_device.to_glib_none().0,
    ));
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
#[must_use = "The builder must be built to be used"]
pub struct InstantRateRequestBuilder<'a> {
    builder: MessageBuilder<'a>,
    rate_multiplier: f64,
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl<'a> InstantRateRequestBuilder<'a> {
    fn new(rate_multiplier: f64) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            rate_multiplier,
        }
    }

    message_builder_generic_impl!(
        |s: &mut Self, src| ffi::gst_message_new_instant_rate_request(src, s.rate_multiplier,)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        crate::init().unwrap();

        // Message without arguments
        let seqnum = Seqnum::next();
        let eos_msg = Eos::builder().seqnum(seqnum).build();
        match eos_msg.view() {
            MessageView::Eos(eos_msg) => {
                assert_eq!(eos_msg.seqnum(), seqnum);
                assert!(eos_msg.structure().is_none());
            }
            _ => panic!("eos_msg.view() is not a MessageView::Eos(_)"),
        }

        // Message with arguments
        let buffering_msg = Buffering::new(42);
        match buffering_msg.view() {
            MessageView::Buffering(buffering_msg) => {
                assert_eq!(buffering_msg.percent(), 42);
            }
            _ => panic!("buffering_msg.view() is not a MessageView::Buffering(_)"),
        }
    }

    #[test]
    #[allow(deprecated)]
    fn test_other_fields() {
        crate::init().unwrap();

        let seqnum = Seqnum::next();
        let eos_msg = Eos::builder()
            .other_fields(&[("extra-field", &true)])
            .seqnum(seqnum)
            .build();
        match eos_msg.view() {
            MessageView::Eos(eos_msg) => {
                assert_eq!(eos_msg.seqnum(), seqnum);
                if let Some(other_fields) = eos_msg.structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("eos_msg.view() is not a MessageView::Eos(_)"),
        }

        let buffering_msg = Buffering::builder(42)
            .other_field("extra-field", true)
            .build();
        match buffering_msg.view() {
            MessageView::Buffering(buffering_msg) => {
                assert_eq!(buffering_msg.percent(), 42);
                if let Some(other_fields) = buffering_msg.structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("buffering_msg.view() is not a MessageView::Buffering(_)"),
        }
    }

    #[test]
    fn test_get_seqnum_valid() {
        crate::init().unwrap();

        let msg = StreamStart::new();
        let seqnum = Seqnum(
            NonZeroU32::new(unsafe { ffi::gst_message_get_seqnum(msg.as_mut_ptr()) }).unwrap(),
        );

        match msg.view() {
            MessageView::StreamStart(stream_start) => assert_eq!(seqnum, stream_start.seqnum()),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_seqnum_invalid() {
        crate::init().unwrap();

        let msg = StreamStart::new();
        let seqnum_init = msg.seqnum();

        // Invalid the seqnum
        unsafe {
            (*msg.as_mut_ptr()).seqnum = ffi::GST_SEQNUM_INVALID as u32;
            assert_eq!(0, (*msg.as_ptr()).seqnum);
        };

        match msg.view() {
            MessageView::StreamStart(stream_start) => {
                // get_seqnum is expected to return a new Seqnum,
                // further in the sequence than the last known seqnum.
                assert!(seqnum_init < stream_start.seqnum());
            }
            _ => panic!(),
        }
    }
}
