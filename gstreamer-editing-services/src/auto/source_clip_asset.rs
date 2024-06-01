// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Asset, ClipAsset, MetaContainer};

glib::wrapper! {
    #[doc(alias = "GESSourceClipAsset")]
    pub struct SourceClipAsset(Object<ffi::GESSourceClipAsset, ffi::GESSourceClipAssetClass>) @extends ClipAsset, Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_source_clip_asset_get_type(),
    }
}

impl SourceClipAsset {
    pub const NONE: Option<&'static SourceClipAsset> = None;
}

unsafe impl Send for SourceClipAsset {}
unsafe impl Sync for SourceClipAsset {}
