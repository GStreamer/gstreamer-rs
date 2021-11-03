// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::OperationClip;
use crate::TimelineElement;

glib::wrapper! {
    #[doc(alias = "GESOverlayClip")]
    pub struct OverlayClip(Object<ffi::GESOverlayClip, ffi::GESOverlayClipClass>) @extends OperationClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_overlay_clip_get_type(),
    }
}

impl OverlayClip {}

impl OverlayClip {
    pub const NONE: Option<&'static OverlayClip> = None;
}
