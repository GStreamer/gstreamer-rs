// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::BaseEffectClip;
use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::MetaContainer;
use crate::OperationClip;
use crate::TimelineElement;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    #[doc(alias = "GESEffectClip")]
    pub struct EffectClip(Object<ffi::GESEffectClip, ffi::GESEffectClipClass>) @extends BaseEffectClip, OperationClip, Clip, Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_effect_clip_get_type(),
    }
}

impl EffectClip {
    #[doc(alias = "ges_effect_clip_new")]
    pub fn new(
        video_bin_description: Option<&str>,
        audio_bin_description: Option<&str>,
    ) -> Option<EffectClip> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_effect_clip_new(
                video_bin_description.to_glib_none().0,
                audio_bin_description.to_glib_none().0,
            ))
        }
    }
}

impl EffectClip {
    pub const NONE: Option<&'static EffectClip> = None;
}

pub trait EffectClipExt: 'static {
    #[doc(alias = "audio-bin-description")]
    fn audio_bin_description(&self) -> Option<glib::GString>;

    #[doc(alias = "video-bin-description")]
    fn video_bin_description(&self) -> Option<glib::GString>;
}

impl<O: IsA<EffectClip>> EffectClipExt for O {
    fn audio_bin_description(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"audio-bin-description\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `audio-bin-description` getter")
        }
    }

    fn video_bin_description(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"video-bin-description\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `video-bin-description` getter")
        }
    }
}
