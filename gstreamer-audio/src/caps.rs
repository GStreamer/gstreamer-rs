use std::ops::{Bound::*, RangeBounds};

use gst::Caps;

use glib::IntoGStr;

use crate::{AudioFormat, AudioLayout};

pub struct AudioCapsBuilder<T> {
    builder: gst::caps::Builder<T>,
}

impl AudioCapsBuilder<gst::caps::NoFeature> {
    // rustdoc-stripper-ignore-next
    /// Constructs an `AudioCapsBuilder` for the "audio/x-raw" encoding.
    ///
    /// If left unchanged, the resulting `Caps` will be initialized with:
    /// - "audio/x-raw" encoding.
    /// - maximum rate range.
    /// - maximum channels range.
    /// - both interleaved and non-interleaved layouts.
    /// - all available formats.
    ///
    /// Use [`AudioCapsBuilder::for_encoding`] to specify another encoding.
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        let builder = Caps::builder(glib::gstr!("audio/x-raw"));
        let builder = AudioCapsBuilder { builder };
        builder
            .rate_range(..)
            .channels_range(..)
            .layout_list([AudioLayout::Interleaved, AudioLayout::NonInterleaved])
            .format_list(AudioFormat::iter_raw())
    }

    // rustdoc-stripper-ignore-next
    /// Constructs an `AudioCapsBuilder` for the "audio/x-raw" encoding
    /// with interleaved layout.
    ///
    /// If left unchanged, the resulting `Caps` will be initialized with:
    /// - "audio/x-raw" encoding.
    /// - maximum rate range.
    /// - maximum channels range.
    /// - interleaved layout.
    /// - all available formats.
    ///
    /// Use [`AudioCapsBuilder::for_encoding`] to specify another encoding.
    pub fn new_interleaved() -> Self {
        AudioCapsBuilder::new().layout(AudioLayout::Interleaved)
    }

    // rustdoc-stripper-ignore-next
    /// Constructs an `AudioCapsBuilder` for the specified encoding.
    ///
    /// The resulting `Caps` will use the `encoding` argument as name
    /// and will not contain any additional fields unless explicitly added.
    pub fn for_encoding(encoding: impl IntoGStr) -> Self {
        assert_initialized_main_thread!();
        AudioCapsBuilder {
            builder: Caps::builder(encoding),
        }
    }

    pub fn any_features(self) -> AudioCapsBuilder<gst::caps::HasFeatures> {
        AudioCapsBuilder {
            builder: self.builder.any_features(),
        }
    }

    pub fn features(
        self,
        features: impl IntoIterator<Item = impl IntoGStr>,
    ) -> AudioCapsBuilder<gst::caps::HasFeatures> {
        AudioCapsBuilder {
            builder: self.builder.features(features),
        }
    }
}

impl Default for AudioCapsBuilder<gst::caps::NoFeature> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AudioCapsBuilder<T> {
    pub fn format(self, format: AudioFormat) -> Self {
        Self {
            builder: self.builder.field(glib::gstr!("format"), format.to_str()),
        }
    }

    pub fn format_if(self, format: AudioFormat, predicate: bool) -> Self {
        if predicate {
            self.format(format)
        } else {
            self
        }
    }

    pub fn format_if_some(self, format: Option<AudioFormat>) -> Self {
        if let Some(format) = format {
            self.format(format)
        } else {
            self
        }
    }

    pub fn format_list(self, formats: impl IntoIterator<Item = AudioFormat>) -> Self {
        Self {
            builder: self.builder.field(
                glib::gstr!("format"),
                gst::List::new(formats.into_iter().map(|f| f.to_str())),
            ),
        }
    }

    pub fn format_list_if(
        self,
        formats: impl IntoIterator<Item = AudioFormat>,
        predicate: bool,
    ) -> Self {
        if predicate {
            self.format_list(formats)
        } else {
            self
        }
    }

    pub fn format_list_if_some(
        self,
        formats: Option<impl IntoIterator<Item = AudioFormat>>,
    ) -> Self {
        if let Some(formats) = formats {
            self.format_list(formats)
        } else {
            self
        }
    }

    pub fn format_list_if_not_empty(self, formats: impl IntoIterator<Item = AudioFormat>) -> Self {
        let mut formats = formats.into_iter().peekable();
        if formats.peek().is_some() {
            self.format_list(formats)
        } else {
            self
        }
    }

    pub fn rate(self, rate: i32) -> Self {
        Self {
            builder: self.builder.field(glib::gstr!("rate"), rate),
        }
    }

    pub fn rate_if(self, rate: i32, predicate: bool) -> Self {
        if predicate {
            self.rate(rate)
        } else {
            self
        }
    }

