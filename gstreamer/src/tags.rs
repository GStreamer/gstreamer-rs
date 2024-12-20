// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem};

use glib::{
    prelude::*,
    translate::*,
    value::{FromValue, SendValue, ToSendValue, Value},
};

use crate::{ffi, Sample, TagError, TagMergeMode, TagScope};

pub trait Tag<'a> {
    type TagType: StaticType + FromValue<'a> + ToSendValue + Send + Sync;
    const TAG_NAME: &'static glib::GStr;
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $rust_tag:ident, $gst_tag:ident) => {
        pub enum $name {}
        impl<'a> Tag<'a> for $name {
            type TagType = $t;
            const TAG_NAME: &'static glib::GStr = unsafe { glib::GStr::from_utf8_with_nul_unchecked(ffi::$gst_tag) };
        }
    };
);

impl_tag!(Title, &'a str, TAG_TITLE, GST_TAG_TITLE);
impl_tag!(
    TitleSortname,
    &'a str,
    TAG_TITLE_SORTNAME,
    GST_TAG_TITLE_SORTNAME
);
impl_tag!(Artist, &'a str, TAG_ARTIST, GST_TAG_ARTIST);
impl_tag!(
    ArtistSortname,
    &'a str,
    TAG_ARTIST_SORTNAME,
    GST_TAG_ARTIST_SORTNAME
);
impl_tag!(Album, &'a str, TAG_ALBUM, GST_TAG_ALBUM);
impl_tag!(
    AlbumSortname,
    &'a str,
    TAG_ALBUM_SORTNAME,
    GST_TAG_ALBUM_SORTNAME
);
impl_tag!(AlbumArtist, &'a str, TAG_ALBUM_ARTIST, GST_TAG_ALBUM_ARTIST);
impl_tag!(
    AlbumArtistSortname,
    &'a str,
    TAG_ALBUM_ARTIST_SORTNAME,
    GST_TAG_ALBUM_ARTIST_SORTNAME
);
impl_tag!(Date, glib::Date, TAG_DATE, GST_TAG_DATE);
impl_tag!(
    DateTime,
    crate::auto::DateTime,
    TAG_DATE_TIME,
    GST_TAG_DATE_TIME
);
impl_tag!(Genre, &'a str, TAG_GENRE, GST_TAG_GENRE);
impl_tag!(Comment, &'a str, TAG_COMMENT, GST_TAG_COMMENT);
impl_tag!(
    ExtendedComment,
    &'a str,
    TAG_EXTENDED_COMMENT,
    GST_TAG_EXTENDED_COMMENT
);
impl_tag!(TrackNumber, u32, TAG_TRACK_NUMBER, GST_TAG_TRACK_NUMBER);
impl_tag!(TrackCount, u32, TAG_TRACK_COUNT, GST_TAG_TRACK_COUNT);
impl_tag!(
    AlbumVolumeNumber,
    u32,
    TAG_ALBUM_VOLUME_NUMBER,
    GST_TAG_ALBUM_VOLUME_NUMBER
);
impl_tag!(
    AlbumVolumeCount,
    u32,
    TAG_ALBUM_VOLUME_COUNT,
    GST_TAG_ALBUM_VOLUME_COUNT
);
impl_tag!(Location, &'a str, TAG_LOCATION, GST_TAG_LOCATION);
impl_tag!(Homepage, &'a str, TAG_HOMEPAGE, GST_TAG_HOMEPAGE);
impl_tag!(Description, &'a str, TAG_DESCRIPTION, GST_TAG_DESCRIPTION);
impl_tag!(Version, &'a str, TAG_VERSION, GST_TAG_VERSION);
impl_tag!(ISRC, &'a str, TAG_ISRC, GST_TAG_ISRC);
impl_tag!(
    Organization,
    &'a str,
    TAG_ORGANIZATION,
    GST_TAG_ORGANIZATION
);
impl_tag!(Copyright, &'a str, TAG_COPYRIGHT, GST_TAG_COPYRIGHT);
impl_tag!(
    CopyrightUri,
    &'a str,
    TAG_COPYRIGHT_URI,
    GST_TAG_COPYRIGHT_URI
);
impl_tag!(EncodedBy, &'a str, TAG_ENCODED_BY, GST_TAG_ENCODED_BY);
impl_tag!(Composer, &'a str, TAG_COMPOSER, GST_TAG_COMPOSER);
impl_tag!(Conductor, &'a str, TAG_CONDUCTOR, GST_TAG_CONDUCTOR);
impl_tag!(Contact, &'a str, TAG_CONTACT, GST_TAG_CONTACT);
impl_tag!(License, &'a str, TAG_LICENSE, GST_TAG_LICENSE);
impl_tag!(LicenseUri, &'a str, TAG_LICENSE_URI, GST_TAG_LICENSE_URI);
impl_tag!(Performer, &'a str, TAG_PERFORMER, GST_TAG_PERFORMER);
impl_tag!(Duration, crate::ClockTime, TAG_DURATION, GST_TAG_DURATION);
impl_tag!(Codec, &'a str, TAG_CODEC, GST_TAG_CODEC);
impl_tag!(VideoCodec, &'a str, TAG_VIDEO_CODEC, GST_TAG_VIDEO_CODEC);
impl_tag!(AudioCodec, &'a str, TAG_AUDIO_CODEC, GST_TAG_AUDIO_CODEC);
impl_tag!(
    SubtitleCodec,
    &'a str,
    TAG_SUBTITLE_CODEC,
    GST_TAG_SUBTITLE_CODEC
);
impl_tag!(
    ContainerFormat,
    &'a str,
    TAG_CONTAINER_FORMAT,
    GST_TAG_CONTAINER_FORMAT
);
impl_tag!(Bitrate, u32, TAG_BITRATE, GST_TAG_BITRATE);
impl_tag!(
    NominalBitrate,
    u32,
    TAG_NOMINAL_BITRATE,
    GST_TAG_NOMINAL_BITRATE
);
impl_tag!(
    MinimumBitrate,
    u32,
    TAG_MINIMUM_BITRATE,
    GST_TAG_MINIMUM_BITRATE
);
impl_tag!(
    MaximumBitrate,
    u32,
    TAG_MAXIMUM_BITRATE,
    GST_TAG_MAXIMUM_BITRATE
);
impl_tag!(Serial, u32, TAG_SERIAL, GST_TAG_SERIAL);
impl_tag!(Encoder, &'a str, TAG_ENCODER, GST_TAG_ENCODER);
impl_tag!(
    EncoderVersion,
    u32,
    TAG_ENCODER_VERSION,
    GST_TAG_ENCODER_VERSION
);
impl_tag!(TrackGain, f64, TAG_TRACK_GAIN, GST_TAG_TRACK_GAIN);
impl_tag!(TrackPeak, f64, TAG_TRACK_PEAK, GST_TAG_TRACK_PEAK);
impl_tag!(AlbumGain, f64, TAG_ALBUM_GAIN, GST_TAG_ALBUM_GAIN);
impl_tag!(AlbumPeak, f64, TAG_ALBUM_PEAK, GST_TAG_ALBUM_PEAK);
impl_tag!(
    ReferenceLevel,
    f64,
    TAG_REFERENCE_LEVEL,
    GST_TAG_REFERENCE_LEVEL
);
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(
    LanguageCode,
    &'a str,
    TAG_LANGUAGE_CODE,
    GST_TAG_LANGUAGE_CODE
);
impl_tag!(
    LanguageName,
    &'a str,
    TAG_LANGUAGE_NAME,
    GST_TAG_LANGUAGE_NAME
);
impl_tag!(Image, Sample, TAG_IMAGE, GST_TAG_IMAGE);
impl_tag!(
    PreviewImage,
    Sample,
    TAG_PREVIEW_IMAGE,
    GST_TAG_PREVIEW_IMAGE
);
impl_tag!(Attachment, Sample, TAG_ATTACHMENT, GST_TAG_ATTACHMENT);
impl_tag!(
    BeatsPerMinute,
    f64,
    TAG_BEATS_PER_MINUTE,
    GST_TAG_BEATS_PER_MINUTE
);
impl_tag!(Keywords, &'a str, TAG_KEYWORDS, GST_TAG_KEYWORDS);
impl_tag!(
    GeoLocationName,
    &'a str,
    TAG_GEO_LOCATION_NAME,
    GST_TAG_GEO_LOCATION_NAME
);
impl_tag!(
    GeoLocationLatitude,
    f64,
    TAG_GEO_LOCATION_LATITUDE,
    GST_TAG_GEO_LOCATION_LATITUDE
);
impl_tag!(
    GeoLocationLongitude,
    f64,
    TAG_GEO_LOCATION_LONGITUDE,
    GST_TAG_GEO_LOCATION_LONGITUDE
);
impl_tag!(
    GeoLocationElevation,
    f64,
    TAG_GEO_LOCATION_ELEVATION,
    GST_TAG_GEO_LOCATION_ELEVATION
);
impl_tag!(
    GeoLocationCity,
    &'a str,
    TAG_GEO_LOCATION_CITY,
    GST_TAG_GEO_LOCATION_CITY
);
impl_tag!(
    GeoLocationCountry,
    &'a str,
    TAG_GEO_LOCATION_COUNTRY,
    GST_TAG_GEO_LOCATION_COUNTRY
);
impl_tag!(
    GeoLocationSublocation,
    &'a str,
    TAG_GEO_LOCATION_SUBLOCATION,
    GST_TAG_GEO_LOCATION_SUBLOCATION
);
impl_tag!(
    GeoLocationHorizontalError,
    f64,
    TAG_GEO_LOCATION_HORIZONTAL_ERROR,
    GST_TAG_GEO_LOCATION_HORIZONTAL_ERROR
);
impl_tag!(
    GeoLocationMovementDirection,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_DIRECTION,
    GST_TAG_GEO_LOCATION_MOVEMENT_DIRECTION
);
impl_tag!(
    GeoLocationMovementSpeed,
    f64,
    TAG_GEO_LOCATION_MOVEMENT_SPEED,
    GST_TAG_GEO_LOCATION_MOVEMENT_SPEED
);
impl_tag!(
    GeoLocationCaptureDirection,
    f64,
    TAG_GEO_LOCATION_CAPTURE_DIRECTION,
    GST_TAG_GEO_LOCATION_CAPTURE_DIRECTION
);
impl_tag!(ShowName, &'a str, TAG_SHOW_NAME, GST_TAG_SHOW_NAME);
impl_tag!(
    ShowSortname,
    &'a str,
    TAG_SHOW_SORTNAME,
    GST_TAG_SHOW_SORTNAME
);
impl_tag!(
    ShowEpisodeNumber,
    u32,
    TAG_SHOW_EPISODE_NUMBER,
    GST_TAG_SHOW_EPISODE_NUMBER
);
impl_tag!(
    ShowSeasonNumber,
    u32,
    TAG_SHOW_SEASON_NUMBER,
    GST_TAG_SHOW_SEASON_NUMBER
);
impl_tag!(Lyrics, &'a str, TAG_LYRICS, GST_TAG_LYRICS);
impl_tag!(
    ComposerSortname,
    &'a str,
    TAG_COMPOSER_SORTNAME,
    GST_TAG_COMPOSER_SORTNAME
);
impl_tag!(Grouping, &'a str, TAG_GROUPING, GST_TAG_GROUPING);
impl_tag!(UserRating, u32, TAG_USER_RATING, GST_TAG_USER_RATING);
impl_tag!(
    DeviceManufacturer,
    &'a str,
    TAG_DEVICE_MANUFACTURER,
    GST_TAG_DEVICE_MANUFACTURER
);
impl_tag!(DeviceModel, &'a str, TAG_DEVICE_MODEL, GST_TAG_DEVICE_MODEL);
impl_tag!(
    ApplicationName,
    &'a str,
    TAG_APPLICATION_NAME,
    GST_TAG_APPLICATION_NAME
);
impl_tag!(
    ApplicationData,
    Sample,
    TAG_APPLICATION_DATA,
    GST_TAG_APPLICATION_DATA
);
impl_tag!(
    ImageOrientation,
    &'a str,
    TAG_IMAGE_ORIENTATION,
    GST_TAG_IMAGE_ORIENTATION
);
impl_tag!(Publisher, &'a str, TAG_PUBLISHER, GST_TAG_PUBLISHER);
impl_tag!(
    InterpretedBy,
    &'a str,
    TAG_INTERPRETED_BY,
    GST_TAG_INTERPRETED_BY
);
impl_tag!(
    MidiBaseNote,
    &'a str,
    TAG_MIDI_BASE_NOTE,
    GST_TAG_MIDI_BASE_NOTE
);
impl_tag!(PrivateData, Sample, TAG_PRIVATE_DATA, GST_TAG_PRIVATE_DATA);

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
mod v1_24 {
    use super::*;

    impl_tag!(
        ContainerSpecificTrackId,
        &'a str,
        TAG_CONTAINER_SPECIFIC_TRACK_ID,
        GST_TAG_CONTAINER_SPECIFIC_TRACK_ID
    );
}

#[cfg(feature = "v1_24")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
pub use v1_24::ContainerSpecificTrackId;

mini_object_wrapper!(TagList, TagListRef, ffi::GstTagList, || {
    ffi::gst_tag_list_get_type()
});

impl TagList {
    #[doc(alias = "gst_tag_list_new_empty")]
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_tag_list_new_empty()) }
    }
}

impl Default for TagList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct TagValue<T>(SendValue, PhantomData<T>);

impl<T> TagValue<T> {
    pub fn get<'a>(&'a self) -> T
    where
        T: StaticType + FromValue<'a>,
    {
        self.0.get().expect("Invalid tag type")
    }
}

impl TagListRef {
    #[doc(alias = "gst_tag_list_add")]
    pub fn add<'a, T: Tag<'a>>(&mut self, value: &T::TagType, mode: TagMergeMode) {
        // result can be safely ignored here as `value`'s type is tied to `T::TAG_NAME`
        let v = <T::TagType as ToSendValue>::to_send_value(value);
        let _res = self.add_value(T::TAG_NAME, &v, mode);
    }

    #[doc(alias = "gst_tag_list_add")]
    pub fn add_generic(
        &mut self,
        tag_name: impl IntoGStr,
        value: impl ToSendValue,
        mode: TagMergeMode,
    ) -> Result<(), TagError> {
        self.add_value(tag_name, &value.to_send_value(), mode)
    }

    #[doc(alias = "gst_tag_list_add_value")]
    pub fn add_value(
        &mut self,
        tag_name: impl IntoGStr,
        value: &glib::SendValue,
        mode: TagMergeMode,
    ) -> Result<(), TagError> {
        unsafe {
            tag_name.run_with_gstr(|tag_name| {
                let tag_type: glib::Type = from_glib(ffi::gst_tag_get_type(tag_name.as_ptr()));
                if tag_type != value.type_() {
                    return Err(TagError::TypeMismatch);
                }

                ffi::gst_tag_list_add_value(
                    self.as_mut_ptr(),
                    mode.into_glib(),
                    tag_name.as_ptr(),
                    value.to_glib_none().0,
                );
                Ok(())
            })
        }
    }

    #[doc(alias = "gst_tag_list_remove_tag")]
    pub fn remove<'a, T: Tag<'a>>(&mut self) {
        self.remove_generic(T::TAG_NAME);
    }

    #[doc(alias = "gst_tag_list_remove_tag")]
    pub fn remove_generic(&mut self, tag_name: impl IntoGStr) {
        unsafe {
            tag_name.run_with_gstr(|tag_name| {
                ffi::gst_tag_list_remove_tag(self.as_mut_ptr(), tag_name.as_ptr());
            })
        }
    }

    #[doc(alias = "gst_tag_list_get")]
    pub fn get<'a, T: Tag<'a>>(&self) -> Option<TagValue<T::TagType>> {
        self.generic(T::TAG_NAME).map(|value| {
            if !value.is::<T::TagType>() {
                panic!(
                    "TagListRef::get type mismatch for tag {}: {}",
                    T::TAG_NAME,
                    value.type_()
                );
            }
            TagValue(value, PhantomData)
        })
    }

    #[doc(alias = "gst_tag_list_get")]
    #[doc(alias = "get_generic")]
    pub fn generic(&self, tag_name: impl IntoGStr) -> Option<SendValue> {
        unsafe {
            let mut value: mem::MaybeUninit<SendValue> = mem::MaybeUninit::zeroed();

            let found: bool = tag_name.run_with_gstr(|tag_name| {
                from_glib(ffi::gst_tag_list_copy_value(
                    (*value.as_mut_ptr()).to_glib_none_mut().0,
                    self.as_ptr(),
                    tag_name.as_ptr(),
                ))
            });

            if !found {
                None
            } else {
                Some(value.assume_init())
            }
        }
    }

    #[doc(alias = "gst_tag_list_n_tags")]
    pub fn n_tags(&self) -> usize {
        unsafe { ffi::gst_tag_list_n_tags(self.as_ptr()) as usize }
    }

    #[doc(alias = "gst_tag_list_nth_tag_name")]
    pub fn nth_tag_name(&self, idx: usize) -> Option<&glib::GStr> {
        if idx >= self.n_tags() {
            return None;
        }

        unsafe {
            let name = ffi::gst_tag_list_nth_tag_name(self.as_ptr(), idx as u32);
            debug_assert!(!name.is_null());
            Some(glib::GStr::from_ptr(name))
        }
    }

    #[doc(alias = "get_index")]
    #[doc(alias = "gst_tag_list_get_index")]
    pub fn index<'a, T: Tag<'a>>(&'a self, idx: usize) -> Option<&'a TagValue<T::TagType>> {
        self.index_generic(T::TAG_NAME, idx).map(|value| {
            if !value.is::<T::TagType>() {
                panic!(
                    "TagListRef::get_index type mismatch for tag {}: {}",
                    T::TAG_NAME,
                    value.type_()
                );
            }
            unsafe { &*(value as *const SendValue as *const TagValue<T::TagType>) }
        })
    }

    #[doc(alias = "get_index_generic")]
    #[doc(alias = "gst_tag_list_get_index")]
    pub fn index_generic(&self, tag_name: impl IntoGStr, idx: usize) -> Option<&SendValue> {
        unsafe {
            let idx = u32::try_from(idx).ok()?;
            let value = tag_name.run_with_gstr(|tag_name| {
                ffi::gst_tag_list_get_value_index(self.as_ptr(), tag_name.as_ptr(), idx)
            });

            if value.is_null() {
                None
            } else {
                Some(&*(value as *const SendValue))
            }
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_tag_list_get_tag_size")]
    pub fn size<'a, T: Tag<'a>>(&self) -> usize {
        self.size_by_name(T::TAG_NAME)
    }

    #[doc(alias = "get_size_by_name")]
    #[doc(alias = "gst_tag_list_get_tag_size")]
    pub fn size_by_name(&self, tag_name: impl IntoGStr) -> usize {
        unsafe {
            tag_name.run_with_gstr(|tag_name| {
                ffi::gst_tag_list_get_tag_size(self.as_ptr(), tag_name.as_ptr()) as usize
            })
        }
    }

    pub fn iter_tag<'a, T: Tag<'a>>(&'a self) -> TagIter<'a, T> {
        TagIter::new(self)
    }

    pub fn iter_tag_generic(&self, tag_name: impl IntoGStr) -> GenericTagIter {
        let tag_name = glib::Quark::from_str(tag_name).as_str();
        GenericTagIter::new(self, tag_name)
    }

    pub fn iter_generic(&self) -> GenericIter {
        GenericIter::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    #[doc(alias = "gst_tag_list_insert")]
    pub fn insert(&mut self, other: &TagListRef, mode: TagMergeMode) {
        unsafe { ffi::gst_tag_list_insert(self.as_mut_ptr(), other.as_ptr(), mode.into_glib()) }
    }

    #[doc(alias = "gst_tag_list_merge")]
    pub fn merge(&self, other: &TagListRef, mode: TagMergeMode) -> TagList {
        unsafe {
            from_glib_full(ffi::gst_tag_list_merge(
                self.as_ptr(),
                other.as_ptr(),
                mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_scope")]
    #[doc(alias = "gst_tag_list_get_scope")]
    pub fn scope(&self) -> TagScope {
        unsafe { from_glib(ffi::gst_tag_list_get_scope(self.as_ptr())) }
    }

    #[doc(alias = "gst_tag_list_set_scope")]
    pub fn set_scope(&mut self, scope: TagScope) {
        unsafe { ffi::gst_tag_list_set_scope(self.as_mut_ptr(), scope.into_glib()) }
    }
}

impl fmt::Debug for TagList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <TagListRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for TagList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <TagListRef as fmt::Display>::fmt(self, f)
    }
}

impl PartialEq for TagList {
    fn eq(&self, other: &TagList) -> bool {
        TagListRef::eq(self, other)
    }
}

impl Eq for TagList {}

impl PartialEq<TagListRef> for TagList {
    fn eq(&self, other: &TagListRef) -> bool {
        TagListRef::eq(self, other)
    }
}

impl PartialEq<TagList> for TagListRef {
    fn eq(&self, other: &TagList) -> bool {
        TagListRef::eq(other, self)
    }
}

impl fmt::Debug for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("TagList");

        for (key, value) in self.iter() {
            debug.field(key, &value);
        }

        debug.finish()
    }
}

impl fmt::Display for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s =
            unsafe { glib::GString::from_glib_full(ffi::gst_tag_list_to_string(self.as_ptr())) };
        f.write_str(&s)
    }
}

impl PartialEq for TagListRef {
    #[doc(alias = "gst_tag_list_is_equal")]
    fn eq(&self, other: &TagListRef) -> bool {
        unsafe { from_glib(ffi::gst_tag_list_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for TagListRef {}

#[derive(Debug)]
pub struct TagIter<'a, T: Tag<'a>> {
    taglist: &'a TagListRef,
    idx: usize,
    size: usize,
    phantom: PhantomData<T>,
}

impl<'a, T: Tag<'a>> TagIter<'a, T> {
    fn new(taglist: &'a TagListRef) -> TagIter<'a, T> {
        skip_assert_initialized!();
        TagIter {
            taglist,
            idx: 0,
            size: taglist.size::<T>(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tag<'a>> Iterator for TagIter<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    type Item = &'a TagValue<T::TagType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.taglist.index::<T>(self.idx).unwrap();
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.size - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.size || overflow {
            self.idx = self.size;
            None
        } else {
            self.idx = end + 1;
            Some(self.taglist.index::<T>(end).unwrap())
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.size {
            None
        } else {
            Some(self.taglist.index::<T>(self.size - 1).unwrap())
        }
    }
}

impl<'a, T: Tag<'a>> DoubleEndedIterator for TagIter<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        Some(self.taglist.index::<T>(self.size).unwrap())
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.size;
            None
        } else {
            self.size = end - 1;
            Some(self.taglist.index::<T>(self.size).unwrap())
        }
    }
}

impl<'a, T: Tag<'a>> ExactSizeIterator for TagIter<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
}

impl<'a, T: Tag<'a>> std::iter::FusedIterator for TagIter<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
}

#[derive(Debug)]
pub struct GenericTagIter<'a> {
    taglist: &'a TagListRef,
    name: &'static glib::GStr,
    idx: usize,
    size: usize,
}

impl<'a> GenericTagIter<'a> {
    fn new(taglist: &'a TagListRef, name: &'static glib::GStr) -> GenericTagIter<'a> {
        skip_assert_initialized!();
        GenericTagIter {
            taglist,
            name,
            idx: 0,
            size: taglist.size_by_name(name),
        }
    }
}

impl<'a> Iterator for GenericTagIter<'a> {
    type Item = &'a SendValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.taglist.index_generic(self.name, self.idx).unwrap();
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.size - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.size || overflow {
            self.idx = self.size;
            None
        } else {
            self.idx = end + 1;
            Some(self.taglist.index_generic(self.name, end).unwrap())
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.size {
            None
        } else {
            Some(
                self.taglist
                    .index_generic(self.name, self.size - 1)
                    .unwrap(),
            )
        }
    }
}

impl DoubleEndedIterator for GenericTagIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        Some(self.taglist.index_generic(self.name, self.size).unwrap())
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.size;
            None
        } else {
            self.size = end - 1;
            Some(self.taglist.index_generic(self.name, self.size).unwrap())
        }
    }
}

impl ExactSizeIterator for GenericTagIter<'_> {}

impl std::iter::FusedIterator for GenericTagIter<'_> {}

#[derive(Debug)]
pub struct GenericIter<'a> {
    taglist: &'a TagListRef,
    idx: usize,
    size: usize,
}

impl<'a> GenericIter<'a> {
    fn new(taglist: &'a TagListRef) -> GenericIter<'a> {
        skip_assert_initialized!();
        let size = taglist.n_tags();
        GenericIter {
            taglist,
            idx: 0,
            size: if size > 0 { size } else { 0 },
        }
    }
}

impl<'a> Iterator for GenericIter<'a> {
    type Item = (&'a glib::GStr, GenericTagIter<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let name = self.taglist.nth_tag_name(self.idx).unwrap();
        let item = (name, self.taglist.iter_tag_generic(name));
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.size - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.size || overflow {
            self.idx = self.size;
            None
        } else {
            self.idx = end + 1;
            let name = self.taglist.nth_tag_name(end).unwrap();
            Some((name, self.taglist.iter_tag_generic(name)))
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.size {
            None
        } else {
            let name = self.taglist.nth_tag_name(self.size - 1).unwrap();
            Some((name, self.taglist.iter_tag_generic(name)))
        }
    }
}

impl DoubleEndedIterator for GenericIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        let name = self.taglist.nth_tag_name(self.idx).unwrap();
        Some((name, self.taglist.iter_tag_generic(name)))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.size;
            None
        } else {
            self.size = end - 1;
            let name = self.taglist.nth_tag_name(self.size).unwrap();
            Some((name, self.taglist.iter_tag_generic(name)))
        }
    }
}

impl ExactSizeIterator for GenericIter<'_> {}

impl std::iter::FusedIterator for GenericIter<'_> {}

#[derive(Debug)]
pub struct Iter<'a> {
    taglist: &'a TagListRef,
    idx: usize,
    size: usize,
}

impl<'a> Iter<'a> {
    fn new(taglist: &'a TagListRef) -> Iter<'a> {
        skip_assert_initialized!();
        let size = taglist.n_tags();
        Iter {
            taglist,
            idx: 0,
            size: if size > 0 { size } else { 0 },
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a glib::GStr, glib::SendValue);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let name = self.taglist.nth_tag_name(self.idx).unwrap();
        let item = (name, self.taglist.generic(name).unwrap());
        self.idx += 1;

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.size - self.idx;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.size - self.idx
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.idx.overflowing_add(n);
        if end >= self.size || overflow {
            self.idx = self.size;
            None
        } else {
            self.idx = end + 1;
            let name = self.taglist.nth_tag_name(end).unwrap();
            Some((name, self.taglist.generic(name).unwrap()))
        }
    }

    fn last(self) -> Option<Self::Item> {
        if self.idx == self.size {
            None
        } else {
            let name = self.taglist.nth_tag_name(self.size - 1).unwrap();
            Some((name, self.taglist.generic(name).unwrap()))
        }
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        let name = self.taglist.nth_tag_name(self.idx).unwrap();
        Some((name, self.taglist.generic(name).unwrap()))
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (end, overflow) = self.size.overflowing_sub(n);
        if end <= self.idx || overflow {
            self.idx = self.size;
            None
        } else {
            self.size = end - 1;
            let name = self.taglist.nth_tag_name(self.size).unwrap();
            Some((name, self.taglist.generic(name).unwrap()))
        }
    }
}

impl ExactSizeIterator for Iter<'_> {}

impl std::iter::FusedIterator for Iter<'_> {}

#[doc(alias = "gst_tag_exists")]
pub fn tag_exists(name: impl IntoGStr) -> bool {
    skip_assert_initialized!();
    unsafe { name.run_with_gstr(|name| from_glib(ffi::gst_tag_exists(name.as_ptr()))) }
}

#[doc(alias = "gst_tag_get_type")]
pub fn tag_get_type(name: impl IntoGStr) -> glib::Type {
    skip_assert_initialized!();
    unsafe { name.run_with_gstr(|name| from_glib(ffi::gst_tag_get_type(name.as_ptr()))) }
}

#[doc(alias = "gst_tag_get_nick")]
pub fn tag_get_nick<'b>(name: impl IntoGStr) -> &'b glib::GStr {
    skip_assert_initialized!();
    unsafe {
        let ptr = name.run_with_gstr(|name| ffi::gst_tag_get_nick(name.as_ptr()));
        glib::GStr::from_ptr(ptr)
    }
}

#[doc(alias = "gst_tag_get_description")]
pub fn tag_get_description<'b>(name: impl IntoGStr) -> Option<&'b glib::GStr> {
    skip_assert_initialized!();
    unsafe {
        let ptr = name.run_with_gstr(|name| ffi::gst_tag_get_description(name.as_ptr()));

        if ptr.is_null() {
            None
        } else {
            Some(glib::GStr::from_ptr(ptr))
        }
    }
}

#[doc(alias = "gst_tag_get_flag")]
pub fn tag_get_flag(name: impl IntoGStr) -> crate::TagFlag {
    skip_assert_initialized!();
    unsafe { name.run_with_gstr(|name| from_glib(ffi::gst_tag_get_flag(name.as_ptr()))) }
}

pub trait CustomTag<'a>: Tag<'a> {
    const FLAG: crate::TagFlag;
    const NICK: &'static glib::GStr;
    const DESCRIPTION: &'static glib::GStr;

    fn merge_func(src: &Value) -> Value {
        skip_assert_initialized!();
        merge_use_first(src)
    }
}

#[doc(alias = "gst_tag_register")]
pub fn register<T: for<'a> CustomTag<'a>>() {
    assert!(!tag_exists(T::TAG_NAME));

    unsafe extern "C" fn merge_func_trampoline<T: for<'a> CustomTag<'a>>(
        dest: *mut glib::gobject_ffi::GValue,
        src: *const glib::gobject_ffi::GValue,
    ) {
        *dest = T::merge_func(&*(src as *const Value)).into_raw();
    }

    unsafe {
        ffi::gst_tag_register(
            T::TAG_NAME.as_ptr(),
            T::FLAG.into_glib(),
            T::TagType::static_type().into_glib(),
            T::NICK.as_ptr(),
            T::DESCRIPTION.as_ptr(),
            Some(merge_func_trampoline::<T>),
        )
    }
}

#[doc(alias = "gst_tag_merge_use_first")]
pub fn merge_use_first(src: &Value) -> Value {
    skip_assert_initialized!();
    assert_eq!(src.type_(), crate::List::static_type());

    unsafe {
        let mut res = Value::uninitialized();
        ffi::gst_tag_merge_use_first(res.to_glib_none_mut().0, src.to_glib_none().0);
        res
    }
}

#[doc(alias = "gst_tag_merge_strings_with_comma")]
pub fn merge_strings_with_comma(src: &Value) -> Value {
    skip_assert_initialized!();
    assert_eq!(src.type_(), crate::List::static_type());

    unsafe {
        let mut res = Value::uninitialized();
        ffi::gst_tag_merge_strings_with_comma(res.to_glib_none_mut().0, src.to_glib_none().0);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClockTime;

    #[test]
    fn test_add() {
        crate::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>(&"some title", TagMergeMode::Append);
            tags.add::<Duration>(&(ClockTime::SECOND * 120), TagMergeMode::Append);
        }
        assert_eq!(
            tags.to_string(),
            "taglist, title=(string)\"some\\ title\", duration=(guint64)120000000000;"
        );
    }

    #[test]
    fn test_get() {
        crate::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>(&"some title", TagMergeMode::Append);
            tags.add::<Duration>(&(ClockTime::SECOND * 120), TagMergeMode::Append);
        }

        assert_eq!(tags.get::<Title>().unwrap().get(), "some title");
        assert_eq!(
            tags.get::<Duration>().unwrap().get(),
            ClockTime::SECOND * 120,
        );
        assert_eq!(tags.index::<Title>(0).unwrap().get(), "some title");
        assert_eq!(tags.index::<Title>(0).unwrap().get(), "some title");
        assert_eq!(
            tags.index::<Duration>(0).unwrap().get(),
            ClockTime::SECOND * 120,
        );
    }

    #[test]
    fn test_scope() {
        crate::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.scope(), TagScope::Stream);
        {
            let tags = tags.get_mut().unwrap();
            tags.set_scope(TagScope::Global);
        }
        assert_eq!(tags.scope(), TagScope::Global);
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_generic() {
        crate::init().unwrap();

        let mut tags = TagList::new();
        {
            let tags = tags.get_mut().unwrap();
            assert!(tags
                .add_generic(Title::TAG_NAME, "some title", TagMergeMode::Append)
                .is_ok());
            assert!(tags
                .add_generic(Title::TAG_NAME, "second title", TagMergeMode::Append)
                .is_ok());
            assert!(tags
                .add_generic(
                    Duration::TAG_NAME,
                    ClockTime::SECOND * 120,
                    TagMergeMode::Append
                )
                .is_ok());
            assert!(tags
                .add_generic(Title::TAG_NAME, "third title", TagMergeMode::Append)
                .is_ok());

            assert_eq!(
                tags.add_generic(
                    Image::TAG_NAME,
                    "`&[str] instead of `Sample`",
                    TagMergeMode::Append
                ),
                Err(TagError::TypeMismatch),
            );
        }

        assert_eq!(
            tags.index_generic(Title::TAG_NAME, 0).unwrap().get(),
            Ok(Some("some title"))
        );
        assert_eq!(
            tags.index_generic(Title::TAG_NAME, 1).unwrap().get(),
            Ok(Some("second title"))
        );
        assert_eq!(
            tags.index_generic(Duration::TAG_NAME, 0).unwrap().get(),
            Ok(Some(ClockTime::SECOND * 120))
        );
        assert_eq!(
            tags.index_generic(Title::TAG_NAME, 2).unwrap().get(),
            Ok(Some("third title"))
        );

        assert_eq!(
            tags.generic(Title::TAG_NAME).unwrap().get(),
            Ok(Some("some title, second title, third title"))
        );

        assert_eq!(tags.n_tags(), 2);
        assert_eq!(tags.nth_tag_name(0), Some(Title::TAG_NAME));
        assert_eq!(tags.size_by_name(Title::TAG_NAME), 3);
        assert_eq!(tags.nth_tag_name(1), Some(Duration::TAG_NAME));
        assert_eq!(tags.size_by_name(Duration::TAG_NAME), 1);

        // GenericTagIter
        let mut title_iter = tags.iter_tag_generic(Title::TAG_NAME);
        assert_eq!(title_iter.size_hint(), (3, Some(3)));
        let first_title = title_iter.next().unwrap();
        assert_eq!(first_title.get(), Ok(Some("some title")));
        let second_title = title_iter.next().unwrap();
        assert_eq!(second_title.get(), Ok(Some("second title")));
        let third_title = title_iter.next().unwrap();
        assert_eq!(third_title.get(), Ok(Some("third title")));
        assert!(title_iter.next().is_none());

        // GenericIter
        let mut tag_list_iter = tags.iter_generic();
        assert_eq!(tag_list_iter.size_hint(), (2, Some(2)));

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, Title::TAG_NAME);
        let first_title = tag_iter.next().unwrap();
        assert_eq!(first_title.get(), Ok(Some("some title")));
        let second_title = tag_iter.next().unwrap();
        assert_eq!(second_title.get(), Ok(Some("second title")));
        let third_title = tag_iter.next().unwrap();
        assert_eq!(third_title.get(), Ok(Some("third title")));
        assert!(tag_iter.next().is_none());

        let (tag_name, mut tag_iter) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, Duration::TAG_NAME);
        let first_duration = tag_iter.next().unwrap();
        assert_eq!(first_duration.get(), Ok(Some(ClockTime::SECOND * 120)));
        assert!(tag_iter.next().is_none());

        // Iter
        let mut tag_list_iter = tags.iter();
        assert_eq!(tag_list_iter.size_hint(), (2, Some(2)));

        let (tag_name, tag_value) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, Title::TAG_NAME);
        assert_eq!(
            tag_value.get(),
            Ok(Some("some title, second title, third title"))
        );

        let (tag_name, tag_value) = tag_list_iter.next().unwrap();
        assert_eq!(tag_name, Duration::TAG_NAME);
        assert_eq!(tag_value.get(), Ok(Some(ClockTime::SECOND * 120)));
        assert!(tag_iter.next().is_none());
    }

    #[test]
    fn test_custom_tags() {
        crate::init().unwrap();

        enum MyCustomTag {}

        impl<'a> Tag<'a> for MyCustomTag {
            type TagType = &'a str;
            const TAG_NAME: &'static glib::GStr = glib::gstr!("my-custom-tag");
        }

        impl CustomTag<'_> for MyCustomTag {
            const FLAG: crate::TagFlag = crate::TagFlag::Meta;
            const NICK: &'static glib::GStr = glib::gstr!("my custom tag");
            const DESCRIPTION: &'static glib::GStr =
                glib::gstr!("My own custom tag type for testing");

            fn merge_func(src: &Value) -> Value {
                skip_assert_initialized!();
                merge_strings_with_comma(src)
            }
        }

        register::<MyCustomTag>();

        assert!(tag_exists(MyCustomTag::TAG_NAME));
        assert_eq!(
            tag_get_type(MyCustomTag::TAG_NAME),
            <MyCustomTag as Tag>::TagType::static_type()
        );
        assert_eq!(tag_get_nick(MyCustomTag::TAG_NAME), MyCustomTag::NICK);
        assert_eq!(
            tag_get_description(MyCustomTag::TAG_NAME),
            Some(MyCustomTag::DESCRIPTION)
        );
        assert_eq!(tag_get_flag(MyCustomTag::TAG_NAME), MyCustomTag::FLAG);

        let mut tags = TagList::new();
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<MyCustomTag>(&"first one", TagMergeMode::Append);
        }

        assert_eq!(tags.get::<MyCustomTag>().unwrap().get(), "first one");

        {
            let tags = tags.get_mut().unwrap();
            tags.add::<MyCustomTag>(&"second one", TagMergeMode::Append);
        }

        assert_eq!(
            tags.get::<MyCustomTag>().unwrap().get(),
            "first one, second one"
        );
    }

    #[test]
    fn test_display() {
        crate::init().unwrap();

        let _ = format!("{}", TagList::new());
    }

    #[test]
    fn test_debug() {
        crate::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(format!("{tags:?}"), "TagList");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>(&"some title", TagMergeMode::Append);
            tags.add::<Duration>(&(ClockTime::SECOND * 120), TagMergeMode::Append);
        }
        assert_eq!(
            format!("{tags:?}"),
            "TagList { title: (gchararray) \"some title\", duration: (guint64) 120000000000 }"
        );
    }
}
