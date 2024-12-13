// Take a look at the license at the top of the repository in the LICENSE file.

use std::{borrow::Cow, ffi::CStr, fmt, ptr};

use glib::{ffi::gpointer, prelude::*, translate::*};
use libc::c_char;
#[cfg(feature = "log")]
use log;
use once_cell::sync::Lazy;

use crate::{ffi, DebugLevel};

// import and rename those so they are namespaced as log::*
pub use crate::auto::functions::debug_add_ring_buffer_logger as add_ring_buffer_logger;
pub use crate::auto::functions::debug_get_default_threshold as get_default_threshold;
pub use crate::auto::functions::debug_get_stack_trace as get_stack_trace;
pub use crate::auto::functions::debug_is_active as is_active;
pub use crate::auto::functions::debug_is_colored as is_colored;
pub use crate::auto::functions::debug_print_stack_trace as print_stack_trace;
pub use crate::auto::functions::debug_remove_ring_buffer_logger as remove_ring_buffer_logger;
pub use crate::auto::functions::debug_ring_buffer_logger_get_logs as ring_buffer_logger_get_logs;
pub use crate::auto::functions::debug_set_active as set_active;
pub use crate::auto::functions::debug_set_colored as set_colored;
pub use crate::auto::functions::debug_set_default_threshold as set_default_threshold;
pub use crate::auto::functions::debug_set_threshold_for_name as set_threshold_for_name;
pub use crate::auto::functions::debug_set_threshold_from_string as set_threshold_from_string;
pub use crate::auto::functions::debug_unset_threshold_for_name as unset_threshold_for_name;

#[derive(PartialEq, Eq)]
#[doc(alias = "GstDebugMessage")]
pub struct DebugMessage(ptr::NonNull<ffi::GstDebugMessage>);

impl fmt::Debug for DebugMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugMessage").field(&self.get()).finish()
    }
}

