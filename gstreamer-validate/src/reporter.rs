use glib::translate::*;

use crate::action::Action;

impl crate::Reporter {
    #[doc(alias = "gst_validate_report_action")]
    pub fn report_action(&self, action: &Action, issue_id: crate::IssueId, message: &str) {
        unsafe {
            crate::ffi::gst_validate_report_action(
                self.to_glib_none().0,
                action.to_glib_none().0,
                issue_id.into_glib(),
                message.to_glib_none().0,
            );
        }
    }
}
