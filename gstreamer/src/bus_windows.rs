// Take a look at the license at the top of the repository in the LICENSE file.

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        use glib::translate::ToGlibPtr;

        use std::mem;
        use std::os::windows;
    } else if #[cfg(docsrs)] {
        // Declare a fake RawHandle for doc generation on unix
        pub mod windows {
            pub mod io {
                pub struct RawHandle{}
            }
        }
    }
}

use super::Bus;

pub trait WindowsBusExtManual: 'static {
    #[doc(alias = "get_pollfd")]
    #[doc(alias = "gst_bus_get_pollfd")]
    fn pollfd(&self) -> windows::io::RawHandle;
}

impl WindowsBusExtManual for Bus {
    fn pollfd(&self) -> windows::io::RawHandle {
        #[cfg(windows)]
        unsafe {
            let mut pollfd = mem::MaybeUninit::uninit();
            crate::ffi::gst_bus_get_pollfd(self.to_glib_none().0, pollfd.as_mut_ptr());
            let pollfd = pollfd.assume_init();
            pollfd.fd as *mut _
        }

        #[cfg(all(not(windows), docsrs))]
        windows::io::RawHandle {}
    }
}
