// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use windows::{
    core::Interface,
    Win32::Graphics::Direct3D12::{ID3D12Device, D3D12_DESCRIPTOR_HEAP_DESC},
};

use crate::{ffi, D3D12DescHeapPool};

impl D3D12DescHeapPool {
    #[doc(alias = "gst_d3d12_desc_heap_pool_new")]
    pub fn new(device: &ID3D12Device, desc: &D3D12_DESCRIPTOR_HEAP_DESC) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let desc_c_ptr = desc as *const D3D12_DESCRIPTOR_HEAP_DESC as *const std::ffi::c_void;
            from_glib_full(ffi::gst_d3d12_desc_heap_pool_new(
                device.as_raw(),
                desc_c_ptr,
            ))
        }
    }
}
