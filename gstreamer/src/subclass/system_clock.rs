// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::subclass::prelude::*;

use crate::SystemClock;

pub trait SystemClockImpl: ClockImpl {}

unsafe impl<T: SystemClockImpl> IsSubclassable<T> for SystemClock {
    fn class_init(klass: &mut glib::Class<Self>) {
        <crate::Clock as IsSubclassable<T>>::class_init(klass);
        let _klass = klass.as_mut();
        // Nothing to do here
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <crate::Clock as IsSubclassable<T>>::instance_init(instance);
    }
}
