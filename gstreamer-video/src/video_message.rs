// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{
    translate::{from_glib, from_glib_full, IntoGlib, ToGlibPtr},
    value::ToSendValue,
};
use gst::{ffi as gst_ffi, prelude::*, Message, Object, Seqnum};

use crate::{ffi, NavigationMessageType};

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

        pub fn other_field(self, name: &'a str, value: impl ToSendValue) -> Self {
            Self {
                builder: self.builder.other_field(name, value),
                ..self
            }
        }

        gst::impl_builder_gvalue_extra_setters!(other_field);

        #[deprecated = "use builder.other_field() instead"]
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
            skip_assert_initialized!();
            unsafe {
                let src = self.builder.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.builder.seqnum {
                    gst_ffi::gst_message_set_seqnum(msg, seqnum.into_glib());
                }

                if !self.builder.other_fields.is_empty() {
                    let structure = gst_ffi::gst_message_writable_structure(msg);

                    if !structure.is_null() {
                        let structure =
                            gst::StructureRef::from_glib_borrow_mut(structure as *mut _);

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

struct MessageBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, glib::SendValue)>,
}

impl<'a> MessageBuilder<'a> {
    pub(crate) fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
        Self {
            src: Some(src.clone().upcast::<Object>()),
            ..self
        }
    }

    pub fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
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

#[must_use = "The builder must be built to be used"]
pub struct NavigationEventMessageBuilder<'a> {
    builder: MessageBuilder<'a>,
    event: gst::Event,
}

impl<'a> NavigationEventMessageBuilder<'a> {
    fn new(event: gst::Event) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            event,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_navigation_message_new_event(
        src,
        s.event.to_glib_none().0
    ));
}

#[derive(Clone, Debug)]
pub struct NavigationEventMessage {
    pub event: gst::Event,
}

impl PartialEq for NavigationEventMessage {
    fn eq(&self, other: &Self) -> bool {
        self.event.as_ptr() == other.event.as_ptr()
    }
}

impl Eq for NavigationEventMessage {}

impl NavigationEventMessage {
    #[doc(alias = "gst_navigation_message_new_event")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(event: gst::Event) -> gst::Message {
        skip_assert_initialized!();
        NavigationEventMessageBuilder::new(event).build()
    }

    pub fn builder<'a>(event: gst::Event) -> NavigationEventMessageBuilder<'a> {
        skip_assert_initialized!();
        NavigationEventMessageBuilder::new(event)
    }

    #[doc(alias = "gst_navigation_message_parse_event")]
    pub fn parse(msg: &gst::MessageRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut event = ptr::null_mut();
            let ret = from_glib(ffi::gst_navigation_message_parse_event(
                msg.as_mut_ptr(),
                &mut event,
            ));
            if ret {
                Ok(Self {
                    event: from_glib_full(event),
                })
            } else {
                Err(glib::bool_error!("Invalid navigation event msg"))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NavigationMessage {
    Event(NavigationEventMessage),
}

impl NavigationMessage {
    #[doc(alias = "gst_navigation_message_get_type")]
    pub fn type_(msg: &gst::MessageRef) -> NavigationMessageType {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gst_navigation_message_get_type(msg.as_mut_ptr())) }
    }

    #[doc(alias = "gst_navigation_message_parse_event")]
    pub fn parse(msg: &gst::MessageRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        let event_type: NavigationMessageType = Self::type_(msg);

        match event_type {
            NavigationMessageType::Event => NavigationEventMessage::parse(msg).map(Self::Event),
            _ => Err(glib::bool_error!(
                "Unsupported navigation msg {:?}",
                event_type
            )),
        }
    }
}
