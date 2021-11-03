// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Extractable;
use crate::MetaContainer;
use crate::Operation;
use crate::TextHAlign;
use crate::TextVAlign;
use crate::TimelineElement;
use crate::TrackElement;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESTextOverlay")]
    pub struct TextOverlay(Object<ffi::GESTextOverlay, ffi::GESTextOverlayClass>) @extends Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_text_overlay_get_type(),
    }
}

impl TextOverlay {
    #[cfg_attr(feature = "v1_18", deprecated = "Since 1.18")]
    #[doc(alias = "ges_text_overlay_new")]
    pub fn new() -> Option<TextOverlay> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_text_overlay_new()) }
    }
}

impl TextOverlay {
    pub const NONE: Option<&'static TextOverlay> = None;
}

pub trait TextOverlayExt: 'static {
    #[doc(alias = "ges_text_overlay_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> u32;

    #[doc(alias = "ges_text_overlay_get_font_desc")]
    #[doc(alias = "get_font_desc")]
    fn font_desc(&self) -> Option<glib::GString>;

    #[doc(alias = "ges_text_overlay_get_halignment")]
    #[doc(alias = "get_halignment")]
    fn halignment(&self) -> TextHAlign;

    #[doc(alias = "ges_text_overlay_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString>;

    #[doc(alias = "ges_text_overlay_get_valignment")]
    #[doc(alias = "get_valignment")]
    fn valignment(&self) -> TextVAlign;

    #[doc(alias = "ges_text_overlay_get_xpos")]
    #[doc(alias = "get_xpos")]
    fn xpos(&self) -> f64;

    #[doc(alias = "ges_text_overlay_get_ypos")]
    #[doc(alias = "get_ypos")]
    fn ypos(&self) -> f64;

    #[doc(alias = "ges_text_overlay_set_color")]
    fn set_color(&self, color: u32);

    #[doc(alias = "ges_text_overlay_set_font_desc")]
    fn set_font_desc(&self, font_desc: &str);

    #[doc(alias = "ges_text_overlay_set_halignment")]
    fn set_halignment(&self, halign: TextHAlign);

    #[doc(alias = "ges_text_overlay_set_text")]
    fn set_text(&self, text: &str);

    #[doc(alias = "ges_text_overlay_set_valignment")]
    fn set_valignment(&self, valign: TextVAlign);

    #[doc(alias = "ges_text_overlay_set_xpos")]
    fn set_xpos(&self, position: f64);

    #[doc(alias = "ges_text_overlay_set_ypos")]
    fn set_ypos(&self, position: f64);
}

impl<O: IsA<TextOverlay>> TextOverlayExt for O {
    fn color(&self) -> u32 {
        unsafe { ffi::ges_text_overlay_get_color(self.as_ref().to_glib_none().0) }
    }

    fn font_desc(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ges_text_overlay_get_font_desc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn halignment(&self) -> TextHAlign {
        unsafe {
            from_glib(ffi::ges_text_overlay_get_halignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ges_text_overlay_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn valignment(&self) -> TextVAlign {
        unsafe {
            from_glib(ffi::ges_text_overlay_get_valignment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn xpos(&self) -> f64 {
        unsafe { ffi::ges_text_overlay_get_xpos(self.as_ref().to_glib_none().0) }
    }

    fn ypos(&self) -> f64 {
        unsafe { ffi::ges_text_overlay_get_ypos(self.as_ref().to_glib_none().0) }
    }

    fn set_color(&self, color: u32) {
        unsafe {
            ffi::ges_text_overlay_set_color(self.as_ref().to_glib_none().0, color);
        }
    }

    fn set_font_desc(&self, font_desc: &str) {
        unsafe {
            ffi::ges_text_overlay_set_font_desc(
                self.as_ref().to_glib_none().0,
                font_desc.to_glib_none().0,
            );
        }
    }

    fn set_halignment(&self, halign: TextHAlign) {
        unsafe {
            ffi::ges_text_overlay_set_halignment(
                self.as_ref().to_glib_none().0,
                halign.into_glib(),
            );
        }
    }

    fn set_text(&self, text: &str) {
        unsafe {
            ffi::ges_text_overlay_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_valignment(&self, valign: TextVAlign) {
        unsafe {
            ffi::ges_text_overlay_set_valignment(
                self.as_ref().to_glib_none().0,
                valign.into_glib(),
            );
        }
    }

    fn set_xpos(&self, position: f64) {
        unsafe {
            ffi::ges_text_overlay_set_xpos(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_ypos(&self, position: f64) {
        unsafe {
            ffi::ges_text_overlay_set_ypos(self.as_ref().to_glib_none().0, position);
        }
    }
}
