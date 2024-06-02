// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ClockTime, ControlSource};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ControlSource>> Sealed for T {}
}

pub trait ControlSourceExtManual: sealed::Sealed + IsA<ControlSource> + 'static {
    #[doc(alias = "get_value_array")]
    #[doc(alias = "gst_control_source_get_value_array")]
    fn value_array(
        &self,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [f64],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib::result_from_gboolean!(
                crate::ffi::gst_control_source_get_value_array(
                    self.as_ref().to_glib_none().0,
                    timestamp.into_glib(),
                    interval.into_glib(),
                    n_values,
                    values.to_glib_none().0,
                ),
                "Failed to get value array"
            )
        }
    }
}

impl<O: IsA<ControlSource>> ControlSourceExtManual for O {}
