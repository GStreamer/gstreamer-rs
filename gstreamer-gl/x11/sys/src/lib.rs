// Generated by gir (https://github.com/gtk-rs/gir @ b3147f2b6043)
// from gir-files (https://github.com/gtk-rs/gir-files @ 7fa401e3ee5d)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 2860909848fa)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayX11Class {
    pub object_class: gst_gl::GstGLDisplayClass,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayX11Class {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayX11Class @ {:p}", self))
            .field("object_class", &self.object_class)
            .field("_padding", &self._padding)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstGLDisplayX11 {
    pub parent: gst_gl::GstGLDisplay,
    pub name: *mut c_char,
    pub display: gpointer,
    pub xcb_connection: gpointer,
    pub foreign_display: gboolean,
    pub _padding: [gpointer; 4],
}

impl ::std::fmt::Debug for GstGLDisplayX11 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstGLDisplayX11 @ {:p}", self))
            .finish()
    }
}

#[link(name = "gstgl-1.0")]
extern "C" {

    //=========================================================================
    // GstGLDisplayX11
    //=========================================================================
    pub fn gst_gl_display_x11_get_type() -> GType;
    pub fn gst_gl_display_x11_new(name: *const c_char) -> *mut GstGLDisplayX11;
    pub fn gst_gl_display_x11_new_with_display(display: gpointer) -> *mut GstGLDisplayX11;

}