impl DebugMessage {
    #[doc(alias = "gst_debug_message_get")]
    #[inline]
    pub fn get(&self) -> Option<Cow<glib::GStr>> {
        unsafe {
            let message = ffi::gst_debug_message_get(self.0.as_ptr());

            if message.is_null() {
                None
            } else {
                Some(glib::GStr::from_ptr_lossy(message))
            }
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_debug_message_get_id")]
    #[inline]
    pub fn id(&self) -> Option<&glib::GStr> {
        unsafe {
            let id = ffi::gst_debug_message_get_id(self.0.as_ptr());

            if id.is_null() {
                None
            } else {
                Some(glib::GStr::from_ptr(id))
            }
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut ffi::GstDebugMessage {
        self.0.as_ptr()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
#[doc(alias = "GstDebugCategory")]
#[repr(transparent)]
pub struct DebugCategory(Option<ptr::NonNull<ffi::GstDebugCategory>>);

impl DebugCategory {
    #[doc(alias = "gst_debug_category_new")]
    #[doc(alias = "GST_DEBUG_CATEGORY")]
    #[doc(alias = "GST_DEBUG_CATEGORY_INIT")]
    pub fn new(
        name: &str,
        color: crate::DebugColorFlags,
        description: Option<&str>,
    ) -> DebugCategory {
        skip_assert_initialized!();
        extern "C" {
            fn _gst_debug_category_new(
                name: *const c_char,
                color: ffi::GstDebugColorFlags,
                description: *const c_char,
            ) -> *mut ffi::GstDebugCategory;
        }

        // Gets the category if it exists already
        unsafe {
            let ptr = name.run_with_gstr(|name| {
                description.run_with_gstr(|description| {
                    _gst_debug_category_new(
                        name.to_glib_none().0,
                        color.into_glib(),
                        description.to_glib_none().0,
                    )
                })
            });

            // Can be NULL if the debug system is compiled out
            DebugCategory(ptr::NonNull::new(ptr))
        }
    }

    #[doc(alias = "gst_debug_get_category")]
    #[inline]
    pub fn get(name: &str) -> Option<DebugCategory> {
        skip_assert_initialized!();
        unsafe {
            extern "C" {
                fn _gst_debug_get_category(name: *const c_char) -> *mut ffi::GstDebugCategory;
            }

            let cat = name.run_with_gstr(|name| _gst_debug_get_category(name.to_glib_none().0));

            if cat.is_null() {
                None
            } else {
                Some(DebugCategory(Some(ptr::NonNull::new_unchecked(cat))))
            }
        }
    }

    #[doc(alias = "get_threshold")]
    #[doc(alias = "gst_debug_category_get_threshold")]
    #[inline]
    pub fn threshold(self) -> crate::DebugLevel {
        match self.0 {
            Some(cat) => unsafe { from_glib(cat.as_ref().threshold) },
            None => crate::DebugLevel::None,
        }
    }

    #[doc(alias = "gst_debug_category_set_threshold")]
    #[inline]
    pub fn set_threshold(self, threshold: crate::DebugLevel) {
        if let Some(cat) = self.0 {
            unsafe { ffi::gst_debug_category_set_threshold(cat.as_ptr(), threshold.into_glib()) }
        }
    }

    #[doc(alias = "gst_debug_category_reset_threshold")]
    #[inline]
    pub fn reset_threshold(self) {
        if let Some(cat) = self.0 {
            unsafe { ffi::gst_debug_category_reset_threshold(cat.as_ptr()) }
        }
    }

    #[inline]
    pub fn above_threshold(self, level: crate::DebugLevel) -> bool {
        match self.0 {
            Some(cat) => unsafe { cat.as_ref().threshold >= level.into_glib() },
            None => false,
        }
    }

    #[doc(alias = "get_color")]
    #[doc(alias = "gst_debug_category_get_color")]
    #[inline]
    pub fn color(self) -> crate::DebugColorFlags {
        match self.0 {
            Some(cat) => unsafe { from_glib(cat.as_ref().color) },
            None => crate::DebugColorFlags::empty(),
        }
    }

    #[doc(alias = "get_name")]
    #[doc(alias = "gst_debug_category_get_name")]
    #[inline]
    pub fn name<'a>(self) -> &'a str {
        match self.0 {
            Some(cat) => unsafe { CStr::from_ptr(cat.as_ref().name).to_str().unwrap() },
            None => "",
        }
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_debug_category_get_description")]
    #[inline]
    pub fn description<'a>(self) -> Option<&'a str> {
        let cat = self.0?;

        unsafe {
            let ptr = cat.as_ref().description;

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[inline]
    #[doc(alias = "gst_debug_log")]
    #[doc(alias = "gst_debug_log_literal")]
    pub fn log(
        self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            args,
        )
    }

    #[inline]
    #[doc(alias = "gst_debug_log_literal")]
    pub fn log_literal(
        self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_literal_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            msg,
        )
    }

    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log")]
    pub fn log_unfiltered(
        self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        self.log_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            args,
        )
    }

    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log_literal")]
    pub fn log_literal_unfiltered(
        self,
        obj: Option<&impl IsA<glib::Object>>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        self.log_literal_unfiltered_internal(
            obj.map(|obj| obj.as_ref()),
            level,
            file,
            function,
            line,
            msg,
        )
    }

    #[inline(never)]
    fn log_unfiltered_internal(
        self,
        obj: Option<&glib::Object>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        let mut w = smallvec::SmallVec::<[u8; 256]>::new();

        // Can't really happen but better safe than sorry
        if std::io::Write::write_fmt(&mut w, args).is_err() {
            return;
        }
        w.push(0);

        self.log_literal_unfiltered_internal(obj, level, file, function, line, unsafe {
            glib::GStr::from_utf8_with_nul_unchecked(&w)
        });
    }

    #[inline(never)]
    fn log_literal_unfiltered_internal(
        self,
        obj: Option<&glib::Object>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        let obj_ptr = match obj {
            Some(obj) => obj.as_ptr(),
            None => ptr::null_mut(),
        };

        function.run_with_gstr(|function| {
            #[cfg(feature = "v1_20")]
            unsafe {
                ffi::gst_debug_log_literal(
                    cat.as_ptr(),
                    level.into_glib(),
                    file.as_ptr(),
                    function.as_ptr(),
                    line as i32,
                    obj_ptr,
                    msg.as_ptr(),
                );
            }
            #[cfg(not(feature = "v1_20"))]
            unsafe {
                ffi::gst_debug_log(
                    cat.as_ptr(),
                    level.into_glib(),
                    file.as_ptr(),
                    function.as_ptr(),
                    line as i32,
                    obj_ptr,
                    b"%s\0".as_ptr() as *const _,
                    msg.as_ptr(),
                );
            }
        });
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[inline]
    #[doc(alias = "gst_debug_log_id")]
    pub fn log_id(
        self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_id_unfiltered_internal(id.as_ref(), level, file, function, line, args);
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[inline]
    #[doc(alias = "gst_debug_log_id_literal")]
    pub fn log_id_literal(
        self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        if !self.above_threshold(level) {
            return;
        }

        self.log_id_literal_unfiltered_internal(id.as_ref(), level, file, function, line, msg);
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log_id")]
    pub fn log_id_unfiltered(
        self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        self.log_id_unfiltered_internal(id.as_ref(), level, file, function, line, args)
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    // rustdoc-stripper-ignore-next
    /// Logs without checking the log level.
    #[inline]
    #[doc(alias = "gst_debug_log_id_literal")]
    pub fn log_id_literal_unfiltered(
        self,
        id: impl AsRef<glib::GStr>,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        self.log_id_literal_unfiltered_internal(id.as_ref(), level, file, function, line, msg)
    }

    #[cfg(feature = "v1_22")]
    #[inline(never)]
    fn log_id_unfiltered_internal(
        self,
        id: &glib::GStr,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        args: fmt::Arguments,
    ) {
        let mut w = smallvec::SmallVec::<[u8; 256]>::new();

        // Can't really happen but better safe than sorry
        if std::io::Write::write_fmt(&mut w, args).is_err() {
            return;
        }
        w.push(0);

        self.log_id_literal_unfiltered_internal(id, level, file, function, line, unsafe {
            glib::GStr::from_utf8_with_nul_unchecked(&w)
        });
    }

    #[cfg(feature = "v1_22")]
    #[inline(never)]
    fn log_id_literal_unfiltered_internal(
        self,
        id: &glib::GStr,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &str,
        line: u32,
        msg: &glib::GStr,
    ) {
        let cat = match self.0 {
            Some(cat) => cat,
            None => return,
        };

        function.run_with_gstr(|function| unsafe {
            ffi::gst_debug_log_id_literal(
                cat.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                function.as_ptr(),
                line as i32,
                id.as_ptr(),
                msg.as_ptr(),
            );
        });
    }

    #[doc(alias = "get_all_categories")]
    #[doc(alias = "gst_debug_get_all_categories")]
    #[inline]
    pub fn all_categories() -> glib::SList<DebugCategory> {
        unsafe { glib::SList::from_glib_container(ffi::gst_debug_get_all_categories()) }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_debug_log_get_line")]
    #[inline]
    pub fn get_line(
        &self,
        level: crate::DebugLevel,
        file: &glib::GStr,
        function: &glib::GStr,
        line: u32,
        object: Option<&LoggedObject>,
        message: &DebugMessage,
    ) -> Option<glib::GString> {
        let cat = self.0?;

        unsafe {
            from_glib_full(ffi::gst_debug_log_get_line(
                cat.as_ptr(),
                level.into_glib(),
                file.as_ptr(),
                function.as_ptr(),
                line as i32,
                object.map(|o| o.as_ptr()).unwrap_or(ptr::null_mut()),
                message.0.as_ptr(),
            ))
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut ffi::GstDebugCategory {
        self.0.map(|p| p.as_ptr()).unwrap_or(ptr::null_mut())
    }
}

unsafe impl Sync for DebugCategory {}
unsafe impl Send for DebugCategory {}

impl fmt::Debug for DebugCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("DebugCategory").field(&self.name()).finish()
    }
}

impl GlibPtrDefault for DebugCategory {
    type GlibType = *mut ffi::GstDebugCategory;
}

unsafe impl TransparentPtrType for DebugCategory {}

impl FromGlibPtrNone<*mut ffi::GstDebugCategory> for DebugCategory {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstDebugCategory) -> Self {
        debug_assert!(!ptr.is_null());
        DebugCategory(Some(ptr::NonNull::new_unchecked(ptr)))
    }
}

impl FromGlibPtrFull<*mut ffi::GstDebugCategory> for DebugCategory {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstDebugCategory) -> Self {
        debug_assert!(!ptr.is_null());
        DebugCategory(Some(ptr::NonNull::new_unchecked(ptr)))
    }
}

pub static CAT_RUST: Lazy<DebugCategory> = Lazy::new(|| {
    DebugCategory::new(
        "GST_RUST",
        crate::DebugColorFlags::UNDERLINE,
        Some("GStreamer's Rust binding core"),
    )
});

macro_rules! declare_debug_category_from_name(
    ($cat:ident, $cat_name:expr) => (
        pub static $cat: Lazy<DebugCategory> = Lazy::new(|| DebugCategory::get($cat_name)
            .expect(&format!("Unable to find `DebugCategory` with name {}", $cat_name)));
    );
);

declare_debug_category_from_name!(CAT_DEFAULT, "default");
declare_debug_category_from_name!(CAT_GST_INIT, "GST_INIT");
declare_debug_category_from_name!(CAT_MEMORY, "GST_MEMORY");
declare_debug_category_from_name!(CAT_PARENTAGE, "GST_PARENTAGE");
declare_debug_category_from_name!(CAT_STATES, "GST_STATES");
declare_debug_category_from_name!(CAT_SCHEDULING, "GST_SCHEDULING");
declare_debug_category_from_name!(CAT_BUFFER, "GST_BUFFER");
declare_debug_category_from_name!(CAT_BUFFER_LIST, "GST_BUFFER_LIST");
declare_debug_category_from_name!(CAT_BUS, "GST_BUS");
declare_debug_category_from_name!(CAT_CAPS, "GST_CAPS");
declare_debug_category_from_name!(CAT_CLOCK, "GST_CLOCK");
declare_debug_category_from_name!(CAT_ELEMENT_PADS, "GST_ELEMENT_PADS");
declare_debug_category_from_name!(CAT_PADS, "GST_PADS");
declare_debug_category_from_name!(CAT_PERFORMANCE, "GST_PERFORMANCE");
declare_debug_category_from_name!(CAT_PIPELINE, "GST_PIPELINE");
declare_debug_category_from_name!(CAT_PLUGIN_LOADING, "GST_PLUGIN_LOADING");
declare_debug_category_from_name!(CAT_PLUGIN_INFO, "GST_PLUGIN_INFO");
declare_debug_category_from_name!(CAT_PROPERTIES, "GST_PROPERTIES");
declare_debug_category_from_name!(CAT_NEGOTIATION, "GST_NEGOTIATION");
declare_debug_category_from_name!(CAT_REFCOUNTING, "GST_REFCOUNTING");
declare_debug_category_from_name!(CAT_ERROR_SYSTEM, "GST_ERROR_SYSTEM");
declare_debug_category_from_name!(CAT_EVENT, "GST_EVENT");
declare_debug_category_from_name!(CAT_MESSAGE, "GST_MESSAGE");
declare_debug_category_from_name!(CAT_PARAMS, "GST_PARAMS");
declare_debug_category_from_name!(CAT_CALL_TRACE, "GST_CALL_TRACE");
declare_debug_category_from_name!(CAT_SIGNAL, "GST_SIGNAL");
declare_debug_category_from_name!(CAT_PROBE, "GST_PROBE");
declare_debug_category_from_name!(CAT_REGISTRY, "GST_REGISTRY");
declare_debug_category_from_name!(CAT_QOS, "GST_QOS");
declare_debug_category_from_name!(CAT_META, "GST_META");
declare_debug_category_from_name!(CAT_LOCKING, "GST_LOCKING");
declare_debug_category_from_name!(CAT_CONTEXT, "GST_CONTEXT");

#[macro_export]
macro_rules! error(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! error(
                () => {}
            );
            error!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! error(
                () => {}
            );
            error!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! error(
                () => {}
            );
            error!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Error, $($args)*)
    }};
);

#[macro_export]
macro_rules! warning(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! warning(
                () => {}
            );
            warning!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! warning(
                () => {}
            );
            warning!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! warning(
                () => {}
            );
            warning!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Warning, $($args)*)
    }};
);

#[macro_export]
macro_rules! fixme(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! fixme(
                () => {}
            );
            fixme!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! fixme(
                () => {}
            );
            fixme!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! fixme(
                () => {}
            );
            fixme!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Fixme, $($args)*)
    }};
);