    pub fn rate_if_some(self, rate: Option<i32>) -> Self {
        if let Some(rate) = rate {
            self.rate(rate)
        } else {
            self
        }
    }

    pub fn rate_range(self, rates: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(rates);
        let gst_rates = gst::IntRange::<i32>::new(start, end);
        Self {
            builder: self.builder.field(glib::gstr!("rate"), gst_rates),
        }
    }

    pub fn rate_range_if(self, rates: impl RangeBounds<i32>, predicate: bool) -> Self {
        if predicate {
            self.rate_range(rates)
        } else {
            self
        }
    }

    pub fn rate_range_if_some(self, rates: Option<impl RangeBounds<i32>>) -> Self {
        if let Some(rates) = rates {
            self.rate_range(rates)
        } else {
            self
        }
    }

    pub fn rate_list(self, rates: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self
                .builder
                .field(glib::gstr!("rate"), gst::List::new(rates)),
        }
    }

    pub fn rate_list_if(self, rates: impl IntoIterator<Item = i32>, predicate: bool) -> Self {
        if predicate {
            self.rate_list(rates)
        } else {
            self
        }
    }

    pub fn rate_list_if_some(self, rates: Option<impl IntoIterator<Item = i32>>) -> Self {
        if let Some(rates) = rates {
            self.rate_list(rates)
        } else {
            self
        }
    }

    pub fn rate_list_if_not_empty(self, rates: impl IntoIterator<Item = i32>) -> Self {
        let mut rates = rates.into_iter().peekable();
        if rates.peek().is_some() {
            self.rate_list(rates)
        } else {
            self
        }
    }

    pub fn channels(self, channels: i32) -> Self {
        Self {
            builder: self.builder.field(glib::gstr!("channels"), channels),
        }
    }

    pub fn channels_if(self, channels: i32, predicate: bool) -> Self {
        if predicate {
            self.channels(channels)
        } else {
            self
        }
    }

    pub fn channels_if_some(self, channels: Option<i32>) -> Self {
        if let Some(channels) = channels {
            self.channels(channels)
        } else {
            self
        }
    }

    pub fn channels_range(self, channels: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(channels);
        let gst_channels: gst::IntRange<i32> = gst::IntRange::new(start, end);
        Self {
            builder: self.builder.field(glib::gstr!("channels"), gst_channels),
        }
    }

    pub fn channels_range_if(self, channels: impl RangeBounds<i32>, predicate: bool) -> Self {
        if predicate {
            self.channels_range(channels)
        } else {
            self
        }
    }

    pub fn channels_range_if_some(self, channels: Option<impl RangeBounds<i32>>) -> Self {
        if let Some(channels) = channels {
            self.channels_range(channels)
        } else {
            self
        }
    }

    pub fn channels_list(self, channels: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self
                .builder
                .field(glib::gstr!("channels"), gst::List::new(channels)),
        }
    }

    pub fn channels_list_if(
        self,
        channels: impl IntoIterator<Item = i32>,
        predicate: bool,
    ) -> Self {
        if predicate {
            self.channels_list(channels)
        } else {
            self
        }
    }

    pub fn channels_list_if_some(self, channels: Option<impl IntoIterator<Item = i32>>) -> Self {
        if let Some(channels) = channels {
            self.channels_list(channels)
        } else {
            self
        }
    }

    pub fn channels_list_if_not_empty(self, channels: impl IntoIterator<Item = i32>) -> Self {
        let mut channels = channels.into_iter().peekable();
        if channels.peek().is_some() {
            self.channels_list(channels)
        } else {
            self
        }
    }

    pub fn layout(self, layout: AudioLayout) -> Self {
        Self {
            builder: self
                .builder
                .field(glib::gstr!("layout"), layout_str(layout)),
        }
    }

    pub fn layout_if(self, layout: AudioLayout, predicate: bool) -> Self {
        if predicate {
            self.layout(layout)
        } else {
            self
        }
    }

    pub fn layout_if_some(self, layout: Option<AudioLayout>) -> Self {
        if let Some(layout) = layout {
            self.layout(layout)
        } else {
            self
        }
    }

    pub fn layout_list(self, layouts: impl IntoIterator<Item = AudioLayout>) -> Self {
        Self {
            builder: self.builder.field(
                glib::gstr!("layout"),
                gst::List::new(layouts.into_iter().map(layout_str)),
            ),
        }
    }

    pub fn layout_list_if(
        self,
        layouts: impl IntoIterator<Item = AudioLayout>,
        predicate: bool,
    ) -> Self {
        if predicate {
            self.layout_list(layouts)
        } else {
            self
        }
    }

    pub fn layout_list_if_some(
        self,
        layouts: Option<impl IntoIterator<Item = AudioLayout>>,
    ) -> Self {
        if let Some(layouts) = layouts {
            self.layout_list(layouts)
        } else {
            self
        }
    }

    pub fn layout_list_if_not_empty(self, layouts: impl IntoIterator<Item = AudioLayout>) -> Self {
        let mut layouts = layouts.into_iter().peekable();
        if layouts.peek().is_some() {
            self.layout_list(layouts)
        } else {
            self
        }
    }

    pub fn channel_mask(self, channel_mask: u64) -> Self {
        Self {
            builder: self
                .builder
                .field("channel-mask", gst::Bitmask::new(channel_mask)),
        }
    }

    pub fn channel_mask_if(self, channel_mask: u64, predicate: bool) -> Self {
        if predicate {
            self.channel_mask(channel_mask)
        } else {
            self
        }
    }

    pub fn channel_mask_if_some(self, channel_mask: Option<u64>) -> Self {
        if let Some(channel_mask) = channel_mask {
            self.channel_mask(channel_mask)
        } else {
            self
        }
    }

    pub fn fallback_channel_mask(self) -> Self {
        let channels = self.builder.structure().get::<i32>(glib::gstr!("channels"));
        match channels {
            Ok(channels) => Self {
                builder: self.builder.field(
                    glib::gstr!("channel-mask"),
                    gst::Bitmask::new(crate::AudioChannelPosition::fallback_mask(channels as u32)),
                ),
            },
            Err(e) => panic!("{e:?}"),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets field `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn field(self, name: impl IntoGStr, value: impl Into<glib::Value> + Send) -> Self {
        Self {
            builder: self.builder.field(name, value),
        }
    }

    gst::impl_builder_gvalue_extra_setters!(field);

    #[must_use]
    pub fn build(self) -> gst::Caps {
        self.builder.build()
    }
}

fn range_bounds_i32_start_end(range: impl RangeBounds<i32>) -> (i32, i32) {
    skip_assert_initialized!();
    let start = match range.start_bound() {
        Unbounded => 1,
        Excluded(n) => n + 1,
        Included(n) => *n,
    };
    let end = match range.end_bound() {
        Unbounded => i32::MAX,
        Excluded(n) => n - 1,
        Included(n) => *n,
    };
    (start, end)
}

fn layout_str(layout: AudioLayout) -> &'static glib::GStr {
    skip_assert_initialized!();
    match layout {
        crate::AudioLayout::Interleaved => glib::gstr!("interleaved"),
        crate::AudioLayout::NonInterleaved => glib::gstr!("non-interleaved"),
        crate::AudioLayout::__Unknown(_) => glib::gstr!("unknown"),
    }
}

#[cfg(test)]
mod tests {
    use super::{AudioCapsBuilder, AudioFormat};

    #[test]
    fn default_encoding() {
        gst::init().unwrap();
        let caps = AudioCapsBuilder::new().build();
        assert_eq!(caps.structure(0).unwrap().name(), "audio/x-raw");
    }

    #[test]
    fn explicit_encoding() {
        gst::init().unwrap();
        let caps = AudioCapsBuilder::for_encoding("audio/mpeg").build();
        assert_eq!(caps.structure(0).unwrap().name(), "audio/mpeg");
    }

    #[test]
    fn format_if() {
        gst::init().unwrap();

        let formats = [AudioFormat::S24be, AudioFormat::S16be, AudioFormat::U8];
        let caps_with_format = AudioCapsBuilder::for_encoding("audio/x-raw")
            .format_list(formats)
            .build();
        assert!(caps_with_format
            .structure(0)
            .unwrap()
            .get::<gst::List>("format")
            .unwrap()
            .iter()
            .map(|f| f.get::<String>().unwrap())
            .eq(formats.iter().map(|f| f.to_string())));

        let caps = AudioCapsBuilder::for_encoding("audio/x-raw")
            .format_list_if_some(Some(formats))
            .build();
        assert_eq!(caps, caps_with_format);

        let caps = AudioCapsBuilder::for_encoding("audio/x-raw")
            .format_list_if_some(Option::<Vec<AudioFormat>>::None)
            .build();
        assert!(!caps.structure(0).unwrap().has_field("format"));

        let caps = AudioCapsBuilder::for_encoding("audio/x-raw")
            .format_list_if_not_empty(formats)
            .build();
        assert_eq!(caps, caps_with_format);

        let caps = AudioCapsBuilder::for_encoding("audio/x-raw")
            .format_list_if_not_empty(Vec::<AudioFormat>::new())
            .build();
        assert!(!caps.structure(0).unwrap().has_field("format"));
    }
}
