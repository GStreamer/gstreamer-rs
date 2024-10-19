// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Extractable, MetaContainer, Source, TextHAlign, TextVAlign, TimelineElement, TrackElement,
    VideoSource,
};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESTitleSource")]
    pub struct TitleSource(Object<ffi::GESTitleSource, ffi::GESTitleSourceClass>) @extends VideoSource, Source, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_title_source_get_type(),
    }
}

impl TitleSource {
    pub const NONE: Option<&'static TitleSource> = None;
}

pub trait TitleSourceExt: IsA<TitleSource> + 'static {
    #[doc(alias = "ges_title_source_get_background_color")]
    #[doc(alias = "get_background_color")]
    fn background_color(&self) -> u32 {
        unsafe { ffi::ges_title_source_get_background_color(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_source_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    fn font_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_title_source_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_title_source_get_halignment")]
    #[doc(alias = "get_halignment")]
    fn halignment(&self) -> TextHAlign {
        unsafe {
            from_glib(ffi::ges_title_source_get_halignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v1_16", deprecated = "Since 1.16")]
    #[allow(deprecated)]
    #[doc(alias = "ges_title_source_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::ges_title_source_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_title_source_get_text_color")]
    #[doc(alias = "get_text_color")]
    fn text_color(&self) -> u32 {
        unsafe { ffi::ges_title_source_get_text_color(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_title_source_get_valignment")]
    #[doc(alias = "get_valignment")]
    fn valignment(&self) -> TextVAlign {
        unsafe {
            from_glib(ffi::ges_title_source_get_valignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_title_source_get_xpos")]
    #[doc(alias = "get_xpos")]
    fn xpos(&self) -> f64 {
        unsafe { ffi::ges_title_source_get_xpos(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_title_source_get_ypos")]
    #[doc(alias = "get_ypos")]
    fn ypos(&self) -> f64 {
        unsafe { ffi::ges_title_source_get_ypos(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "ges_title_source_set_background_color")]
    fn set_background_color(&self, color: u32) {
        unsafe {
            ffi::ges_title_source_set_background_color(self.as_ref().to_glib_none().0, color);
        }
    }

    #[doc(alias = "ges_title_source_set_font_desc")]
    fn set_font_desc(&self, font_desc: Option<&str>) {
        unsafe {
            ffi::ges_title_source_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "ges_title_source_set_halignment")]
    fn set_halignment(&self, halign: TextHAlign) {
        unsafe {
            ffi::ges_title_source_set_halignment(
                self.as_ref().to_glib_none().0,
                halign.into_glib(),
            );
        }
    }

    #[doc(alias = "ges_title_source_set_text")]
    fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::ges_title_source_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "ges_title_source_set_text_color")]
    fn set_text_color(&self, color: u32) {
        unsafe {
            ffi::ges_title_source_set_text_color(self.as_ref().to_glib_none().0, color);
        }
    }

    #[doc(alias = "ges_title_source_set_valignment")]
    fn set_valignment(&self, valign: TextVAlign) {
        unsafe {
            ffi::ges_title_source_set_valignment(
                self.as_ref().to_glib_none().0,
                valign.into_glib(),
            );
        }
    }

    #[doc(alias = "ges_title_source_set_xpos")]
    fn set_xpos(&self, position: f64) {
        unsafe {
            ffi::ges_title_source_set_xpos(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "ges_title_source_set_ypos")]
    fn set_ypos(&self, position: f64) {
        unsafe {
            ffi::ges_title_source_set_ypos(self.as_ref().to_glib_none().0, position);
        }
    }
}

impl<O: IsA<TitleSource>> TitleSourceExt for O {}
