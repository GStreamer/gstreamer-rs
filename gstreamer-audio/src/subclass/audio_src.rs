// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::{prelude::*, translate::*};
use gst::LoggableError;
use gst_base::subclass::prelude::*;

use super::prelude::*;
use crate::{ffi, AudioRingBufferSpec, AudioSrc};

pub trait AudioSrcImpl: AudioSrcImplExt + AudioBaseSrcImpl {
    fn close(&self) -> Result<(), LoggableError> {
        self.parent_close()
    }

    fn delay(&self) -> u32 {
        self.parent_delay()
    }

    fn open(&self) -> Result<(), LoggableError> {
        self.parent_open()
    }

    fn prepare(&self, spec: &mut AudioRingBufferSpec) -> Result<(), LoggableError> {
        AudioSrcImplExt::parent_prepare(self, spec)
    }

    fn unprepare(&self) -> Result<(), LoggableError> {
        self.parent_unprepare()
    }

    fn read(&self, audio_data: &mut [u8]) -> Result<(u32, Option<gst::ClockTime>), LoggableError> {
        self.parent_read(audio_data)
    }

    fn reset(&self) {
        self.parent_reset()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AudioSrcImplExt> Sealed for T {}
}

pub trait AudioSrcImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_close(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).close {
                None => return Ok(()),
                Some(f) => f,
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to close element using the parent function"
            )
        }
    }

    fn parent_delay(&self) -> u32 {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).delay {
                Some(f) => f,
                None => return 0,
            };
            f(self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0)
        }
    }

    fn parent_open(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).open {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to open element using the parent function"
            )
        }
    }

    fn parent_prepare(&self, spec: &mut AudioRingBufferSpec) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).prepare {
                Some(f) => f,
                None => return Ok(()),
            };
            gst::result_from_gboolean!(
                f(
                    self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0,
                    &mut spec.0
                ),
                gst::CAT_RUST,
                "Failed to prepare element using the parent function"
            )
        }
    }

    fn parent_unprepare(&self) -> Result<(), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).unprepare {
                Some(f) => f,
                None => {
                    return Err(gst::loggable_error!(
                        gst::CAT_RUST,
                        "Unprepare is not implemented!"
                    ))
                }
            };
            gst::result_from_gboolean!(
                f(self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0),
                gst::CAT_RUST,
                "Failed to unprepare element using the parent function"
            )
        }
    }

    fn parent_read(
        &self,
        buffer: &mut [u8],
    ) -> Result<(u32, Option<gst::ClockTime>), LoggableError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            let f = match (*parent_class).read {
                Some(f) => f,
                None => return Ok((0, gst::ClockTime::NONE)),
            };
            let buffer_ptr = buffer.as_mut_ptr() as *mut _;
            let mut timestamp = mem::MaybeUninit::uninit();
            let ret = f(
                self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0,
                buffer_ptr,
                buffer.len() as u32,
                timestamp.as_mut_ptr(),
            );
            if ret > 0 {
                Ok((ret, from_glib(timestamp.assume_init())))
            } else {
                Err(gst::loggable_error!(
                    gst::CAT_RUST,
                    "Failed to read using the parent function"
                ))
            }
        }
    }

    fn parent_reset(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAudioSrcClass;
            if let Some(f) = (*parent_class).reset {
                f(self.obj().unsafe_cast_ref::<AudioSrc>().to_glib_none().0)
            }
        }
    }
}

impl<T: AudioSrcImpl> AudioSrcImplExt for T {}

unsafe impl<T: AudioSrcImpl> IsSubclassable<T> for AudioSrc {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.close = Some(audiosrc_close::<T>);
        klass.delay = Some(audiosrc_delay::<T>);
        klass.open = Some(audiosrc_open::<T>);
        klass.prepare = Some(audiosrc_prepare::<T>);
        klass.unprepare = Some(audiosrc_unprepare::<T>);
        klass.read = Some(audiosrc_read::<T>);
        klass.reset = Some(audiosrc_reset::<T>);
    }
}

unsafe extern "C" fn audiosrc_close<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.close() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosrc_delay<T: AudioSrcImpl>(ptr: *mut ffi::GstAudioSrc) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, 0, { imp.delay() })
}

unsafe extern "C" fn audiosrc_open<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.open() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosrc_prepare<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
    spec: *mut ffi::GstAudioRingBufferSpec,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let spec = &mut *(spec as *mut AudioRingBufferSpec);

    gst::panic_to_error!(imp, false, {
        match AudioSrcImpl::prepare(imp, spec) {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosrc_unprepare<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, false, {
        match imp.unprepare() {
            Ok(()) => true,
            Err(err) => {
                err.log_with_imp(imp);
                false
            }
        }
    })
    .into_glib()
}

unsafe extern "C" fn audiosrc_read<T: AudioSrcImpl>(
    ptr: *mut ffi::GstAudioSrc,
    data: glib::ffi::gpointer,
    length: u32,
    timestamp: *mut gst::ffi::GstClockTime,
) -> u32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let data_slice = if length == 0 {
        &mut []
    } else {
        std::slice::from_raw_parts_mut(data as *mut u8, length as usize)
    };

    gst::panic_to_error!(imp, 0, {
        let (res, timestamp_res) = imp.read(data_slice).unwrap_or((0, gst::ClockTime::NONE));
        *timestamp = timestamp_res.into_glib();

        res
    })
}

unsafe extern "C" fn audiosrc_reset<T: AudioSrcImpl>(ptr: *mut ffi::GstAudioSrc) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    gst::panic_to_error!(imp, (), {
        imp.reset();
    });
}
