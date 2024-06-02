// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};

use crate::{ffi, RTSPAddress, RTSPAddressPool, RTSPAddressPoolResult};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPAddressPool>> Sealed for T {}
}

pub trait RTSPAddressPoolExtManual: sealed::Sealed + IsA<RTSPAddressPool> + 'static {
    #[doc(alias = "gst_rtsp_address_pool_reserve_address")]
    fn reserve_address(
        &self,
        ip_address: &str,
        port: u32,
        n_ports: u32,
        ttl: u32,
    ) -> Result<RTSPAddress, RTSPAddressPoolResult> {
        unsafe {
            let mut address = ptr::null_mut();
            let ret = from_glib(ffi::gst_rtsp_address_pool_reserve_address(
                self.as_ref().to_glib_none().0,
                ip_address.to_glib_none().0,
                port,
                n_ports,
                ttl,
                &mut address,
            ));
            match ret {
                RTSPAddressPoolResult::Ok => Ok(from_glib_full(address)),
                _ => Err(ret),
            }
        }
    }
}

impl<O: IsA<RTSPAddressPool>> RTSPAddressPoolExtManual for O {}