#[macro_export]
macro_rules! info(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! info(
                () => {}
            );
            info!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! info(
                () => {}
            );
            info!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! info(
                () => {}
            );
            info!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Info, $($args)*)
    }};
);

#[macro_export]
macro_rules! debug(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! debug(
                () => {}
            );
            debug!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! debug(
                () => {}
            );
            debug!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! debug(
                () => {}
            );
            debug!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Debug, $($args)*)
    }};
);

#[macro_export]
macro_rules! log(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! log(
                () => {}
            );
            log!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! log(
                () => {}
            );
            log!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! log(
                () => {}
            );
            log!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Log, $($args)*)
    }};
);

#[macro_export]
macro_rules! trace(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! trace(
                () => {}
            );
            trace!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! trace(
                () => {}
            );
            trace!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: `"]
            macro_rules! trace(
                () => {}
            );
            trace!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Trace, $($args)*)
    }};
);

#[macro_export]
macro_rules! memdump(
    ($cat:expr, obj = $obj:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp = $imp:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, imp = $imp, $($args)*)
    }};
    ($cat:expr, id = $id:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, id = $id, $($args)*)
    }};

    ($cat:expr, obj: $obj:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style obj format. Use `obj = ` instead of `obj: ` for better tooling support"]
            macro_rules! memdump(
                () => {}
            );
            memdump!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, obj = $obj, $($args)*)
    }};
    ($cat:expr, imp: $imp:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style imp format. Use `imp = ` instead of `imp: ` for better tooling support"]
            macro_rules! memdump(
                () => {}
            );
            memdump!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, imp = $imp, $($args)*)
    }};
    ($cat:expr, id: $id:expr, $($args:tt)*) => { {
        {
            #[deprecated = "Using old-style id format. Use `id = ` instead of `id: ` for better tooling support"]
            macro_rules! memdump(
                () => {}
            );
            memdump!();
        }
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, id = $id, $($args)*)
    }};
    ($cat:expr, $($args:tt)*) => { {
        $crate::log_with_level!($cat, $crate::DebugLevel::Memdump, $($args)*)
    }};
);

#[macro_export]
macro_rules! log_with_level(
    ($cat:expr, $level:expr, obj = $obj:expr, $msg:literal) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        #[allow(clippy::redundant_closure_call)]
        if cat.above_threshold($level) {
            use $crate::glib::prelude::Cast;

            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let obj = &$obj;
            let obj = unsafe { obj.unsafe_cast_ref::<$crate::glib::Object>() };
            let function_name = $crate::glib::function_name!();

            // Check if formatting is necessary or not
            // FIXME: This needs to be a closure because the return value of format_args!() can't
            // be assigned to a variable
            (|args: std::fmt::Arguments| {
                if args.as_str().is_some() {
                    $crate::DebugCategory::log_literal_unfiltered(
                        cat,
                        Some(obj),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        $crate::glib::gstr!($msg),
                    )
                } else {
                    $crate::DebugCategory::log_unfiltered(
                        cat,
                        Some(obj),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        args,
                    )
                }
            })(format_args!($msg))
        }
    }};
    ($cat:expr, $level:expr, obj = $obj:expr, $($args:tt)*) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if cat.above_threshold($level) {
            use $crate::glib::prelude::Cast;

            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let obj = &$obj;
            let obj = unsafe { obj.unsafe_cast_ref::<$crate::glib::Object>() };
            $crate::DebugCategory::log_unfiltered(
                cat,
                Some(obj),
                $level,
                unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                $crate::glib::function_name!(),
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, $level:expr, imp = $imp:expr, $msg:literal) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        #[allow(clippy::redundant_closure_call)]
        if cat.above_threshold($level) {
            use $crate::glib::prelude::Cast;

            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let obj = $imp.obj();
            let obj = unsafe { obj.unsafe_cast_ref::<$crate::glib::Object>() };
            let function_name = $crate::glib::function_name!();

            // Check if formatting is necessary or not
            // FIXME: This needs to be a closure because the return value of format_args!() can't
            // be assigned to a variable
            (|args: std::fmt::Arguments| {
                if args.as_str().is_some() {
                    $crate::DebugCategory::log_literal_unfiltered(
                        cat,
                        Some(obj),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        $crate::glib::gstr!($msg),
                    )
                } else {
                    $crate::DebugCategory::log_unfiltered(
                        cat,
                        Some(obj),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        args,
                    )
                }
            })(format_args!($msg))
        }
    }};
    ($cat:expr, $level:expr, imp = $imp:expr, $($args:tt)*) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if cat.above_threshold($level) {
            use $crate::glib::prelude::Cast;

            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let obj = $imp.obj();
            let obj = unsafe { obj.unsafe_cast_ref::<$crate::glib::Object>() };
            $crate::DebugCategory::log_unfiltered(
                cat,
                Some(obj),
                $level,
                unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                $crate::glib::function_name!(),
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, $level:expr, id = $id:literal, $msg:literal) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        #[allow(clippy::redundant_closure_call)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let function_name = $crate::glib::function_name!();

            // Check if formatting is necessary or not
            // FIXME: This needs to be a closure because the return value of format_args!() can't
            // be assigned to a variable
            (|args: std::fmt::Arguments| {
                if args.as_str().is_some() {
                    $crate::DebugCategory::log_id_literal_unfiltered(
                        cat,
                        $crate::glib::gstr!($id),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        $crate::glib::gstr!($msg),
                    )
                } else {
                    $crate::DebugCategory::log_id_unfiltered(
                        cat,
                        $crate::glib::gstr!($id),
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        args,
                    )
                }
            })(format_args!($msg))
        }
    }};
    ($cat:expr, $level:expr, id = $id:literal, $($args:tt)*) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            $crate::DebugCategory::log_id_unfiltered(
                cat,
                $crate::glib::gstr!($id),
                $level,
                unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                $crate::glib::function_name!(),
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, $level:expr, id = $id:expr, $msg:literal) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        #[allow(clippy::redundant_closure_call)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let function_name = $crate::glib::function_name!();

            // Check if formatting is necessary or not
            // FIXME: This needs to be a closure because the return value of format_args!() can't
            // be assigned to a variable
            (|args: std::fmt::Arguments| {
                if args.as_str().is_some() {
                    $crate::DebugCategory::log_id_literal_unfiltered(
                        cat,
                        $id,
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        $crate::glib::gstr!($msg),
                    )
                } else {
                    $crate::DebugCategory::log_id_unfiltered(
                        cat,
                        $id,
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        args,
                    )
                }
            })(format_args!($msg))
        }
    }};
    ($cat:expr, $level:expr, id = $id:expr, $($args:tt)*) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            $crate::DebugCategory::log_id_unfiltered(
                cat,
                $id,
                $level,
                unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                $crate::glib::function_name!(),
                line!(),
                format_args!($($args)*),
            )
        }
    }};
    ($cat:expr, $level:expr, $msg:literal) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        #[allow(clippy::redundant_closure_call)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            let function_name = $crate::glib::function_name!();

            // Check if formatting is necessary or not
            // FIXME: This needs to be a closure because the return value of format_args!() can't
            // be assigned to a variable
            (|args: std::fmt::Arguments| {
                if args.as_str().is_some() {
                    $crate::DebugCategory::log_literal_unfiltered(
                        cat,
                        None as Option<&$crate::glib::Object>,
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        $crate::glib::gstr!($msg),
                    )
                } else {
                    $crate::DebugCategory::log_unfiltered(
                        cat,
                        None as Option<&$crate::glib::Object>,
                        $level,
                        unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                        function_name,
                        line!(),
                        args,
                    )
                }
            })(format_args!($msg))
        }
    }};
    ($cat:expr, $level:expr, $($args:tt)*) => { {
        let cat = $cat.clone();

        // Check the log level before using `format_args!` otherwise
        // formatted arguments are evaluated even if we end up not logging.
        #[allow(unused_unsafe)]
        if cat.above_threshold($level) {
            // FIXME: Once there's a function_name! macro that returns a string literal we can
            // directly pass it as `&GStr` forward

            $crate::DebugCategory::log_unfiltered(
                cat,
                None as Option<&$crate::glib::Object>,
                $level,
                unsafe { $crate::glib::GStr::from_utf8_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
                $crate::glib::function_name!(),
                line!(),
                format_args!($($args)*),
            )
        }
    }};
);

#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
#[derive(Debug)]
pub struct DebugCategoryLogger(DebugCategory);

#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
impl DebugCategoryLogger {
    pub fn new(cat: DebugCategory) -> Self {
        skip_assert_initialized!();
        Self(cat)
    }

    fn to_level(level: log::Level) -> crate::DebugLevel {
        skip_assert_initialized!();
        match level {
            log::Level::Error => DebugLevel::Error,
            log::Level::Warn => DebugLevel::Warning,
            log::Level::Info => DebugLevel::Info,
            log::Level::Debug => DebugLevel::Debug,
            log::Level::Trace => DebugLevel::Trace,
        }
    }
}

#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
impl log::Log for DebugCategoryLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.0.above_threshold(Self::to_level(metadata.level()))
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        record.file().unwrap_or("").run_with_gstr(|file| {
            self.0.log(
                None::<&glib::Object>,
                Self::to_level(record.level()),
                file,
                record.module_path().unwrap_or(""),
                record.line().unwrap_or(0),
                *record.args(),
            );
        });
    }

    fn flush(&self) {}
}

unsafe extern "C" fn log_handler<T>(
    category: *mut ffi::GstDebugCategory,
    level: ffi::GstDebugLevel,
    file: *const c_char,
    function: *const c_char,
    line: i32,
    object: *mut glib::gobject_ffi::GObject,
    message: *mut ffi::GstDebugMessage,
    user_data: gpointer,
) where
    T: Fn(
            DebugCategory,
            DebugLevel,
            &glib::GStr,
            &glib::GStr,
            u32,
            Option<&LoggedObject>,
            &DebugMessage,
        ) + Send
        + Sync
        + 'static,
{
    if category.is_null() {
        return;
    }
    let category = DebugCategory(Some(ptr::NonNull::new_unchecked(category)));
    let level = from_glib(level);
    let file = glib::GStr::from_ptr(file);
    let function = glib::GStr::from_ptr(function);
    let line = line as u32;
    let object = ptr::NonNull::new(object).map(LoggedObject);
    let message = DebugMessage(ptr::NonNull::new_unchecked(message));
    let handler = &*(user_data as *mut T);
    (handler)(
        category,
        level,
        file,
        function,
        line,
        object.as_ref(),
        &message,
    );
}

unsafe extern "C" fn log_handler_data_free<T>(data: gpointer) {
    let data = Box::from_raw(data as *mut T);
    drop(data);
}

#[derive(Debug)]
pub struct DebugLogFunction(ptr::NonNull<std::os::raw::c_void>);

// The contained pointer is never dereferenced and has no thread affinity.
// It may be convenient to send it or share it between threads to allow cleaning
// up log functions from other threads than the one that created it.
unsafe impl Send for DebugLogFunction {}
unsafe impl Sync for DebugLogFunction {}

#[derive(Debug)]
#[doc(alias = "GObject")]
pub struct LoggedObject(ptr::NonNull<glib::gobject_ffi::GObject>);

impl LoggedObject {
    #[inline]
    pub fn as_ptr(&self) -> *mut glib::gobject_ffi::GObject {
        self.0.as_ptr()
    }
}

impl fmt::Display for LoggedObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let ptr = self.0.as_ptr();
            let g_type_instance = &mut (*ptr).g_type_instance;
            if glib::gobject_ffi::g_type_check_instance_is_fundamentally_a(
                g_type_instance,
                glib::gobject_ffi::g_object_get_type(),
            ) != glib::ffi::GFALSE
            {
                let type_ = (*g_type_instance.g_class).g_type;

                if glib::gobject_ffi::g_type_is_a(type_, ffi::gst_pad_get_type())
                    != glib::ffi::GFALSE
                {
                    let name_ptr = (*(ptr as *mut ffi::GstObject)).name;
                    let name = if name_ptr.is_null() {
                        "<null>"
                    } else {
                        CStr::from_ptr(name_ptr)
                            .to_str()
                            .unwrap_or("<invalid name>")
                    };

                    let parent_ptr = (*(ptr as *mut ffi::GstObject)).parent;
                    let parent_name = if parent_ptr.is_null() {
                        "<null>"
                    } else {
                        let name_ptr = (*(parent_ptr)).name;
                        if name_ptr.is_null() {
                            "<null>"
                        } else {
                            CStr::from_ptr(name_ptr)
                                .to_str()
                                .unwrap_or("<invalid name>")
                        }
                    };

                    write!(f, "{parent_name}:{name}")
                } else if glib::gobject_ffi::g_type_is_a(type_, ffi::gst_object_get_type())
                    != glib::ffi::GFALSE
                {
                    let name_ptr = (*(ptr as *mut ffi::GstObject)).name;
                    let name = if name_ptr.is_null() {
                        "<null>"
                    } else {
                        CStr::from_ptr(name_ptr)
                            .to_str()
                            .unwrap_or("<invalid name>")
                    };
                    write!(f, "{name}")
                } else {
                    let type_name = CStr::from_ptr(glib::gobject_ffi::g_type_name(type_));
                    write!(
                        f,
                        "{}:{:?}",
                        type_name.to_str().unwrap_or("<invalid type>"),
                        ptr
                    )
                }
            } else {
                write!(f, "{ptr:?}")
            }
        }
    }
}

#[doc(alias = "gst_debug_add_log_function")]
pub fn add_log_function<T>(function: T) -> DebugLogFunction
where
    T: Fn(
            DebugCategory,
            DebugLevel,
            &glib::GStr,
            &glib::GStr,
            u32,
            Option<&LoggedObject>,
            &DebugMessage,
        ) + Send
        + Sync
        + 'static,
{
    skip_assert_initialized!();
    unsafe {
        let user_data = Box::new(function);
        let user_data_ptr = Box::into_raw(user_data) as gpointer;
        ffi::gst_debug_add_log_function(
            Some(log_handler::<T>),
            user_data_ptr,
            Some(log_handler_data_free::<T>),
        );
        DebugLogFunction(ptr::NonNull::new_unchecked(user_data_ptr))
    }
}

pub fn remove_default_log_function() {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_remove_log_function(None);
    }
}

#[doc(alias = "gst_debug_remove_log_function_by_data")]
pub fn remove_log_function(log_fn: DebugLogFunction) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_remove_log_function_by_data(log_fn.0.as_ptr());
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{mpsc, Arc, Mutex};

    use super::*;

    #[test]
    #[doc(alias = "get_existing")]
    fn existing() {
        crate::init().unwrap();

        let perf_cat = DebugCategory::get("GST_PERFORMANCE")
            .expect("Unable to find `DebugCategory` with name \"GST_PERFORMANCE\"");
        assert_eq!(perf_cat.name(), CAT_PERFORMANCE.name());
    }

    #[test]
    fn all() {
        crate::init().unwrap();

        assert!(DebugCategory::all_categories()
            .iter()
            .any(|c| c.name() == "GST_PERFORMANCE"));
    }

    #[test]
    fn new_and_log() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat",
            crate::DebugColorFlags::empty(),
            Some("some debug category"),
        );

        error!(cat, "meh");
        warning!(cat, "meh");
        fixme!(cat, "meh");
        info!(cat, "meh");
        debug!(cat, "meh");
        log!(cat, "meh");
        trace!(cat, "meh");
        memdump!(cat, "meh");

        let obj = crate::Bin::with_name("meh");

        error!(cat, obj = &obj, "meh");
        warning!(cat, obj = &obj, "meh");
        fixme!(cat, obj = &obj, "meh");
        info!(cat, obj = &obj, "meh");
        debug!(cat, obj = &obj, "meh");
        log!(cat, obj = &obj, "meh");
        trace!(cat, obj = &obj, "meh");
        memdump!(cat, obj = &obj, "meh");

        error!(cat, obj = obj, "meh");
        warning!(cat, obj = obj, "meh");
        fixme!(cat, obj = obj, "meh");
        info!(cat, obj = obj, "meh");
        debug!(cat, obj = obj, "meh");
        log!(cat, obj = obj, "meh");
        trace!(cat, obj = obj, "meh");
        memdump!(cat, obj = obj, "meh");
    }

    #[cfg(feature = "log")]
    static LOGGER: Lazy<DebugCategoryLogger> = Lazy::new(|| {
        DebugCategoryLogger::new(DebugCategory::new(
            "Log_trait",
            crate::DebugColorFlags::empty(),
            Some("Using the Log trait"),
        ))
    });

    #[test]
    #[cfg(feature = "log")]
    fn log_trait() {
        crate::init().unwrap();

        log::set_logger(&(*LOGGER)).expect("Failed to set logger");
        log::set_max_level(log::LevelFilter::Trace);
        log::error!("meh");
        log::warn!("fish");

        let (sender, receiver) = mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let handler = move |category: DebugCategory,
                            level: DebugLevel,
                            _file: &glib::GStr,
                            _function: &glib::GStr,
                            _line: u32,
                            _object: Option<&LoggedObject>,
                            message: &DebugMessage| {
            let cat = DebugCategory::get("Log_trait").unwrap();

            if category != cat {
                // This test can run in parallel with other tests, including new_and_log above.
                // We cannot be certain we only see our own messages.
                return;
            }

            assert_eq!(level, DebugLevel::Error);
            assert_eq!(message.get().unwrap().as_ref(), "meh");
            let _ = sender.lock().unwrap().send(());
        };

        remove_default_log_function();
        add_log_function(handler);

        let cat = LOGGER.0;

        cat.set_threshold(crate::DebugLevel::Warning);
        log::error!("meh");
        receiver.recv().unwrap();

        cat.set_threshold(crate::DebugLevel::Error);
        log::error!("meh");
        receiver.recv().unwrap();

        cat.set_threshold(crate::DebugLevel::None);
        log::error!("fish");
        log::warn!("meh");
    }

    #[test]
    fn log_handler() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "test-cat-log",
            crate::DebugColorFlags::empty(),
            Some("some debug category"),
        );
        cat.set_threshold(DebugLevel::Info);
        let obj = crate::Bin::with_name("meh");

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let handler = move |category: DebugCategory,
                            level: DebugLevel,
                            _file: &glib::GStr,
                            _function: &glib::GStr,
                            _line: u32,
                            _object: Option<&LoggedObject>,
                            message: &DebugMessage| {
            let cat = DebugCategory::get("test-cat-log").unwrap();

            if category != cat {
                // This test can run in parallel with other tests, including new_and_log above.
                // We cannot be certain we only see our own messages.
                return;
            }

            assert_eq!(level, DebugLevel::Info);
            assert_eq!(message.get().unwrap().as_ref(), "meh");
            let _ = sender.lock().unwrap().send(());
        };

        remove_default_log_function();
        let log_fn = add_log_function(handler);
        info!(cat, obj = &obj, "meh");

        receiver.recv().unwrap();

        remove_log_function(log_fn);

        info!(cat, obj = &obj, "meh2");
        assert!(receiver.recv().is_err());
    }

    #[test]
    fn no_argument_evaluation() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "no_argument_evaluation",
            crate::DebugColorFlags::empty(),
            Some("No Argument Evaluation debug category"),
        );

        let mut arg_evaluated = false;
        trace!(cat, "{}", {
            arg_evaluated = true;
            "trace log"
        });

        assert!(!arg_evaluated);
    }

    #[cfg(feature = "v1_22")]
    #[test]
    fn id_logging() {
        crate::init().unwrap();

        let cat = DebugCategory::new(
            "log_with_id_test_category",
            crate::DebugColorFlags::empty(),
            Some("Blablabla"),
        );

        cat.set_threshold(crate::DebugLevel::Trace);

        trace!(cat, id = "123", "test");
        trace!(cat, id = glib::GString::from("123"), "test");
        trace!(cat, id = &glib::GString::from("123"), "test");

        // Try with a formatted string too (which is a different code path in the bindings)
        let log_id = glib::GString::from("456");
        trace!(cat, id = &log_id, "{log_id:?}");
    }
}
