// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use NetTimeProvider;

use glib::IsA;
use glib::translate::*;
use gst;

impl NetTimeProvider {
    pub fn new<'a, P: IsA<gst::Clock>, Q: Into<Option<&'a str>>>(clock: &P, address: Q, port: i32) -> NetTimeProvider {
        assert_initialized_main_thread!();
        let address = address.into();
        let address = address.to_glib_none();

        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            unsafe {
                from_glib_full(ffi::gst_net_time_provider_new(clock.to_glib_none().0, address.0, port))
            }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13
            unsafe {
                from_glib_none(ffi::gst_net_time_provider_new(clock.to_glib_none().0, address.0, port))
            }
        }
    }
}
