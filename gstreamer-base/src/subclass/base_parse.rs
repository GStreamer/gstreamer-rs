// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;
use gst::subclass::prelude::*;

use crate::{ffi, prelude::*, BaseParse, BaseParseFrame};

pub trait BaseParseImpl: BaseParseImplExt + ElementImpl {
    fn start(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_start()
    }

    fn stop(&self) -> Result<(), gst::ErrorMessage> {
        self.parent_stop()
    }

    fn set_sink_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        self.parent_set_sink_caps(caps)
    }

    fn handle_frame(
        &self,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        self.parent_handle_frame(frame)
    }

    fn convert(
        &self,
        src_val: impl gst::format::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        self.parent_convert(src_val, dest_format)
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::BaseParseImplExt> Sealed for T {}
}

pub trait BaseParseImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_start(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .start
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseParse>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `start` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_stop(&self) -> Result<(), gst::ErrorMessage> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .stop
                .map(|f| {
                    if from_glib(f(self
                        .obj()
                        .unsafe_cast_ref::<BaseParse>()
                        .to_glib_none()
                        .0))
                    {
                        Ok(())
                    } else {
                        Err(gst::error_msg!(
                            gst::CoreError::StateChange,
                            ["Parent function `stop` failed"]
                        ))
                    }
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_set_sink_caps(&self, caps: &gst::Caps) -> Result<(), gst::LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            (*parent_class)
                .set_sink_caps
                .map(|f| {
                    gst::result_from_gboolean!(
                        f(
                            self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                            caps.to_glib_none().0,
                        ),
                        gst::CAT_RUST,
                        "Parent function `set_sink_caps` failed",
                    )
                })
                .unwrap_or(Ok(()))
        }
    }

    fn parent_handle_frame(
        &self,
        frame: BaseParseFrame,
    ) -> Result<(gst::FlowSuccess, u32), gst::FlowError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let mut skipsize = 0;
            (*parent_class)
                .handle_frame
                .map(|f| {
                    let res = try_from_glib(f(
                        self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                        frame.to_glib_none().0,
                        &mut skipsize,
                    ));
                    (res.unwrap(), skipsize as u32)
                })
                .ok_or(gst::FlowError::Error)
        }
    }

    fn parent_convert(
        &self,
        src_val: impl gst::format::FormattedValue,
        dest_format: gst::Format,
    ) -> Option<gst::GenericFormattedValue> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstBaseParseClass;
            let res = (*parent_class).convert.map(|f| {
                let mut dest_val = mem::MaybeUninit::uninit();

                let res = from_glib(f(
                    self.obj().unsafe_cast_ref::<BaseParse>().to_glib_none().0,
                    src_val.format().into_glib(),
                    src_val.into_raw_value(),
                    dest_format.into_glib(),
                    dest_val.as_mut_ptr(),
                ));
                (res, dest_val)
            });

            match res {
                Some((true, dest_val)) => Some(gst::GenericFormattedValue::new(
                    dest_format,
                    dest_val.assume_init(),
                )),
                _ => None,
            }
        }
    }
}

impl<T: BaseParseImpl> BaseParseImplExt for T {}

unsafe impl<T: BaseParseImpl> IsSubclassable<T> for BaseParse {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.start = Some(base_parse_start::<T>);
        klass.stop = Some(base_parse_stop::<T>);
        klass.set_sink_caps = Some(base_parse_set_sink_caps::<T>);
        klass.handle_frame = Some(base_parse_handle_frame::<T>);
        klass.convert = Some(base_parse_convert::<T>);
    }
}

unsafe extern "C" fn base_parse_start<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.start() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_stop<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.stop() {
            Ok(()) => true,
            Err(err) => {
                imp.post_error_message(err);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_set_sink_caps<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    caps: *mut gst::ffi::GstCaps,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let caps: Borrowed<gst::Caps> = from_glib_borrow(caps);

    gst::panic_to_error!(imp, false, {
        match imp.set_sink_caps(&caps) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn base_parse_handle_frame<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    frame: *mut ffi::GstBaseParseFrame,
    skipsize: *mut i32,
) -> gst::ffi::GstFlowReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let instance = imp.obj();
    let instance = instance.unsafe_cast_ref::<BaseParse>();
    let wrap_frame = BaseParseFrame::new(frame, instance);

    let res = gst::panic_to_error!(imp, Err(gst::FlowError::Error), {
        imp.handle_frame(wrap_frame)
    });

    match res {
        Ok((flow, skip)) => {
            *skipsize = i32::try_from(skip).expect("skip is higher than i32::MAX");
            gst::FlowReturn::from_ok(flow)
        }
        Err(flow) => gst::FlowReturn::from_error(flow),
    }
    .into_glib()
}

unsafe extern "C" fn base_parse_convert<T: BaseParseImpl>(
    ptr: *mut ffi::GstBaseParse,
    source_format: gst::ffi::GstFormat,
    source_value: i64,
    dest_format: gst::ffi::GstFormat,
    dest_value: *mut i64,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let source = gst::GenericFormattedValue::new(from_glib(source_format), source_value);

    let res = gst::panic_to_error!(imp, None, { imp.convert(source, from_glib(dest_format)) });

    match res {
        Some(dest) => {
            *dest_value = dest.into_raw_value();
            true
        }
        _ => false,
    }
    .into_glib()
}
