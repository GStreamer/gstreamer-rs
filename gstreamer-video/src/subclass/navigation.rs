// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ffi, Navigation};

pub trait NavigationImpl: ObjectImpl {
    fn send_event(&self, structure: gst::Structure);

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    fn send_event_simple(&self, event: gst::Event) {
        if let Some(structure) = event.structure() {
            self.send_event(structure.to_owned());
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::NavigationImplExt> Sealed for T {}
}

pub trait NavigationImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_send_event(&self, structure: gst::Structure) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Navigation>()
                as *const ffi::GstNavigationInterface;

            let func = match (*parent_iface).send_event {
                Some(func) => func,
                None => return,
            };

            func(
                self.obj().unsafe_cast_ref::<Navigation>().to_glib_none().0,
                structure.into_glib_ptr(),
            );
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    fn parent_send_event_simple(&self, event: gst::Event) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<Navigation>()
                as *const ffi::GstNavigationInterface;

            let func = match (*parent_iface).send_event_simple {
                Some(func) => func,
                None => return,
            };

            func(
                self.obj().unsafe_cast_ref::<Navigation>().to_glib_none().0,
                event.into_glib_ptr(),
            );
        }
    }
}

impl<T: NavigationImpl> NavigationImplExt for T {}

unsafe impl<T: NavigationImpl> IsImplementable<T> for Navigation {
    #[cfg(not(any(feature = "v1_22", docsrs)))]
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.send_event = Some(navigation_send_event::<T>);
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.send_event = Some(navigation_send_event::<T>);
        iface.send_event_simple = Some(navigation_send_event_simple::<T>);
    }
}

unsafe extern "C" fn navigation_send_event<T: NavigationImpl>(
    nav: *mut ffi::GstNavigation,
    structure: *mut gst::ffi::GstStructure,
) {
    let instance = &*(nav as *mut T::Instance);
    let imp = instance.imp();

    imp.send_event(from_glib_full(structure));
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
unsafe extern "C" fn navigation_send_event_simple<T: NavigationImpl>(
    nav: *mut ffi::GstNavigation,
    event: *mut gst::ffi::GstEvent,
) {
    let instance = &*(nav as *mut T::Instance);
    let imp = instance.imp();

    imp.send_event_simple(from_glib_full(event));
}
