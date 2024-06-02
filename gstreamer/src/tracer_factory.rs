// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::TracerFactory;

impl TracerFactory {
    #[doc(alias = "gst_tracer_factory_get_list")]
    #[doc(alias = "get_list")]
    pub fn factories() -> glib::List<TracerFactory> {
        assert_initialized_main_thread!();
        unsafe { FromGlibPtrContainer::from_glib_full(crate::ffi::gst_tracer_factory_get_list()) }
    }
}
