// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ReportLevel;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Issue(Boxed<ffi::GstValidateIssue>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gst_validate_issue_get_type(), ptr as *mut _) as *mut ffi::GstValidateIssue,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gst_validate_issue_get_type(), ptr as *mut _),
        type_ => || ffi::gst_validate_issue_get_type(),
    }
}

impl Issue {
    //#[doc(alias = "gst_validate_issue_new")]
    //pub fn new(issue_id: /*Ignored*/IssueId, summary: &str, description: &str, default_level: ReportLevel) -> Issue {
    //    unsafe { TODO: call ffi:gst_validate_issue_new() }
    //}

    //#[doc(alias = "gst_validate_issue_new_full")]
    //pub fn new_full(issue_id: /*Ignored*/IssueId, summary: &str, description: &str, default_level: ReportLevel, flags: IssueFlags) -> Issue {
    //    unsafe { TODO: call ffi:gst_validate_issue_new_full() }
    //}

    #[doc(alias = "gst_validate_issue_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&mut self) -> u32 {
        unsafe { ffi::gst_validate_issue_get_id(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "gst_validate_issue_register")]
    pub fn register(&mut self) {
        unsafe {
            ffi::gst_validate_issue_register(self.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "gst_validate_issue_set_default_level")]
    pub fn set_default_level(&mut self, default_level: ReportLevel) {
        unsafe {
            ffi::gst_validate_issue_set_default_level(
                self.to_glib_none_mut().0,
                default_level.into_glib(),
            );
        }
    }

    //#[doc(alias = "gst_validate_issue_from_id")]
    //pub fn from_id(issue_id: /*Ignored*/IssueId) -> Option<Issue> {
    //    unsafe { TODO: call ffi:gst_validate_issue_from_id() }
    //}
}

unsafe impl Send for Issue {}
unsafe impl Sync for Issue {}