// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ptr, slice};

use glib::translate::*;

use crate::{ffi, Caps, Plugin, Rank, TypeFindFactory, TypeFindProbability};

#[repr(transparent)]
#[derive(Debug)]
#[doc(alias = "GstTypeFind")]
pub struct TypeFind(ffi::GstTypeFind);

pub trait TypeFindImpl {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]>;
    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps);
    #[doc(alias = "get_length")]
    fn length(&self) -> Option<u64> {
        None
    }
}

impl TypeFind {
    #[doc(alias = "gst_type_find_register")]
    pub fn register<F>(
        plugin: Option<&Plugin>,
        name: &str,
        rank: Rank,
        extensions: Option<&str>,
        possible_caps: Option<&Caps>,
        func: F,
    ) -> Result<(), glib::error::BoolError>
    where
        F: Fn(&mut TypeFind) + Send + Sync + 'static,
    {
        skip_assert_initialized!();
        unsafe {
            let func: Box<F> = Box::new(func);
            let func = Box::into_raw(func);

            let res = ffi::gst_type_find_register(
                plugin.to_glib_none().0,
                name.to_glib_none().0,
                rank.into_glib() as u32,
                Some(type_find_trampoline::<F>),
                extensions.to_glib_none().0,
                possible_caps.to_glib_none().0,
                func as *mut _,
                Some(type_find_closure_drop::<F>),
            );

            glib::result_from_gboolean!(res, "Failed to register typefind factory")
        }
    }

    #[doc(alias = "gst_type_find_peek")]
    pub fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        unsafe {
            let data = ffi::gst_type_find_peek(&mut self.0, offset, size);
            if data.is_null() {
                None
            } else if size == 0 {
                Some(&[])
            } else {
                Some(slice::from_raw_parts(data, size as usize))
            }
        }
    }

    #[doc(alias = "gst_type_find_suggest")]
    pub fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        unsafe {
            ffi::gst_type_find_suggest(
                &mut self.0,
                probability.into_glib() as u32,
                caps.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_length")]
    #[doc(alias = "gst_type_find_get_length")]
    pub fn length(&mut self) -> Option<u64> {
        unsafe {
            let len = ffi::gst_type_find_get_length(&mut self.0);
            if len == 0 {
                None
            } else {
                Some(len)
            }
        }
    }
}

impl TypeFindFactory {
    #[doc(alias = "gst_type_find_factory_call_function")]
    pub fn call_function<T: TypeFindImpl + ?Sized>(&self, mut find: &mut T) {
        unsafe {
            let find_ptr = &mut find as *mut &mut T as glib::ffi::gpointer;
            let mut find = ffi::GstTypeFind {
                peek: Some(type_find_peek::<T>),
                suggest: Some(type_find_suggest::<T>),
                data: find_ptr,
                get_length: Some(type_find_get_length::<T>),
                _gst_reserved: [ptr::null_mut(); 4],
            };

            ffi::gst_type_find_factory_call_function(self.to_glib_none().0, &mut find)
        }
    }
}

unsafe extern "C" fn type_find_trampoline<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    find: *mut ffi::GstTypeFind,
    user_data: glib::ffi::gpointer,
) {
    let func: &F = &*(user_data as *const F);
    func(&mut *(find as *mut TypeFind));
}

unsafe extern "C" fn type_find_closure_drop<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    data: glib::ffi::gpointer,
) {
    let _ = Box::<F>::from_raw(data as *mut _);
}

unsafe extern "C" fn type_find_peek<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
    offset: i64,
    size: u32,
) -> *const u8 {
    let find = &mut *(data as *mut &mut T);
    match find.peek(offset, size) {
        None => ptr::null(),
        Some(data) => data.as_ptr(),
    }
}

unsafe extern "C" fn type_find_suggest<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
    probability: u32,
    caps: *mut ffi::GstCaps,
) {
    let find = &mut *(data as *mut &mut T);
    find.suggest(from_glib(probability as i32), &from_glib_borrow(caps));
}

unsafe extern "C" fn type_find_get_length<T: TypeFindImpl + ?Sized>(
    data: glib::ffi::gpointer,
) -> u64 {
    let find = &*(data as *mut &mut T);
    find.length().unwrap_or(u64::MAX)
}

#[derive(Debug)]
pub struct SliceTypeFind<T: AsRef<[u8]>> {
    pub probability: Option<TypeFindProbability>,
    pub caps: Option<Caps>,
    data: T,
}

impl<T: AsRef<[u8]>> SliceTypeFind<T> {
    pub fn new(data: T) -> SliceTypeFind<T> {
        assert_initialized_main_thread!();
        SliceTypeFind {
            probability: None,
            caps: None,
            data,
        }
    }

    pub fn run(&mut self) {
        let factories = TypeFindFactory::factories();

        for factory in factories {
            factory.call_function(self);
            if let Some(prob) = self.probability {
                if prob >= TypeFindProbability::Maximum {
                    break;
                }
            }
        }
    }

    pub fn type_find(data: T) -> (TypeFindProbability, Option<Caps>) {
        assert_initialized_main_thread!();
        let mut t = SliceTypeFind {
            probability: None,
            caps: None,
            data,
        };

        t.run();

        (t.probability.unwrap_or(TypeFindProbability::None), t.caps)
    }
}

impl<T: AsRef<[u8]>> TypeFindImpl for SliceTypeFind<T> {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        let data = self.data.as_ref();
        let len = data.len();

        let offset = if offset >= 0 {
            usize::try_from(offset).ok()?
        } else {
            let offset = usize::try_from(offset.unsigned_abs()).ok()?;
            if len < offset {
                return None;
            }

            len - offset
        };

        let size = usize::try_from(size).ok()?;
        let end_offset = offset.checked_add(size)?;
        if end_offset <= len {
            Some(&data[offset..end_offset])
        } else {
            None
        }
    }

    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        match self.probability {
            None => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            Some(old_probability) if old_probability < probability => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            _ => (),
        }
    }
    fn length(&self) -> Option<u64> {
        Some(self.data.as_ref().len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typefind_call_function() {
        crate::init().unwrap();

        let xml_factory = TypeFindFactory::factories()
            .into_iter()
            .find(|f| {
                f.caps()
                    .map(|c| {
                        c.structure(0)
                            .map(|s| s.name() == "application/xml")
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .unwrap();

        let data = b"<?xml version=\"1.0\"?><test>test</test>";
        let data = &data[..];
        let mut typefind = SliceTypeFind::new(&data);
        xml_factory.call_function(&mut typefind);

        assert_eq!(
            typefind.caps,
            Some(Caps::builder("application/xml").build())
        );
        assert_eq!(typefind.probability, Some(TypeFindProbability::Minimum));
    }

    #[test]
    fn test_typefind_register() {
        crate::init().unwrap();

        TypeFind::register(
            None,
            "test_typefind",
            crate::Rank::PRIMARY,
            None,
            Some(&Caps::builder("test/test").build()),
            |typefind| {
                assert_eq!(typefind.length(), Some(8));
                let mut found = false;
                if let Some(data) = typefind.peek(0, 8) {
                    if data == b"abcdefgh" {
                        found = true;
                    }
                }

                if found {
                    typefind.suggest(
                        TypeFindProbability::Likely,
                        &Caps::builder("test/test").build(),
                    );
                }
            },
        )
        .unwrap();

        let data = b"abcdefgh";
        let data = &data[..];
        let (probability, caps) = SliceTypeFind::type_find(data);

        assert_eq!(caps, Some(Caps::builder("test/test").build()));
        assert_eq!(probability, TypeFindProbability::Likely);
    }
}
