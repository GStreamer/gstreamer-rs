// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;

pub trait GstParamSpecExt {
    #[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
    fn array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &glib::ParamSpec,
        flags: glib::ParamFlags,
    ) -> Self;

    fn fraction(
        name: &str,
        nick: &str,
        blurb: &str,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> Self;
}

impl GstParamSpecExt for glib::ParamSpec {
    #[cfg(any(feature = "v1_14", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v1_14")))]
    fn array(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &glib::ParamSpec,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_param_spec_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    fn fraction(
        name: &str,
        nick: &str,
        blurb: &str,
        min: crate::Fraction,
        max: crate::Fraction,
        default: crate::Fraction,
        flags: glib::ParamFlags,
    ) -> glib::ParamSpec {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_param_spec_fraction(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                *min.numer(),
                *min.denom(),
                *max.numer(),
                *max.denom(),
                *default.numer(),
                *default.denom(),
                flags.to_glib(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_trait() {
        crate::init().unwrap();

        let _pspec = glib::ParamSpec::fraction(
            "foo",
            "Foo",
            "Foo Bar",
            (0, 1).into(),
            (100, 1).into(),
            (1, 1).into(),
            glib::ParamFlags::READWRITE,
        );
    }
}
