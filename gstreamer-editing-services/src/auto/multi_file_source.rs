// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Extractable;
use crate::MetaContainer;
use crate::Source;
use crate::TimelineElement;
use crate::TrackElement;
use crate::VideoSource;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    #[doc(alias = "GESMultiFileSource")]
    pub struct MultiFileSource(Object<ffi::GESMultiFileSource, ffi::GESMultiFileSourceClass>) @extends VideoSource, Source, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_multi_file_source_get_type(),
    }
}

impl MultiFileSource {
    #[doc(alias = "ges_multi_file_source_new")]
    pub fn new(uri: &str) -> MultiFileSource {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_multi_file_source_new(uri.to_glib_none().0)) }
    }
}

impl MultiFileSource {
    pub const NONE: Option<&'static MultiFileSource> = None;
}

pub trait MultiFileSourceExt: 'static {
    fn uri(&self) -> Option<glib::GString>;
}

impl<O: IsA<MultiFileSource>> MultiFileSourceExt for O {
    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"uri\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `uri` getter")
        }
    }
}
