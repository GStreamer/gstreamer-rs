// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Asset;
use glib::object::IsA;
use glib::translate::*;

glib::wrapper! {
    pub struct Extractable(Interface<ffi::GESExtractable, ffi::GESExtractableInterface>);

    match fn {
        get_type => || ffi::ges_extractable_get_type(),
    }
}

pub const NONE_EXTRACTABLE: Option<&Extractable> = None;

pub trait ExtractableExt: 'static {
    #[doc(alias = "ges_extractable_get_asset")]
    fn get_asset(&self) -> Option<Asset>;

    #[doc(alias = "ges_extractable_get_id")]
    fn get_id(&self) -> Option<glib::GString>;

    #[doc(alias = "ges_extractable_set_asset")]
    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError>;
}

impl<O: IsA<Extractable>> ExtractableExt for O {
    fn get_asset(&self) -> Option<Asset> {
        unsafe {
            from_glib_none(ffi::ges_extractable_get_asset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_id(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::ges_extractable_get_id(self.as_ref().to_glib_none().0)) }
    }

    fn set_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::ges_extractable_set_asset(
                    self.as_ref().to_glib_none().0,
                    asset.as_ref().to_glib_none().0
                ),
                "Failed to set asset"
            )
        }
    }
}
