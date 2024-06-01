// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Extractable, MetaContainer, Operation, TimelineElement, TrackElement};

glib::wrapper! {
    #[doc(alias = "GESTransition")]
    pub struct Transition(Object<ffi::GESTransition, ffi::GESTransitionClass>) @extends Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_transition_get_type(),
    }
}

impl Transition {
    pub const NONE: Option<&'static Transition> = None;
}
