// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod formatter;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;
    pub use gst::subclass::prelude::*;

    pub use super::formatter::{FormatterImpl, FormatterImplExt};
}
