// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{ffi, RTSPMediaFactory};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPMediaFactory>> Sealed for T {}
}

pub trait RTSPMediaFactoryExtManual: sealed::Sealed + IsA<RTSPMediaFactory> + 'static {
    #[doc(alias = "gst_rtsp_media_factory_add_role_from_structure")]
    fn add_role_from_structure(&self, structure: &gst::StructureRef) {
        unsafe {
            ffi::gst_rtsp_media_factory_add_role_from_structure(
                self.as_ref().to_glib_none().0,
                structure.as_mut_ptr(),
            );
        }
    }
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {}
