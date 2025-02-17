# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
specifically the [variant used by Rust](http://doc.crates.io/manifest.html#the-version-field).

## [0.23.5] - 2025-02-17
### Fixed
- Properly validate `gst::IntRange::with_step()` step size
- Fix `gst::Buffer` serde serialization
- Forward gap events by default in `gst_utils::StreamProducer`.
- Correctly account for alternate interlace mode in `gst_video::VideoMeta::add_full()`.
- Return `Result`s instead of `bool`s in new `gst_play` API.

### Added
- Support for `TracerImpl::USE_STRUCTURE_PARAMS` with GStreamer < 1.26.
- Bindings for `gst_analytics::ODMtd`.
- Bindings for `TopSurroundRight` and `TopSurroundLeft` audio channels.
- Bindings for AV1 and H266 codec helpers API.
- Bindings for `gst_audio::reorder_channels_with_reorder_map()`.

### Changed
- Updated GStreamer gir files for latest 1.26 APIs.
- Documentation links URL was updated.

## [0.23.4] - 2024-12-21
### Fixed
- `gst_video::VideoFrame::plane_data()` does not return a truncated buffer for
  the alpha plane in A420 and similar formats anymore.
- `FnMut` closures are now correctly passed via a mutable reference to FFI code.
- Order of arguments in `gst_video::VideoFormat::from_mask()` was corrected.

### Added
- Bindings for 1.26 `gst_analytics::Tensor` API.
- `gst::DebugCategory::as_ptr()` and `Hash` impl, `gst::DebugMessage::as_ptr()`.
- Support for hex-dumping `&mut [u8]` in addition to `&[u8]`, `gst::Buffer`, etc.
- Functions to work with meta `glib::Type`s.

### Changed
- Updated GStreamer gir files for latest 1.26 APIs.

## [0.23.3] - 2024-11-01
### Fixed
- Bind `gst::Pad::proxy_query_caps()` to the correct C function.
- Update `gst_utils::StreamProducer` appsrc latency upon appsink latency event.
- Fix type of `gst_app::AppSinkBuilder::processing_deadline()`.

### Added
- Various new `gst::Iterator` constructors for convenience.

### Changed
- Updated GStreamer gir files for latest 1.26 APIs.

## [0.23.2] - 2024-09-28
### Fixed
- Lifetime of `gst::TagList::index()` return value is correctly bound to `&self` now.
- Don't assume `gst::Structure` name / field names have `'static` lifetime.
- Set pad probe data to `NULL` if `HANDLED` is returned and the item is an
  event, buffer or buffer list.
- Don't unnecessarily add `#[link]` attribute to the `extern "C"` sections to
  allow linking against gstreamer-full and make static linking easier.

### Changed
- Add `#[must_use]` to `gst_video::VideoTimeCode::add_interval()`.

### Added
- Add API to take events/buffers from a `gst::PadProbeInfo`.
- Add `gst::EventViewMut` and `gst::Event::view_mut()`, and a few setters for
  event fields.
- Add `gst::MessageViewMut` and `gst::Message::view_mut()`, and a few setters
  for message fields.

## [0.23.1] - 2024-08-27
### Fixed
- Support `gst_utils::StreamProducer` API on platforms without 64 bit atomics.
- Fix off-by-one in `gst::BufferList::remove()` range end handling.
- Pass an empty tag array instead of NULL in `gst::CustomMeta::register_simple()`.
- Fix various new clippy warnings.

### Added
- Add getters for `gst::format::Percent`.

## [0.23.0] - 2024-07-11
### Changed
- Compatible with gtk-rs-core 0.20 / gtk4-rs 0.9.
- Update GStreamer gir files to latest (upcoming) 1.26 APIs.
- Minimum support Rust version is updated from 1.70 to 1.71.1.
- Move `gst::Meta` tags into separate modules and improve API around them.
- Improve `gst::Meta` transform functions and make the API more generic, and
  as part of that add support the video meta transform.
- Pass an immutable instead of mutable output buffer reference to
  `gst_rtp::RtpHeaderExtension::write()` function.
- Make `gst_net::PtpClock::new()` constructor fallible.
- Change `gst_rtsp_server::RTSPToken` API to be consistent with
  `gst::Structure`, specifically add a builder.
- Change `from_glib_ptr_borrow()` functions to work with references instead of
  raw pointers for improved safety.
- Improve code generation when building with `panic=abort`.
- Change `gst::BufferList` APIs to work with ranges instead of index+length.
- Use various `usize` instead of `u32` for various indices in `gst::Buffer`,
  `gst::Caps`, `gst::Structure` and related APIs.
- `gst::Clock` calibration-related API uses plain `u64` instead of
  `gst::ClockTime` for the clock rate.
- `gst::debug!` and related macros use `obj = x` instead of `obj: x` for
  specifying the target object now. Similar for `imp` and `id`. The old syntax
  is still supported but deprecated. The new syntax works better with tooling,
  especially rustfmt.

### Added
- Mutable access to the `gst_webrtc::WebRTCSessionDescription` fields.
- `gst::StructureBuilder::field_if_some()` and the same for related builders
  to only set a value if `Some(x)` is provided.
- `gst::StructureBuilder::field_from_iter()` and `field_if_not_empty()` for
  various builders.
- `gst::PadBuilder` API for selecting an automatically generated name.
- Adapter for the `log` crate around the GStreamer debug log system. This
  allows the macros from the `log` crate to be output via the GStreamer debug
  log system.
- Bindings for the double click `gst_video::Navigation` event.
- Bindings for `gst_pbutils` missing/install plugins API.
- Setters for `gst_editing_services::FrameCompositionMeta`.
- `ges::UriClipAsset::new()`.

## [0.22.6] - 2024-06-19
### Fixed
- When logging with an id and a formatted log message this would previously panic.
- A couple of clippy warnings.

## [0.22.5] - 2024-05-23
### Fixed
- A couple of clippy warnings and compiler warnings about unused imports with
  latest rustc.
- Memory leak in builder for the `SelectStreams` event.
- Add parameter validity assertions to various `BufferList` and `Caps` APIs
  where these assertions were missing to avoid assertions in C.

### Added
- `StreamProducer::set_forward_preroll()` API to configure if the preroll
  buffer should be directly forwarded or not yet.

### Changed
- Remove nonsensical gstreamer-video test that fails with latest GStreamer main.
- Update to itertools 0.13.

## [0.22.4] - 2024-04-08
### Added
- Implement `From<glib::Value>` / `ToValue` for `gst_audio::AudioConverterConfig` and
  `gst_video::VideoConverterConfig`.

### Changed
- Fixed various 1.77 clippy warnings.

## [0.22.3] - 2024-03-19
### Changed
- Change `ges::CompositionMeta` position fields to `f64`s in correspondence
  with the C API.
- Change `gst_analytics::AnalyticsMtdRef::obj_type()` to an `Option` in
  correspondence with the C API.

### Added
- `gst::Fraction::new_raw()` and `from_integer()` const constructors.

## [0.22.2] - 2024-02-26
### Changed
- Update GStreamer gir files and add more new 1.24 API.

### Fixed
- Add `gst::Object` as parent class for various `gst_rtp` types.
- Handle all already queued messages in `gst::BusStream` instead of just new
  messages.

### Added
- Add `gst::CustomMeta::is_registered()`.

## [0.22.1] - 2024-02-13
### Changed
- Update GStreamer gir files and add more new 1.24 API.

### Fixed
- Make `AnalyticsODLocation` struct fields public.
- `MetaRefMut::upcast_mut()` returns a mutable reference now.

## [0.22.0] - 2024-02-08
### Changed
- Compatible with gtk-rs-core 0.19 / gtk4-rs 0.8.
- Update GStreamer gir files to latest (upcoming) 1.24 APIs.
- Various standalone functions were moved to separate modules or methods.
- `gst::Rank` is not implemented as an enum but as a struct with associated
  constants now.
- Optimized `gst::Buffer::from_slice()` and `Memory::from_slice()`
  implementations that have one heap allocation fewer.
- Various `gst::Buffer` and `gst::Memory` functions take ranges now instead of
  offset/size parameters.

### Added
- Bindings for `gst_gl::GLContext::thread_add()`, `GLFrameBuffer::draw_to_texture()`.
- New `gst_gl::GLVideoFrame` type that replaces `gst_video::VideoFrame` for
  GL-specific API, and comes with mostly the same interface.
- Basic gstreamer-tag bindings.
- `gst::Buffer:dump()` and `dump_range()` together with the same API on
  `gst::Memory` for hex-dumping the whole buffer/memory content.
- Implement `Clone` on `gst::MetaRef`.
- Bindings for `gst::Buffer::map_range_readable()` and its writable variant.
- Array-based accessor for `gst_video::VideoFrame` and
  `gst_audio::AudioBuffer` plane data.
- Support for handling custom authentication in `gstreamer-rtsp-server`.
- Accessors for various base class struct fields.
- Owned buffer getter for `AudioBuffer` / `VideoFrame`.
- `gst_rtp::RTPSourceMeta` bindings.
- `gst::macos_main()` bindings.
- gstreamer-analytics bindings.

### Fixed
- API typo in owned `gst::ReferenceTimestampMeta` reference getter.
- Allow variable expansion in `gst::loggable_error!` macro.
- `gstreamer-gl-*` crates can build docs again on stable.

### Removed
- `gst::Pad::caps()` property getter. Use `current_caps()` instead which does
  the same thing.
- Various deprecated APIs that were deprecated in previous releases.
- Getter for a mutable buffer reference from `AudioBuffer` / `VideoFrame` as
  that allowed invalidating the buffer map.

### Fixed

## [0.21.3] - 2023-12-18
### Added
- Update GStreamer gir files to latest (upcoming) 1.24 APIs.
- Add an example for writing subclasses with virtual methods.
- Add `gst::ClockTime::absdiff()` and same for similar types.

### Fixed
- In `Play` example, set bus to flushing before dropping `Play` instance.
- Add missing `docsrs` configuration for correct documentation generation.
- Make `gst_pbutils::element_properties` module public.
- Add missing `gst_audio::AudioFilterImpl::parent_allowed_caps()`.
- Fix assertions in `gst::Memory` copy/share/resize functions.

### Changed
- Update to itertool 0.12, pretty-hex 0.4.

## [0.21.2] - 2023-11-11
### Changed
- Update GStreamer gir files to latest (upcoming) 1.24 APIs.
- Update to latest gir code generator from the gtk-rs 0.18 branch.

### Fixed
- Big endian video format order is correct again.
- `gst::MetaRef::has_tags()` and `tags()` API actually works and works based
  on the tags of the concrete meta instance.
- `gst::MetaRef::tags()` returns strings with arbitrary lifetimes now because
  they're statically stored anyway.
- Fix another potential deadlock in `gst_utils::StreamProducer` when sending
  force-keyunit events.

### Added
- Bindings for `gst_video::VBIEncoder` and `VBIParser`.
- Accessors for the different `gst::PadProbeData` types on `PadProbeInfo`.
- `Default` impl for `gst::AllocationParams`.
- `From` / `TryFrom` implementations between formatted types (e.g.
  `gst::Bytes`) and `usize`.
- `gst::MetaRef::copy()` to copy metas from one buffer to another.
- `gst::ElementImpl::catch_panic_future()` to wrap a `Future` in such a way
  that panics are converted to GStreamer error messages and the element is
  marked as unusable.
 - `gst_gl::GLDisplay::handle()` to get a raw display handle.

## [0.21.1] - 2023-10-04
### Changed
- Update GStreamer gir files to latest (upcoming) 1.24 APIs.

### Fixed
- Use correct media links in the tutorials code.
- Fix a couple of new 1.72/1.73 clippy warnings.
- Fix description of gstreamer-validate crate.
- Copyright/license files for the gstreamer-gl were added.
- Ordering of raw video formats follows the rules of latest libgstvideo now.
- Fix potential deadlock in `gst_utils::StreamProducer` when sending
  force-keyunit events.

### Added
- `max-time` / `max-bytes` setters to `gst_app::AppSink` builder.
- `gst::CustomMeta::register_simple()`.

## [0.21.0] - 2023-08-08
### Changed
- Minimum supported Rust version is updated to 1.70.0.
- Compatible with gtk-rs-core 0.18.
- `gst::Bin::add_many()`, `remove_many()` and `gst::Element::link_many()`,
  `unlink_many()` are more generic now.
- `gst_base::Aggregator::src_pad()` returns an `AggregatorPad`.
- `gst::Bus::add_watch()` now returns a guard value that automatically removes
  the watch when it goes out of scope.
- `gst::Bin`, `Pipeline` and `Pad` constructors don't take the optional name
  parameter anymore but it can instead be provided via the builder API.
- `gst::Pad` and `GhostPad` builders inherit name from the pad template (or
  target) if possible and no other name is provided explicitly.
- The preroll samples and selected sticky events are forwarded to `StreamProducer` consumers.

### Added
- Support for the upcoming GStreamer 1.24 APIs.
- Support for inline variable names in format strings for error/warning/info
  messages.
- Methods for converting between floating point seconds and `gst::ClockTime`.
- Various additions to the gst-validate bindings.
- `Display` implementations for error/warning/info messages.
- More useful `Debug` implementations for messages, events and queries and
  `gst_pbutils::DiscovererInfo` related structs.
- API for listing/checking `gst::Meta` tags.

## [0.20.7] - 2023-07-05
### Fixed
- Fix `wait-for-eos` property name string in `appsink`.
- Fix various memory leaks in `BaseTransform` subclassing bindings.
- Mark some GES APIs as `Send+Sync`.

### Added
- Implement `DiscovererInfo::debug()` and on related structs.
+ Add subclassing bindings for `GESFormatter`.

## [0.20.6] - 2023-06-06
### Added
- Getter for the `gst_rtsp_server::RTSPContext` URI field.

### Fixed
- `gst_pbutils::DiscovererStreamInfo::stream_id()` can return `NULL`. This is
  mapped to the empty string for this release to keep backwards compatibility.
- `gst_pbutils::DiscovererStreamInfo` iterator methods can be called on any
  subclass directly now without casting.
- Debug logs use the actual function name against instead of the name of a
  closure generated by the log macros.

### Changed
- Minor performance improvements to debug logging.

## [0.20.5] - 2023-04-22
### Added
- `glib::HasParamSpec` impl for miniobjects to allow using them with the
  properties derive macro.
- `Default` impl for `gst_player::Player`.

## [0.20.4] - 2023-04-07
### Fixed
- Work around `gst_webrtc::WebRTCICE::add_candidate()` API breakage in 1.24.

### Changed
- Reduce size of `gst_audio::AudioBuffer` and `gst_video::VideoFrame` by a
  factor of two by not storing an unnecessary copy of the audio/video info.

## [0.20.3] - 2023-03-14
### Fixed
- `gst::ParamSpecArray` uses the correct `glib::Type` now.
- Work around accidental ABI breakage in 1.18 gst-rtsp-server `GstRTSPClient`.

### Added
- Document `gst_utils::StreamProducer::forward_eos()` default value.

## [0.20.2] - 2023-02-21
### Added
- `glib::HasParamSpec` impl for `gst::ClockTime`
- `Default` impl for `gst_play::Play`
- Constructors for non-raw `gst_audio::AudioCapsBuilder` / `gst_video::VideoCapsBuilder`

## [0.20.1] - 2023-02-13
### Fixed
- Fix memory leaks when converting a `gst_audio::AudioBuffer` or
  `gst_video::VideoFrame` to a `gst::Buffer` or FFI type.

## [0.20.0] - 2023-02-10
### Fixed
- Make `gst_gL::GLDisplay::create_context()` `other_context` parameter optional.
- Make allocation query caps optional.

### Added
- Conversions between `gst::Signed<T>` and `T` and signed integer types.
- Bindings for the object lock via `gst::Object::lock()`.
- Various `FromIterator`, `Extend` and `From` impls for creating `Caps`,
  `Structure`, `Buffer`, `BufferList`, `CapsFeatures` and other types.
- `PartialEq` impls between owned/borrowed miniobjects/structures.
- API for appending items to `gst::Array` and `gst::List`.

### Changed
- Compatible with the 0.17 gtk-rs release.
- Updated minimum supported Rust version to 1.64.
- Require GStreamer 1.22.0 or newer when enabling the `v1_22` feature.
- Require the object lock to be taken for various `gst_gl::GLDisplay` methods.
- Renamed `gst::TagSetter::add()` to `add_tags()` to avoid name conflict with
  `Bin::add()`.
- Mark various un-extendable enums as exhaustive.
- Make use of `glib::GStr` and related API in caps, structure, tags and
  logging API to reduce temporary string allocations.
- Various code optimizations to reduce generated code size and allow more
  optimal code to be generated.
- Reduce size of various types, including reduction of `gst_audio::AudioInfo`
  from 832 to 320 bytes.
- Use actual function name instead of module name in log output.
- Change `gst_utils::StreamProducer` API to forward buffers by default and
  allow temporarily discarding via new `set_discard()` function.

## [0.19.8] - 2023-02-09
### Changed
- Update GStreamer .gir files to 1.22.0 release.

### Fixed
- Marked `gst::MessageType` as non-exhaustive.

### Added
- Added bindings for `gst::Message::structure_mut()`.
- Added subclassing support for `gst_allocators::FdAllocator` and `DmabufAllocator`.

## [0.19.7] - 2023-01-19
### Fixed
- Work around the possibility that the caps in the allocation query can be
  `NULL` by returning any caps for now. This will be handled properly with a
  minimal API change in the 0.20 release.

## [0.19.6] - 2023-01-18
### Fixed
- The `AppSrc` and `AppSink` builders now assert that GStreamer is initialized
  before creating an instance.

## [0.19.5] - 2022-12-27
### Fixed
- Clear video frame values when mapping as GL texture to avoid out of bounds
  reads when trying to access the GL texture as raw video frame.
- Allow returning `Handled` from `BufferList` pad probes.

### Changed
- Update GStreamer .gir files to latest 1.21 git.

## [0.19.4] - 2022-12-16
### Added
- Subclassing bindings for `gst_audio::AudioFilter`.

### Fixed
- Various new clippy warnings.

### Changed
- Update GStreamer .gir files to 1.21.3 release.

## [0.19.3] - 2022-11-28
### Added
- `FromIterator<Caps>` and `Extend<Caps>` for `Caps`.
- `PartialEq` impls between owned/borrowed miniobjects/structures.

### Fixed
- Sticky event ordering for instant-rate-change.

### Changed
- Updated GStreamer .gir files to post 1.22.2 release.

## [0.19.2] - 2022-11-13
### Added
- Subclassing support for `gst::Allocator`.
- `gst_gl::GLBaseMemory::context()` to retrieve the GL context used by the
  memory.

### Changed
- Updated GStreamer .gir files to 1.22.2 release.

### Fixed
- `gst::Allocator::register()` does not cause use-after free with
   GStreamer < 1.20.5 anymore.
- Don't generate version constants in `gstreamer-editing-services-sys` as they
  are useless and change with every update.

### Changed
- Fixed various new clippy warnings.

## [0.19.1] - 2022-10-24
### Changed
- Make it possible to use objects not just as reference in the logging macros.

## [0.19.0] - 2022-10-22
### Added
- Builders for element construction. `gst::ElementFactory::make()` returns a
  builder now that allows to easily set the name or any other property at
  construction time. The old API is available as `make_with_name()`.
- Builders for `Bin` and `Pipeline` as well as a `Default` trait
  implementation to simplify object construction.
- Builders for `appsrc` and `appsink`, which allow type-safe construction of
  both elements while also allowing to easily set all their properties at
  construction time.

- Builders for the GStreamer-specific fraction/array param/property specs.

- Infrastructure for casting between `gst::Memory` subtypes/supertypes, and
  make use of it for GL memory.
- Bindings for the `gstreamer-allocator` library with support for file
  descriptor-based and DMABUF memory.

- Complete bindings for `gst_video` `Navigation` events.

- Constructors for error/warning/info messages with a pre-built `glib::Error`.
  This also leads to some minor simplification of the existing API.

- Accessors for static pads of various base classes for making accessing them
  cheaper and less error-prone than getting them by name.

- Builder for pad templates.

- Static PTP clock API for statistics, initialization and deinitialization.

- New `gstreamer-utils` crate that currently contains only a `StreamProducer`
  API. This allows building 1:N bridges between live pipelines via `appsink` /
  `appsrc` elements.

- Bindings for the new `gstreamer-play` library that was added in 1.20.

- `gst::Caps::new_empty_simple()` to create caps without fields and just a
  name.

- `gst_audio::AudioCapsBuilder` and `gst_video::VideoCapsBuilder` for building
  (possibly) unfixed raw audio/videos caps with typed setters for the various
  fields. This makes it impossible to mix up types and e.g. use an `u32`
  instead of an `i32` for the width of video caps.

- `gst::Buffer::ptr_eq()` to compare buffers by pointer instead of doing a
  deep comparison, and also `ptr_eq()` on all other miniobject types.

- Accessors for `gst_webrtc::WebRTCICECandidateStats` fields.

- Bindings for the `gstreamer-validate` API.

- Subclassing bindings for `gst_audio::AudioVisualizer` base class for easily
  writing audio visualization elements.

- `gst_pbutils::EncodingProfile` API for element properties.

- Support for returning buffer lists from `BaseSrc` / `PushSrc` subclasses.

- Support for implementing `gst::Bin::do_latency()`.

- Minimal bindings for the `gstreamer-mpegts` library.

### Fixed
- Signature for `gst_base::Aggregator::connect_samples_selected()` to remove
  unnecessary generic parameter and make it straightforward to use.

- Various APIs had optional parameters/return types corrected to match the C
  API more closely.

- Logging does not evaluate its arguments anymore if the debug category is not
  enabled or below the current threshold.

- Registering custom metas is now possible without transform function.

- `gst::subclass::ElementImpl::request_new_pad()` signature uses a `&str`
  instead of an owned `String` now.

### Removed
- `fragile` dependency and instead use the same functionality from `glib`.

- `gst_audio::AudioAggregator` `ignore_inactive_pads` property, which was
  duplicated from the `Aggregator` base class.

### Changed
- Compatible with the 0.16 gtk-rs release.
- Updated minimum supported GStreamer version from 1.8 to 1.14.
- Updated to the latest GStreamer 1.22 APIs while still supporting up to
  GStreamer 1.14. Any new 1.22 APIs might still change until the stable 1.22
  release.
- Updated minimum supported Rust version to 1.63.

- In `EventView` / `QueryView`, getters that return references now return
  references that can outlive the view and are only bound by the lifetime of
  the corresponding event/query.
- In addition `Query`, `Event` and `Message` views are implemented more
  consistently now, which makes them easier to use and as a side effect allows
  to pass e.g. more strongly typed queries to functions that only accept a
  single query type.

- Various improvements to `gst::ClockTime`, `gst::format::Bytes`, `gst::format::Signed`
  and related types and their usage in the API, which should make its use from
  applications easier and less error-prone. Check the `gst::format`
  module-level documentation for details.

- `gst::StreamsSelected` event builder takes the selected streams as iterator
  instead of slice.

- For consistency with other macros the `gst` prefix of the logging macros was
  also removed.

- Various iterator implementations were added and the existing ones were
  optimized by implementing more specialized traits and custom implementations
  for a couple of iterator functions.

- GStreamer initialization safety checks were optimized.

- `gst::Bus::post()` takes ownership of the passed messages like the C API.

- Better and easier to read `Debug` impls for `Caps`, `TagList`,
  `Structure` and `Promise`.

- `ser_de` feature was renamed to `serde`.

- `gst::Tracer` implementations get result enums passed as `Result`s now
  instead of single enums.

- `gst::Pad`, `ProxyPad`, `GhostPad` default functions are all associated
  functions instead of methods now to avoid conflicts between multiple types
  with the same method.

- `Pad` tasks are catching panics from the task function and if the parent of
  the pad is an element then the panic is converted into an error message and
  the task is simply stopped. Otherwise the panic is rethrown.

## [0.18.8] - 2022-04-26
### Added
- Bindings for `RTPBasePayload` and `RTPBaseDepayload`.
- Accessors for `RTPBuffer` buffer.
- Bindings for `RTPBuffer` length calculation API.
- More complete `gst::Task` bindings.

### Fixed
- Export `gst::subclass::TaskPoolFunction`.

## [0.18.7] - 2022-04-04
### Added
- Bindings for `VideoAggregator` and the `VideoAggregatorPad`s.
- Bindings for `AudioAggregator` and the `AudioAggregatorPad`s.
- Bindings for `TaskPool`.
- Various helper functions for `VideoFormatInfo`, `VideoInfo` and
  `VideoFrame`.

## [0.18.6] - 2022-03-08
### Fixed
- Require `Send` and not `Sync` for the values of an `gst::Array` / `gst::List`.

### Changed
- Simplify and speed up log message string construction

## [0.18.5] - 2022-02-20
### Changed
- Require GStreamer 1.20.0 at least when building with `v1_20`. Earlier
  versions were already going to fail due to API mismatches before.

### Added
- `gst::BufferPool` subclassing support.
- `Debug` impl for `gst::MiniObject`.
- `gst_rtsp_server::RTSPOnvifServer` and related API, including subclassing
  support.

### Fixed
- Handle empty slices correctly at the FFI layer.
- `MiniObjectRef::downcast_ref()` and similar functions return the correct
  type now. While this is an API change, the previous API would've never
  worked.

## [0.18.4] - 2022-02-04
### Changed
- Update gir files to GStreamer 1.20.0 release.

### Added
- `gst_video::VideoCodecFrame::input_buffer_owned()` for getting an owned
  reference.

### Fixed
- All documentation links in the `README.md`s are correct again.

## [0.18.3] - 2022-01-31
### Added
- `Default` implementation for `gst_video::VideoOverlayComposition` when
  targeting GStreamer 1.20.
- `gst_video::VideoOverlayComposition::add_rectangle()` in addition to the
  addition of all rectangles via an iterator during construction.
- Subclassing support for `gst_rtp::RTPHeaderExtension`.
- `gst_webrtc::WebRTCError` for programmatically handling WebRTC errors.

### Fixed
- `gst_rtp::RTPHeaderExtension` has `gst::Element` set as parent class now.
- Global functions are re-exported from the `gst_rtp` crate root.

### Changed
- GIO-style async operations in GES no longer need `Send`-able closures.

### Removed
- `fragile` is no longer a dependency and instead the corresponding GLib API
  is used.

## [0.18.2] - 2022-01-24
### Added
- `glib::FromValue` for mini object references.
- Bindings for `gst::DebugCategory::get_line()`.

## [0.18.1] - 2022-01-18
### Fixed
- `Message::view()` also handles the redirect message now.
- `Message` and `Query` view variants that return references now borrow again
  from the underlying query and not the view enum, allowing to use them in a
  wider scope.

### Changed
- All miniobjects, `VideoTimeCode`, `Structure` and `CapsFeatures` are marked
  as `#[repr(transparent)]` now to ensure that their memory representation is
  exactly the underlying raw pointer.

## [0.18.0] - 2022-01-16
### Added
- `gst_rtp::RtpHeaderExtension::read()` and `write()`.
- `gst::ElementMetadata` has a `const` constructor now.
- `gst_rtp::RtpBuffer` API works on buffer references instead of plain buffers
   for statically enforcing writability and usage in more places.
- `gst_video::VideoCodecAlphaMeta` and `gst::CustomMeta`.
- `gst::MiniObject` for generically passing around mini objects instead of
   their concrete types.
- `gst_app::AppSink` `new-event` callback and `pull_object()` function.
- `gst_pbutils::PbUtilsCapsDescriptionFlags` and
  `pb_utils_get_caps_description_flags()`.
- `gst_rtp::RtpBuffer::remove_extension_data()`.
- `gst_video::VideoDecoder` subframe API.
- `gst_webrtc::WebRTCSCTPTransport`.
- `gst::ElementFactory` `create_with_properties()` / `make_with_properties()`.
- `gst_video::VideoContentLightLevel` and `VideoMasteringDisplayInfo` for HDR
   signalling.
- Lots of missing `GES` API.
- `gst::AllocationParams` and support in the allocation query.
- `propose_allocation()` and `decide_allocation()` support in the various base
   classes.
- `Iterator` implementation for `gst_video::VideoOverlayComposition`.
- `Extend`, `IntoIterator` and `FromIterator` implementations for `Buffer`,
  `Caps`, `BufferList`, `CapsFeatures`, `StreamCollection` and `Structure` for
   more natural Rust APIs.
- `instant-rate-change` events/messages bindings.
- Support for arithmetic operations on `Option<gst::ClockTime>` and related
  types.
- `gst_video::ColorBalance`.
- `gst::MetaFlags`.
- `gst_base::Aggregator::set_position()`.
- Convenience getters for `gst::ElementFactory` and
  `gst::DeviceProviderFactory` metadata.
- `gst_rtp::RtpBuffer::set_padding()`, `get_padding()` and `payload_mut()`.
- `#[must_use]` to many types and functions.
- `gst::Event`, `gst::Message` and `gst::Structure` `has_name()`.
- `gst_video::Navigation` subclassing support and API improvements.
- `gst::Structure` and `gst::Caps` `foreach()`, `map_in_place()` and
  `filter_map_in_place()`.
- `gst_gl::GLBufferPool` and various GL constants and functions.
- `gst_pbutils` codec utils APIs.

### Fixed
- `gst_base::BaseTransform::prepare_output_buffer()` correctly reflects buffer
  writability.

### Changed
- Compatible with the 0.15 gtk-rs release.
- Updated to the latest GStreamer 1.20 APIs while still supporting up to
  GStreamer 1.8. Any new 1.20 APIs might still change until the stable 1.20
  release.
- Update all code to the Rust 2021 edition. This causes no user-facing
  changes.
- `gst::Sample::segment()` returns a reference instead of a copy of the
  segment.
- `gst::Object::set_property_from_str()` returns a `Result` now instead of silently
  failing like the C version.
- Allow handling passed in buffers in `gst_base::PushSrc::create`.
- Allow passing in `None` in `gst_player::Player::set_uri()`.
- Use `[[f32; 4]; 4]` instead of `[f32; 16]` for affine transformation matrix.
- `gst::Pad::sticky_event()` statically gets the event of the requested type
  instead of requiring to match on it afterwards.
- Clean up `gst_pbutils` `EncodingProfile` API to be harder to misuse and less
  confusing.
- Various `gst::Array`, `gst::List`, `gst::IntRange` and `gst::Fraction` API
  improvements that should reduce some friction.
- Directly generate `NUL`-terminated C strings in debug log API instead of
  having multiple allocations per message.
- Various functions return `glib::SList` and `glib::List` now to avoid copying
  into a `Vec` if only iteration is needed.
- `gst::ChildProxy` API is more consistent with object property API.
- Improved `gst::Buffer::foreach()`, `gst::Pad::sticky_events_foreach()` and
  `gst::BufferList::foreach()` APIs.
- Don't post error messages from `propose_allocation()` and
  `decide_allocation()`.

## [0.17.4] - 2021-09-13
### Added
- Add constructor for device provider metadata.

## [0.17.3] - 2021-08-23
### Fixed
- `gst::Value::deserialize()` takes the target type as parameter now. This is
  technically an API change but the function would've never worked previously.

### Added
- The release date-time parameter to `gst::plugin_define!` is optional now
  like in the C version of the macro.
- Bindings to `gst::Tracer` and `gst::TracerFactory` for allowing to implement
  custom tracers in Rust.
- Bindings for the new `gst::Value::deserialize_with_psec()` function from
  GStreamer 1.20.
- serde `Serialize`/`Deserialize` impls for `gst::PadDirection`,
  `gst::PadPresence`, `gst::URIType` and `gst::Rank`.

## [0.17.2] - 2021-08-05

### Fixed
- Various new clippy warnings.
- Compilation of `gstreamer-audio` on big-endian platforms.

### Added
- Support for 1.20 `Gap` event `GapFlags`.
- Support for 1.20 `Structure::serialize()` / `Caps::serialize()`.

## [0.17.1] - 2021-07-13

### Fixed
- Store 1.19 as plugin version when building plugins with `v1_20`. Otherwise
  plugins fail to load with GStreamer versions below 1.20.0.
- Fix documentation for `gst::Element::request_pad_simple()` to actually show
  up.

## [0.17.0] - 2021-06-28

### Fixed
- Use `#[repr(transparent)]` where it is more correct and remove unneeded
  `#[repr(C)]` annotations.
- Don't provide direct access to the logged object in logging functions as the
  object might currently be finalized and might be unsafe to access.
- Moved X11/EGL/Wayland-specific GL APIs into their own crates instead of
  having them inside gstreamer-gl and behind feature flags. This simplifies
  conditional usage of them in applications.
- Various nullability issues: parameters and return values that should've been
  or shouldn't have been nullable were fixed.
- Print source object correctly in `gst::Message` `Debug` impl.
- `gst_rtsp_server::RTSPServer::attach()` is fallible.
- `gst::ElementFactoryListType` is a proper bitflags type now instead of
  generic `u64`.
- `gst::PluginFeature::load()` returns the same type as the one passed in.
- Value returned by `gst::PromiseFuture` can no longer be freed while still
  in scope.
- Only assign to `GError**`s in subclassing code if they're not `NULL`.

### Added
- Bindings for the GStreamer Controller library and the corresponding core API.
- Subclassing support for `gst_player::PlayerVideoRenderer`.
- `gst::PARAM_FLAG_CONTROLLABLE` and related bindings.
- `gst_video::VideoOrientation` and `VideoOrientationMethod` bindings.
- Support for removing pad probes from inside the pad probe callback.
- `gst_check::Harness::pull_until_eos()` bindings.
- `ges::TransitionClip` and `OperationClip`.
- Bindings for `gst_gl::GLMemory` and related APIs.
- Subclassing support for `gst_gl::GLFilter` and `gst_gl::BaseSrc`.
- `gst::TagList::remove()`.
- `gst::CapsFeatures` and `gst::Structure` API based on `glib::Quark`s instead
  of strings.
- Subclassing support for `gst_video::VideoFilter`.
- Bindings for various new 1.20 APIs: `gst_app::LeakyType`,
  `gst_video::VideoDecoderRequestSyncPointFlags`,
  `gst_rtp::RTPHeaderExtension`, `gst_audio::AudioLevelMeta`,
  `gst_webrtc::WebRTCKind` and various other new flags/enum types.
- Subclassing support for `gst_rtsp_server::RTSPMountPoints`.

### Removed
- Deprecated APIs in 0.16.
- Don't declare that `gst_app::AppSink` and `AppSrc` inherit from
  `gst_base::BaseSink` and `BaseSrc` to avoid exposing API that is meant for
  subclasses to applications.
- `gst_app::AppSrc` and `AppSink` signals that are also covered by the
  callbacks. The callbacks are more flexible and have lower overhead.
- Duplicated getters/setters for `gst_base::BaseSink` and `BaseTransform`
  properties.

### Changed
- Compatible with the 0.14 gtk-rs release.
- Updated to the new GStreamer 1.20 APIs while still supporting up to GStreamer
  1.8. Any new 1.20 APIs might still change until the stable 1.20 release.
- FFI and safe high-level bindings are in the same repository now and use the
  same version numbers.
- The .gir files are shared with gtk-rs and the GStreamer-specific ones are in
  a separate git submodule.
- Update all code to the Rust 2018 edition. As part of this, most macros lost
  their `gst_` prefix.
- Re-export dependency crates from the different preludes.
- Getter functions don't have a `get_` prefix anymore and GObject property
  accessors don't include the `_property_` part in the middle of their
  function names anymore. Applications developers should use
  [`fix-getters-calls`](https://crates.io/crates/fix-getters-calls) to ease
  migration of their applications.
  Use [`fix-getters-def`](https://crates.io/crates/fix-getters-def) if you also
  want your `get` functions definition to comply with the API standards applied
  in this release.
- Lots of changes to the subclassing API. Check the various elements in
  [gst-plugins-rs](https://gitlab.freedesktop.org/gstreamer/gst-plugins-rs)
  for examples.
- Major improvements to the documentation infrastructure and generated
  documentation.
- `gst::ClockID` bindings are refactored to use different types for
  single-shot and periodic clock ids, which makes misuse harder.
- `gst::ProxyPad` extension trait uses trait functions instead of associated
  functions now for usability reasons.
- Use `Result<gst::FlowSuccess, gst::FlowError>` for overriding flow returns
  from pad probes.
- `gst_video::VideoInfo::align()` returns a `Result` instead of a `bool`.
- Use actual error types instead of `()` in `gst_sdp` APIs.
- `Display` impl for `gst::ClockTime` provides better human-readable strings.
- `gst::Element::link_filtered()` and `link_pads_filtered()` takes a
  non-optional caps now. That's easier to use and for not providing caps the
  non-filtered variants of the functions exist.
- Replace various manual bindings with auto-generated ones.
- `gst::Element::get_request_pad()` is replaced by `request_pad_simple()` as a
  simpler version of `request_pad()` and in accordance with the deprecation in
  GStreamer 1.20.
- `gst::ClockTime` and APIs working on it were changed to make possibility of
  using `GST_CLOCK_TIME_NONE` expressed in the type system.
  `Option<gst::ClockTime>` can be `None` while `gst::ClockTime` is always a
  valid time.

## [0.16.7] - 2021-02-13
### Fixed
- Usage of the logging system with a GStreamer library with the logging system
  compiled out does not crash any longer.
- Double-free in `gst_video::VideoTimeCode` API when converting between
  validated and unvalidated timecodes.

### Added
- `gst::Element::get_current_state()` and `get_pending_state()` convenience APIs.
- `gst_audio::AudioConverterConfig` for setting the configuration on e.g. the
  `audiomixer` element. The low-level `AudioConverter` API is still not
  included in the bindings.

## [0.16.6] - 2020-12-20
### Fixed
- `VideoTimeCodeInterval`'s `Ord` and `PartialEq` implementations compare
  against the correct fields now.
- `SDPMessage::medias_mut()` iterator does not crash any longer.

### Added
- `PartialEq` and `Eq` implementations on `VideoAlignment`.
- Alignment API for `VideoMeta` and `get_plane_height()` / `get_plane_size()`.
- `VideoInfo::align_full()`.

## [0.16.5] - 2020-11-23
### Fixed
- Make sure to use `$crate` in more macros to allow them to work without
  anything special in scope already.
- Update documentation location.
- Don't panic if C code stores invalid seqnums in events and the seqnum is
  used directly or via the `Display` impl.
- Fix docs build for some crates on docs.rs.
- Fix `Debug` impl for `gst_video::VideoTimeCode` to print the correct type
  name.
- Fix plugin version to be 1.18 instead of 1.17 when compiling a plugin with
  `v1_18`.

### Added
- Event handling support in pad probes, that is returning
  `PadProbeReturn::Handled` for events.
- `EventRef::get_structure_mut()` getter that allows changing the events'
  structures.

### Changed
- Remove unnecessary `PhantomData` markers and use `repr(transparent)` instead
  of `repr(C)` where it is more correct.

## [0.16.4] - 2020-10-09
### Fixed
- Correctly implement `ExactSizeIterator` on the `AudioFormat` and
  `VideoFormat` iterators. Previously they returned the overall size instead
  of the remaining size, and they didn't implement `Iterator::size_hint()`.
- Don't implement `ExactSizeIterator` on the buffer `gst::Meta` iterator. The
  overall length is not known easily and the implementation would've simply
  panicked in the past.

### Added
- `gst::ClockID::wait_async_stream()` for async integration for clock waiting.
- `From` / `TryFrom` impls for converting between `gst::ClockTime` and
  `std::time::Duration`.

## [0.16.3] - 2020-09-08
### Fixed
- Reset vfuncs if calling `BaseTransformClass::configure()` multiple times.
- Fix `gst::debug_remove_default_log_function()` to actually remove the
  default log function.

### Added
- Some more new APIs added in 1.18.
- API for getting an owned buffer from a readable `gst_video::VideoFrame` /
  `VideoFrameRef`.

### Changed
- Updated bindings to 1.18.0. This stabilized GStreamer 1.18 support and any
  API behind the "v1_18" feature is considered stable now.
- Factor out some common code from `gst::Pad::ProbeInfo` code. This reduces
  the code generated for each pad probe considerably.
- Update paste dependency to 1.0 and pretty-hex to 0.2.

## [0.16.2] - 2020-07-27
### Fixed
- Use correct pointer for the plane data in `gst_audio::AudioBuffer`.

### Added
- Add `gst::GhostPad` convenience constructors that take a target pad, similar
  to the ones that existed in 0.15 and before.
- Add `gst::parse_bin_from_description_with_name` that allows setting a name
  for the created bin without having to use unsafe code in application code.

## [0.16.1] - 2020-07-10
### Fixed
- Allow calling `gst::DebugCategory::new()` before `gst::init()` again.

## [0.16.0] - 2020-07-06
### Added
- Updated bindings to 1.17.2, adding experimental 1.18 support. This can be
  opted-in via the "v1_18" feature flag but there might still be API changes
  in the newly added API.
- `gst::MemoryRef::dump()` for dumping contents of a memory.
- `gst::Bus::stream()` instead of a custom constructor on the `BusStream`.
- Use more accurate types for `Seqnum`, `GroupId` and `MetaSeqnum`. These are
  now proper wrapper types instead of plain integers, which makes misuse
  harder.
- Provide `TryFrom` impls for conversion between `glib::DateTime` and
  `gst::DateTime`.
- Add `get_allocator()` functions to `gst_base::{Aggregator, BaseTransform,
  BaseSrc}`, and allow overriding `BaseSrc::alloc()`.
- Add subclassing bindings for `gst_base::PushSrc`.
- Add new `gst::BufferCursor` API that allows to handle a buffer as `Read`,
  `Write` and `Seek` and accesses the underlying memories of the buffer
  individually without mapping them all together.
- Add `gst::Plugin::get_plugin_name()`.
- Support for `gst_video::VideoAFDMeta` and `VideoBarMeta`.
- API for getting all / iterating over all `gst_audio::AudioFormat` and
  `gst_video::VideoFormat`.
- Bindings and subclassing bindings for `gst_video::VideoSink`.
- `gst::Pad` can be constructed via the builder pattern and `gst::PadBuilder`
  now, which allows to safely set the pad functions and various other fields
  during construction. The `PadBuilder` works on any `gst::Pad` subclass and
  also has special support for `GhostPad`s by allowing to set pad functions of
  the proxy pad.
- `gst::Message`, `gst::Event` and `gst::Query` type constructors are now on
  the specific target type instead of various `new_XXX()` functions on the
  basic type. E.g. `gst::message::Eos::new()`.
- Support for overriding `gst_audio::AudioSrc/Sink::reset()`.
- Support for overriding `gst_base::BaseParse::stop()`.
- Support for overriding `gst::Element::post_message()`.
- Added bindings for `gst::BufferList::foreach()` and `foreach_mut()`.
- Added bindings for `gst::Buffer::foreach_meta()` and `foreach_meta_mut()`.

### Fixed
- Allow using any `glib::Object` as target object for logging instead of just
  `gst::Object`.
- Remove restriction API from `gst_pbutils::EncodingContainerProfile`. They
  are supposed to be used only with the other encoding profiles.
- Return `&'static str` for various `gst::StructureRef` functions where the
  string is backed by a `glib::Quark`.
- Fix various `gst::DateTime` functions to actually return `Option`s.
- Add support for filling in a buffer passed to the `gst::Pad` getrange
  function, allow passing one in into `get_range()` and `pull_range()` and
  provide the corresponding API on `gst_base::BaseSrc` too.
- Allocator in audio/video `Decoder` base classes is optional and can return
  `None`.
- `gst_video::ValidVideoTimeCode::add_interval()` always returns a valid
  timecode again.
- Allow resolving a `gst::Promise` with `None` and also handle that correctly
  in the callback. This is allowed by the API.
- Allow calling various debugging related functions before `gst::init()`.
- Various enum/function versions were fixed to only show up if the
  corresponding version feature is enabled.
- `gst::Pad` function setters are marked unsafe now as changing the functions
  is not thread-safe.
- Remove `gst::Object::set_name()` as changing the name after construction
  generally causes problems and is potentially unsafe.
- Remove `gst::Pad::set_pad_template()` as changing the pad template after
  construction is generally unsafe.
- `gst::Pad::stream_lock()` borrows the pad now instead of taking a new
  reference.
- Unimplemented `Jitter` and `Buffer` queries were removed from the bindings.
  These are not implemented in C and only have a type registered.
- Various `LAST`, `NONE` variants of enums and flags were removed as these
  only make sense in C.
- Call the parent impl of various vfuncs that were omitted before to not
  require further subclasses of them to implement them but automatically call
  the parent ones.

### Changed
- Use `NonZeroU64/U32` for various ID types to allow further optimizations.
- Use `thiserror` crate for deriving error types.
- Switch from `lazy_static` to `once_cell`.
- Change various miniobject functions like `gst::Caps::append()` from taking
  the object by value to modifying it internally. This makes them easier to
  use and only applies to functions that are defined on the non-reference type
  and take ownership of the values passed in.
- Use `mem::ManuallyDrop` instead of `mem::forget()` everywhere.
- Replace most `mem::transmute()` calls with safer alternatives.
- `gst:StreamCollection` API was changed to the builder pattern for
  construction as the collection must not be changed after construction.
- `gst::ProxyPad` default functions are plain functions on `ProxyPad` now
  instead of trait functions to allow easier usage of them.
- Use proper error types in various `TryFrom` impls.
- `gst_video::VideoMeta::add()` returns a `Result` now instead of panicking.
- Various constructors were renamed from `new_with_XXX()` and `new_from_XXX()`
  to the more idiomatic `with_XXX()` and `from_XXX()`.
- Miniobject bindings are simplified now and there is no `gst::GstRc` type
  anymore, instead everything is directly implemented on the concrete types.
  As part of this the `gst::MiniObject` trait was also removed as it was
  unneeded now.

## [0.15.7] - 2020-06-08
### Fixed
- Allow multiple filter types per process with `gst::Iterator::filter()`.
- Check that `VideoInfo` is valid when creating a `VideoFrame`.
- Don't potentially dereference a `NULL` pointer when getting the format
  from an invalid `VideoInfo` or `AudioInfo`.
- Don't unmap borrowed `VideoFrameRef`s.

### Added
- `gst::ProtectionMeta`, `gst_video::VideoAffineTransformationMeta`,
  `VideoCropMeta` and `VideoRegionOfInterestMeta` bindings.
- Various new `gst_rtp::RTPBuffer` methods.
- `gst_audio::audio_buffer_truncate()`, `AudioMeta` and `AudioBuffer`
  bindings.

## [0.15.6] - 2020-05-28
### Fixed
- Assert that the data passed to `VideoCaptionMeta::add()` is not empty.
- Don't store strong references to the object in the bus, appsink and appsrc
  futures `Stream` / `Sink` adapters. This would keep them alive unnecessarily
  and would prevent the `Stream` / `Sink` to ever "finish" on its own.
- Handle receiving a `None` reply in the change function of `gst::Promise`.
  This is apparently valid. For backwards compatibility reasons this is
  currently replaced with an empty structure but in 0.16 the API will
  explicitly handle `None`.

### Added
- `gst::Stream::debug()` and `gst::StreamCollection::debug()` for converting
  into a structured string with the actual contents of each.
- `gst::Structure::from_iter()` and `gst::Caps::from_iter()` to create
  structures/caps from iterators.
- `gst::Event` support for getting/setting the `gst::Stream` in the
  `StreamStart` event.
- `gst_video::calculate_display_ratio()` and `::guess_framerate()`.
- Various video related `gst::CapsFeatures` in `gst_video`.
- `TryFrom`/`From` impls for converting between `gst::Structure` and
  `gst_video::VideoConverterConfig`.
- Various `glib::Value` trait impls for `SDPMessage`, `StructureRef`,
  `CapsFeatureRef` and all borrowed variants of miniobjects to be able to
  work with the borrowed, non-owned variants when handling `glib::Value`s.

## [0.15.5] - 2020-05-03
### Fixed
- Revert: Allow logging any `glib::Object` and not just `gst::Object`. This
  broke API in subtle ways and needs to wait until 0.16
- Replace `%` in log output with `%%` to prevent accidental C formatting
- Add missing manual traits to the documentation

### Added
- `BufferRef::peek_memory_mut()` to give a mutable reference to a given memory
- Different iterators for iterating over the memories of a buffer
- Support for `gst_audio::AudioClippingMeta`
- `gst::Plugin::get_plugin_name()` was added
- `gst::Element::get_current_clock_time()` and
  `gst::Element::get_current_running_time() helper functions
- `gst::State` and `StateChange` API for calculating next/previous state and
  convert from/to the components of a state change

### Changed
- Use `mem::ManuallyDrop` instead of `mem::forget` everywhere

## [0.15.4] - 2020-03-09
### Fixed
- Allow logging any `glib::Object` and not just `gst::Object`
- Fix floating reference handling in `RTSPMedia::take_pipeline()`
- Hold `GMutex` guards for the remainder of the function and warn if they're
  directly dropped
- Work around empty/any caps handling bugs in `Caps::fixate()`

### Added
- Add `BaseTransform::prepare_output_buffer()` subclassing support
- `RTSPServer`, `RTSPClient`, `RTSPMedia` and `RTSPMediaFactory` subclassing
  support
- Handle panicking in `appsrc`/`appsink` callbacks by posting an error message
  instead of killing the process

## [0.15.3] - 2020-02-15
### Fixed
- `UniqueFlowCombiner::clear()` should take a mutable reference.
- `AudioStreamAlign` doesn't require mutable references for getters anymore.
- Don't use bool return value of `gst_video_info_set_format()` and
  `gst_video_info_align()` with GStreamer < 1.11.1 as it returned void back
  then. We'd otherwise use some random value.
- Make `VideoInfo::align()` is available since 1.8.
- Fix changing/clearing of `AppSrc`, `AppSink` callbacks and `Bus` sync
  handler. Before 1.16.3 this was not thread-safe and caused crashes. When
  running with older versions changing them causes a panic now and unsetting
  the bus sync handler has not effect. With newer versions it works correctly.

### Added
- Add `Clone` impls for `BufferPoolConfig` and `PlayerConfig`.
- Add `VideoConverter` bindings.
- Add `Future`s variant for `gst::Promise` constructor.
- Add `Future`s variant for `gst_video::convert_sample_async()`.
- Add `submit_input_buffer()`, `generate_output()`, `before_transform()`,
  `copy_metadata()` and `transform_meta()` virtual method support for
  `BaseTransform`.
- Add `AppSink` `Stream` adapter and `AppSrc` `Sink` adapter for integrating
  both into Rust async contexts.

### Changed
- More generic implementations of `VideoFrame` / `VideoFrameRef` functions to
  allow usage in more generic contexts.

## [0.15.2] - 2020-01-30
### Fixed
- Fix another race condition in the `gst::Bus` `Stream` that could cause it to
  not wake up although a message is available.

## [0.15.1] - 2020-01-23
### Added
- Use static inner lifetime for `VideoCodecState<Readable>` so that it can be
  stored safely on the heap.
- Getters/setters for `BinFlags` on `gst::Bin`.
- `gst::Caps::builder_full()` for building caps with multiple structures
  conveniently.
- `gst::Element::call_async_future()` for asynchronously spawning a closure
  and returning a `Future` for awaiting its return value.

### Fixed
- Various clippy warnings.
- Getters/setters for `PadFlags` on `gst::Pad` now provide the correct
  behaviour.
- Take mutex before popping messages in the `gst::Bus` `Stream` to close a
  small race condition that could cause it to not be woken up.
- `gst::ChildProxy` implementers do not have to provide `child_added()` and
  `child_removed()` functions anymore but these are optional now.
- Manually implement `Debug` impls for various generic types where to `Debug`
  impl should not depend on their type parameters also implementing `Debug`.

## [0.15.0] - 2019-12-18
### Added
- `StructureRef::get_optional()` for returning `None` if the field does not
  exist instead of `Err`
- Bindings for `gstreamer-rtp` library, mostly `RTPBuffer`
- Support for writing `Preset`, `TagSetter`, `Clock`, `SystemClock` subclasses
- Bindings for `Typefind::get_length()`
- Bindings for `BaseSrcImpl::get_times()`
- Bindings (incl. subclassing) for `AudioSink` and `AudioSrc`
- Missing `Send`/`Sync` impl for various types

### Fixed
- Cleanup of cargo features/dependencies to improve build times
- Serde serialization with optional values.
  Attention: This changes the format of the serialization!
- `VideoEncoder`/`VideoDecoder` `proxy_getcaps()` can't return `None`
- Use non-panicking UTF8 conversion in log handler. We don't want to panic
  just because some C code printed a non-UTF8 string
- Re-rexport all traits from the crate level and also ensure that all traits
  are actually included in the preludes
- Actually export `is_video_overlay_prepare_window_handle_message()` function
- Use `FnMut` for the `appsink` callbacks instead of `Fn`
- `Promise` change function returns the actual reply to the promise now
  instead of just passing the promise itself
- Memory leak in `Iterator::filter()`
- `BinImpl::add()` takes ownership of floating references
- `DeviceImpl::create_element()` preserves floating flag
- `BinImpl::remove()` takes a strong reference of the element now as the last
  reference might be owned by the bin and otherwise we would potentially have
  a use-after-free afterwards
- `BaseParseFrame` and `VideoCodecFrame` take a `&mut self` now for various
  functions that actually change the frame

### Changed
- Minimum supported Rust version is 1.39
- Allow passing `None` to `VideoEncoder::finish_frame()`
- Various `to_string()` methods were moved into the `Display` trait impl and
  for some types `to_str()` was added to return a `&'static str`
- .gir files were updated to 1.16.2 release
- `Sample` constructor uses the builder pattern now
- `VideoMeta::add_full()` is simplified and requires parameters
- `BasetransformImpl::set_caps()` returns a `Result` instead of `bool`
- SDP data type getters for strings return an `Option` now as these can be
  `None` in practice although not allowed by the SDP spec
- Various functions returning `Option`s were changed to return `Results` if
  `None` actually signalled an error instead of just a missing value

### Removed
- "subclassing" and "futures" cargo features. These are enabled by default now

## [0.14.5] - 2019-09-17
### Added
- Support subclassing of `gst::Device`, `gst::DeviceProvider`,
  `gst_audio::AudioDecoder` and `::AudioEncoder`
- Support for `Element::set_clock` and `::provide_clock` virtual methods
- `ElementClass::add_metadata` was added
- `gst_video::VideoDecoder` and `::VideoEncoder` got support for `get_caps`,
  `negotiate`, `src/sink_query/event` and the `drain` virtual methods
- `Element::num_pads`, `::num_src_pads` and `::num_sink_pads` functions
- `gst_video::VideoDecoder` and `::VideoEncoder` got `get_allocator` bindings
- `gst::Iterator` implements `IntoIterator` now for providing
  `std::iter::Iterator<Item=<Result<T, IteratorError>>` adapter
- Error macros for audio/video decoder subclasses to handle decoding errors
  more gracefully and only actually error out after many consecutive errors

### Fixed
- Macros now also work in Rust 2018 edition without `#[macro_use]` but
  explicit imports
- The log handler unit test runs reliable in parallel with other tests
- Manually implement `Debug` for `gst::Iterator` to allow it for any `T`
  instead of `T: Debug`
- `Device::create_element` has correct reference count handling now
- Return `NotNegotiated` in the video codec base classes if setting the output
  state fails instead of `Error`

## [0.14.4] - 2019-08-14
### Added
- Bindings for adding/removing custom log functions
- Bindings for `calculate_linear_regression()`
- Constants for base class custom flow returns

### Fixed
- Ownership of pad in `Element::release_pad()` virtual method implementations

## [0.14.3] - 2019-07-16
### Added
- `Buffer::unset_flags()` for unsetting specific buffer flags
- `VideoBufferFlags` flags type and `VideoBufferExt::set_video_flags()`,
  `unset_video_flags()` and `get_video_flags()` for working with video buffer
  flags from safe code.

### Fixed
- Setting buffer flags does not override arbitrary other flags anymore but
  only sets the flags in question. This is necessary to not override extension
  buffer flags like `gst_video::VideoBufferFlags`.

## [0.14.2] - 2019-07-15
### Added
- Support for `ReferenceTimestampMeta`

## [0.14.1] - 2019-07-06
### Added
- Various new WebRTC enum types from 1.14.1/1.16.0

### Fixed
- Correctly generate interlaced `VideoInfo` by using
  `gst_video_info_set_interlaced_format()` instead of the generic function.
- serde serialization unit tests for `gst::format` succeed again now.

### Changed
- `Debug` impls for `VideoFormatInfo` and `AudioFormatInfo` now print all the
  details of the format instead of only the name, and the `Debug` impls for
  `VideoInfo` and `AudioInfo` also print the format now.

## [0.14.0] - 2019-06-24
### Added
- Bindings for `GLSyncMeta`.
- Bindings for setting/getting `TagScope` on a `TagList`
- Bindings for `GLDisplayWayland` and `GLDisplayX11` in addition to the
  already existing `GLDisplayEGL`
- Bindings for `Bus::pop_filtered()` and related functions
- Bindings for getting/setting `Object`, `Element`, `Bin`, `Pipeline` and
  `Plugin` flags
- Bindings for `VideoCaptionMeta`
- `Debug` impl of `Buffer` now also shows the metas of the buffers
- Expose flow return in `PadProbeInfo` for overriding the return value
- Bindings for `VideoDecoder` and `VideoEncoder`, including subclassing
  support
- Bindings for `Memory`, `Allocator` and `VideoBufferPool`
- Bindings for `VideoFormatInfo::pack` and `::unpack` for format conversion
- Bindings for `BaseParse`, including subclassing support
- Various new arithmetic operation impls for fractions, formatted values and
  `ClockTime`
- Bindings for `VideoInfo::align()`

### Changed
- The `SDPMessage` and `SDPMedia` bindings were completely rewritten as they
  were broken before and caused crashes in various usages. As part of this
  there's also some more convenience API available on these types, like
  iterators for example, and API to modify the `SDPMedia` contained in a
  `SDPMessage`.
- Update to GStreamer 1.16.
- Regenerate with latest gir.
- Run all autogenerated code through rustfmt after generation too.
- Updated to latest versions of GLib/GIO/etc crates.
- Updated to futures 0.3 / `std::future`
- `ProxyPad` default functions moved to an extension trait instead of plain
  functions on `ProxyPad`, making them more in sync with the default `Pad`
  functions
- GStreamer plugins are now exporting the new 1.14+ plugin symbols if they
  were configured for GStreamer 1.14+
- Arithmetic operations on formatted values and `ClockTime` do overflow checks
  now and replace the result with the `NONE` value on overflow
- `TryFrom`/`TryInto` traits are used in various places now instead of the
  previous ad-hoc implementations of them.
- Registering element/typefind/device monitor factories requires passing a
  value of `gst::Rank` now instead of an arbitrary `u32`

### Fixed
- Use correct type for destroying pad task closure data. This was previously
  using the wrong type, causing crashes at runtime.
- `DeviceAdded`/`DeviceRemoved` message getters are transfer full so we don't
  need to take an additional reference that would be leaked.
- `AppSink` callbacks are correctly marked as `Send` instead of `Send+Sync`,
  allowing a wider range of closures to be used for them.
- Handle `PadProbeReturn::Handled` return values from pad probes more
  correctly.
- `ToOwned::to_owned()` on miniobjects has to create copies instead of
  only increasing the reference count. Otherwise it was possible to create
  multiple mutable and immutable references to the same object at the same
  time.
- Various functions take references to owned miniobjects instead of borrowed
  references as it was otherwise possible to create multiple mutable or
  immutable references to the same object at the same time.
- `URIHandler::set_uri` does not accept `None` anymore as this is not allowed
  by the C function.
- Comparisons and addition of `TypeFindProbability` and `Rank` work correctly now
- Various `Display` implementations were fixed to not cause a stack overflow
  due to infinite recursion anymore
- Various `::to_string()` functions don't take ownership of C strings anymore
  that they do not own, which caused double frees before

### Removed
- MIKEY related bindings from the SDP library. The bindings were broken and
  until someone needs them these are not available anymore.

## [0.13.0] - 2019-02-22
### Added
- Subclassing infrastructure was moved directly into the bindings,
  making the `gst-plugin` crate deprecated. This involves many API
  changes but generally cleans up code and makes it more flexible.
  Take a look at the `gst-plugins-rs` crate for various examples.
- Bindings for GStreamer GL library
- Bindings for `CapsFeatures` and `Meta`
- Bindings for `ParentBufferMeta, `VideoMeta` and `VideoOverlayCompositionMeta`
- Bindings for `VideoOverlayComposition` and `VideoOverlayRectangle`
- Bindings for `VideoTimeCode`
- Bindings for `NetAddressMeta`
- Bindings for registering custom tags
- `UniqueFlowCombiner` and `UniqueAdapter` wrappers that make use of
  the Rust compile-time mutability checks and expose more API in a safe
  way, and as a side-effect implement `Sync` and `Send` now
- `Bus::add_watch_local()` and `gst_video::convert_frame_async_local()` that
  allows to use a closure that does not implement `Send` but can only be
  called from the thread owning the main context.
- More complete bindings for `Allocation` `Query`
- `pbutils` functions for codec descriptions
- `TagList::iter()` for iterating over all tags while getting a single
   value per tag. The old `::iter_tag_list()` function was renamed to
   `::iter_generic()` and still provides access to each value for a tag
- `Bus::iter()` and `Bus::iter_timed()` iterators around the
  corresponding `::pop*()` functions
- Getters for `VideoColorimetry` to access its fields
- `Debug` impls for various missing types.
- serde serialization of `Value` can also handle `Buffer` now
- Extensive comments to all examples with explanations
- Transmuxing example showing how to use `typefind`, `multiqueue` and
  dynamic pads
- basic-tutorial-12 was ported and added

### Changed
- Rust 1.31 is the minimum supported Rust version now
- Update to latest gir code generator and glib bindings
- Functions returning e.g. `gst::FlowReturn` or other "combined" enums
  were changed to return split enums like `Result<gst::FlowSuccess,
  gst::FlowError>` to allow usage of the standard Rust error handling.
- Various functions and callbacks returning `bool` or `Option<_>` were
  changed to return a `Result<_, glib::BoolError>` or
  `Result<_, gst::LoggableError>` or `Result<_, gst::ErrorMessage>` for
  better integration with Rust's error handling infrastructure.
- Some infallible functions returning `bool` were changed to return `()`.
- `MiniObject` subclasses are now newtype wrappers around the
   underlying `GstRc<FooRef>` wrapper. This does not change the
   API in any breaking way for the current usages, but allows
   `MiniObject`s to also be implemented in other crates and
   makes sure `rustdoc` places the documentation in the right places.
- `BinExt` extension trait was renamed to `GstBinExt` to prevent
  conflicts with `gtk::Bin` if both are imported
- `Buffer::from_slice()` can't possible return `None`

### Fixed
- `gst::tag::Album` is the album tag now instead of artist sortname
- Return `0` for the channel mask corresponding to negative
  `AudioChannelPosition`s.
- `PartialOrd` and related traits are implemented via pointer equality on
  `ClockId` instead of using the compare function. Two clock ids with the same
  timestamp are not necessarily the same.
- Various functions that are actually fallible are now returning an
  `Option<_>`.
- Various `clippy` warnings

## [0.12.2] - 2018-11-26
### Fixed
- PTP clock constructor actually creates a PTP instead of NTP clock

### Added
- Bindings for GStreamer Editing Services
- Bindings for GStreamer Check testing library
- Bindings for the encoding profile API (encodebin)
- VideoFrame, VideoInfo, AudioInfo, StructureRef implements Send and Sync now
- VideoFrame has a function to get the raw FFI pointer
- From impls from the Error/Success enums to the combined enums like
  FlowReturn
- Bin-to-dot file functions were added to the Bin trait
- gst_base::Adapter implements SendUnique now

### Changed
- All references were updated from GitHub to freedesktop.org GitLab
- Fix various links in the README.md
- Link to the correct location for the documentation
- Remove GitLab badge as that only works with gitlab.com currently

## [0.12.1] - 2018-09-21
### Added
- More complete bindings for the gst_video::VideoOverlay interface, especially
  gst_video::is_video_overlay_prepare_window_handle_message()

## [0.12.0] - 2018-09-08
### Added
- Bindings for the GStreamer SDP and WebRTC libraries
- Generic API for working with tags that is based on string tag names and
  glib::Value for the tag values
- Bindings for Aggregator and AggregatorPad
- Bindings for BaseTransform/BaseSrc::get_buffer_pool()
- Optional serde implementations for the basic GStreamer data flow and metadata types

### Changed
- Use ptr::NonNull in various places
- Updated to muldiv 0.2, num-rational 0.2
- Bus::create_watch() can't return None
- Remove CallbackGuard as unwinding across FFI boundaries is not undefined
  behaviour anymore but will directly cause a panic
- Changed from the futures to the futures-preview crate as an optional
  dependency
- Various Caps operations take a &CapsRef instead of &Caps
- "deep-notify" signal takes the whole ParamSpec as parameter instead of only
  the signal name
- Some structs were changed from empty struct to empty enums
- Pad probe code does not take an additional reference to the data anymore,
  potentially passing writable events/buffers into the probe
- ValueExt::compare() is implemented around std::cmp::Ordering now instead of
  a custom enum that was basically the same

### Fixed
- Pad::add_probe() can return None if an IDLE probe was already called and
  removed in the meantime
- Various compiler and clippy warnings

### Removed
- std::Iterator impl for gst::Iterator. It was awkward to use because the
  gst::Iterator could fail at each iteration

## [0.11.6] - 2018-08-27
### Fixed
- Build with NLL/two-phase borrows
- Explicitly define [bin] section for discoverer example to fix a cargo
  warning

### Added
- Add unsafe gst::deinit() function
- Ord/PartialOrd impls on gst::Seqnum
- Getter for current pad mode
- gst::Pad::sticky_events_foreach() for iterating over all sticky events
  in a thread-safe way

## [0.11.5] - 2018-07-24
### Fixed
- `gst::Bus`'s sync handler must unref every message if
  `gst::BusSyncReply::Drop` is returned, otherwise they are all leaked

## [0.11.4] - 2018-07-19
### Fixed
- `gst::Caps::subtract()` does not leak its arguments anymore
- `gst::Caps::get_structure()` gracefully returns `None` if the index
  is out of bounds instead of a `g_return_val_if_fail()`
- `gst::Structure::new()` has to give away ownership of the info structure
  but didn't. For 0.11 we internally copy, in 0.12 it will take the info
  structure by value
- Typefind tests don't fail anymore if the system has typefind factories
  without caps

### Added
- An additional assertion that ensures that miniobjects are actually
  writable before creating a mutable reference

## [0.11.3] - 2018-06-08
### Added
- `gst::Bus::remove_watch()` is now available to remove a bus watch again
- `fmt::Debug` impls for `AudioInfo` and `VideoInfo` were added
- `fmt::Debug` impls for mini objects also print the pointer value now to make
  it easier to track them in debug logs
- `PlayerVisualization` has accessors for the name and description fields now,
  without which there is no sensible way to use them or to set a player
  visualization

## [0.11.2] - 2018-05-09
### Fixed
- Work-around various floating reference handling changes between 1.12 and
  1.14 to be able to run with both versions without memory leaks or other
  reference count problems.
  This affects NetTimeProvider, BufferPool, DeviceMonitor, Stream,
  StreamCollection, and Player, NetClientClock, NetClock, PtpClock which were
  already previously fixed.

### Changed
- Change the appsrc need-data and all appsink callbacks to not require the
  Sync bound anymore and change from Fn to FnMut. They can only be called from
  a single thread at a time. This change is only done for the corresponding
  callbacks, not the signals.

## [0.11.1] - 2018-04-07
### Fixed
- Fix Structure::to_string() to not run into an infinite recursion but call
  the method on the contained StructureRef instead of on itself

## [0.11.0] - 2018-03-20
### Changed
- Updated everything to GStreamer 1.14.0
- Event, Message and Query types were refactored to improve usability.
  Especially newly constructed queries allow to directly use the type-specific
  functions to be used without first creating a view
- VideoFrameRef::copy_to_ref() and ::copy_plane_to_ref() are gone now and the
  original functions work with refs instead of full frames
- PadProbeId and NotifyIds are not Copy/Clone anymore and are taken by value
- GstPlayer has GstObject as parent class now

### Added
- GstPbutils, GstSdp, GstRtsp and GstRtspServer bindings
- GstPromise, GstAudioStreamAlign and various other 1.14 API
- GstVideoFilter and GstBufferPool bindings
- Element::call_async()
- Debug impl For Toc and TocEntry
- Various new examples (RTP FEC, RTSP server, tag usage, ...)

### Fixed
- Memory leak in gst_video::convert_sample_async()

## [0.10.2] - 2018-02-18
### Fixed
- Fix building of messages with custom fields for types that don't have a
  GstStructure

### Added
- VideoFrameRef::copy_to_ref() and ::copy_plane_to_ref(), which work with
  VideoFrameRefs instead of full VideoFrames
- Getters for the BaseSrc/Sink/Transform configured segment
- Document the gstreamer-player-1.0 dependency in the README.md

## [0.10.1] - 2018-01-03
### Fixed
- Don't require &mut self for TagSetterExtManual::add()

### Added
- A TagSetter example application
- Bindings for gst_video::convert_sample() and ::convert_sample_async()
- Bindings for gst_video::VideoRectangle
- Debug impl for Sample and ::with_buffer_list() constructor
- A borrowing version of VideoFrame: VideoFrameRef
- Bindings for GstVideoFilter

### Changed
- Deprecated Sample::get_info() in favour of ::get_structure()
- Player has gst::Object as another parent class now

## [0.10.0] - 2017-12-22
### Fixed
- Various clippy warnings
- Memory leak of the tag list in Toc::merge_tags()
- Property getters use Values of the correct type
- Event::get_structure(), Message::get_structure() and
  Query::get_structure() can return None for the structure
- Various other nullability fixes all over the API, changing functions to
  accept Option<> or returning Option<>, or only plain types
- Functions taking paths/filenames now actually take Paths instead of &strs
- Element::remove_pad() is not giving away a new reference to the pad
  anymore, which caused a memory leak of all pads ever removed
- Precision handling in ClockTime's Display impl
- Video/AudioInfo are only Send, not Sync

### Added
- Various enums now also derive useful traits like Copy, Clone and Hash in
  addition to PartialEq, Eq and Debug
- TagList::merge() and insert() for combining tag lists
- EventType gained many useful functions to work with event types and
  a PartialOrd impl to check expected event order of event types where it matters
- MessageRef/EventRef/QueryRef implement ToOwned
- Bindings for Registry and PluginFeature
- Event::set_running_time_offset() for adjusting the offset while events
  pass through the pipeline
- Event/Message GroupIds and Seqnums now have a newtype wrapper around u32
  instead of the plain value, making usage of them slightly more typesafe.
  Also add an "invalid" value for both, as exists in latest GStreamer now.
- FormattedValue, GenericFormattedValue and related types were
  implemented now, which allows more convenient and type-safe usage of
  formatted values (time, bytes, etc)
- Bindings for force-keyunit and still-frame events were added
- MappedBuffer/BufferMap now implement various other useful traits, including
  AsRef<[u8]>, AsMut, Deref, DerefMut, Debug, PartialEq and Eq
- Add VideoMultiviewFramePacking enum, and use it in Player
- Bindings for the GStreamer Net library, including PTP/NTP/network client
  clocks and the GStreamer NetClock provider for network synchronization of
  pipelines
- IteratorError implements std::error:Error
- Plugin::add_dependency() and ::add_dependency_simple() was added
- Rank and TypeFindProbability implement PartialOrd/Ord now
- Bindings for TypeFind, TypeFindFactory and the typefind helpers
- StreamCollection::iter() for iterating over all contained streams
- ErrorMessage type that can be used e.g. in a Result for passing an error
  message from somewhere to upper layers to then be posted on an element the
  same way gst_element_error!() would've done

### Changed
- Sample::new(), TagList::add(), Structure::set() and similar
  functions take the values (ToSendValue impls) by reference instead of value.
  They were not consumed by the function before.
- The Debug impls of various types, including Event/Buffer/Message/Query/Structure
  were improved to print all the fields, similar to what GST_PTR_FORMAT would
  do in C
- Switched to lazy_static 1.0
- Gap event and Duration tag are using ClockTimes now, as well as various
  Player signals
- Segment is now based on a generic type FormattedSegment that can
  take any format (time, bytes, etc) or a GenericFormattedValue for more
  type-safety and convenience. Also functions for "casting" between a generic
  segment and a segment with a specific format exist on this now
- AppSrc and AppSink now have a builder for the callbacks, making it
  unnecessary to always provide all callbacks even if only one is actually
  needed
- Various functions that returned bool for errors, are now returning a Result
- Player configuration is now a custom type with more convenient API
- Player VideoInfo uses a Fraction instead of (u32,u32) for the framerate and
  pixel-aspect-ratio
- VideoFrame API has more consistent API between writable and read-only
  variants
- Buffer::copy_into() was added, and ::copy_region() now takes a
  BufferCopyFlags parameter instead of always using the default flags
- ChildProxy::set_child_property() takes a &ToValue now to follow the API of
  Object::set_property() and improve usability
- Proxy/GhostPad default pad functions use the correct specific pad type now
  instead of a generic Pad
- Bus::add_signal_watch_full() takes a Priority for the priority instead of u32
- Clock::(un)adjust_with_calibration() takes no clock parameter anymore

### Removed
- FormatValue was removed in favour of GenericFormattedValue and the
  connected traits and specific format impls

## [0.9.1] - 2017-11-26
### Fixed
- Export `FlowError`/`FlowSuccess`, `ClockError`/`ClockSuccess`,
  `PadLinkError`/`PadLinkSuccess` too

## [0.9.0] - 2017-11-26
### Added
- Bindings for (outputting to) the GStreamer logging system
- Bindings for the GStreamer base library
- Bindings for all the `Pad` functions to override pad behaviour, and pad task
  functions
- Bindings for `StaticCaps` and `StaticPadTemplate`
- Bindings for `deep-notify` signal on `Object`
- Support for directly creating `Error`/`Warning`/`Info` `Messages` and posting them
  from an element with context information (file, line, module, etc.) similar
  to the C `GST_ELEMENT_ERROR` macro
- Support for setting custom fields in `Messages`/`Events` during construction
- Support for creating Buffers out of anything that is `AsRef<[u8]>` or
  `AsMut<[u8]>`
- Support for using the `Read` trait on `Adapter`
- Functions for getting all sink/src/all pads of an `Element`, and all children
  of a `Bin`
- Builder for `Caps` and `Structures` in addition to the existing functions
- `AppSrc`/`AppSink` implement `BaseSrc`/`BaseSink` and `URIHandler`
- Rust ports of the basic tutorials 1 to 8 from
  https://gstreamer.freedesktop.org/documentation/tutorials/
- "Getting started" and "Installation" sections to the README.md
- "dox" feature for generating documentation for all available configurations

### Fixed
- `StackTraceFlags` are only available since 1.12
- Worked around macOS requiring a `NSRunLoop` running on the main thread in all
  examples and tutorials, to be able to show a window or anything else

### Changed
- `ClockTime` is now a wrapper around `Option<u64>` to handle the
  `CLOCK_TIME_NONE` case better. This wrapper implements all the arithmetic
  and other traits as needed and ensures that no accidental calculations with
  `CLOCK_TIME_NONE` can happen
- "Values with format", like in `Duration`/`Position`/`Convert` queries or
  `Seek` events now return a `FormatValue` type. This contains the actual
  `Format` together with the value and does any required conversions. This
  also makes it harder to accidentally mix e.g. values in bytes and time
- `PadProbeId` does not implement `Clone`/`Copy` anymore
- Property notify watches return a custom type instead of ulong
- `Error`/`Warning`/`Info` `Messages` can only be created with specific kinds of
  `glib::Error` now. Using arbitrary ones does not work
- `Iterator` bindings were completely rewritten and provide the item type as a
  generic type parameter now, greatly simplifying its usage
- All `glib::Values` are now `glib::SendValue` instead, e.g. in `Caps` and
  `Structures`, as their content must be possible to send to different threads
  safely
- `Message::get_src()` can return `None`
- Allow `None` as `Caps` in `AppSrc`/`AppSink`
- Allow everything implementing `Into<Option<&str>>` to be used as a pad name
- Moved `copy()` from `GstRc` directly to `MiniObject`
- Success/Error enums (like `FlowReturn`, `PadLinkReturn`, `StateChangeReturn`) now
  implement an `into_result()` function that splits them into a `Result` with
  the good and bad cases. Also mark them as `#[must_use]` to make it harder to
  accidentally ignore errors.
- Error enums implement the `Error` trait
- Many examples use the `failure` crate for error handling now, cleaning up the
  error handling code quite a bit
- Lots of other code cleanup, compiler/clippy warning cleanup, etc.

## [0.8.2] - 2017-11-11
### Fixed
- Implement StaticType of BufferRef instead of Buffer. Buffer aka
  GstRc<BufferRef> already implements StaticType if BufferRef does, and
  without this it was not possible to use Buffers in GValues.
- Free memory of the appsink/appsrc callbacks with the correct type. It was
  crashing because of using the wrong type before.
- Fix documentation URLs in Cargo.toml.

### Added
- Installation instructions and links to documentation for getting started to
  README.md.

## [0.8.1] - 2017-09-15
### Added
- Implement Send+Sync for Query, Message and Event, and their corresponding
  Ref types.

### Fixed
- Constructor for gst_player::Player now works properly with GStreamer 1.12
  when passing a video renderer or signal dispatcher. There was a reference
  counting bug.
- Instead of returning &'static references from functions, return references
  with a generic, unbound lifetime instead.
  See https://github.com/rust-lang/rust/pull/42417#issue-233404573
- Various "unused external crate" warnings and clippy warnings everywhere.

### Changed
- Remove Cargo.lock from GIT, it's not very useful for library crates.
- Run everything through latest rustfmt-nightly.
- Use while-let (instead of loop and if-let) and CLOCK_TIME_NONE (instead of
  u64::MAX) in the examples.

## 0.8.0 - 2017-08-31

- Initial release of the autogenerated GStreamer bindings. Older versions
  (< 0.8.0) of the bindings can be found [here](https://github.com/arturoc/gstreamer1.0-rs).
  The API of the two is incompatible.

[Unreleased]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.5...HEAD
[0.23.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.4...0.23.5
[0.23.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.3...0.23.4
[0.23.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.2...0.23.3
[0.23.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.1...0.23.2
[0.23.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.23.0...0.23.1
[0.23.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.6...0.23.0
[0.22.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.5...0.22.6
[0.22.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.4...0.22.5
[0.22.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.3...0.22.4
[0.22.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.2...0.22.3
[0.22.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.1...0.22.2
[0.22.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.22.0...0.22.1
[0.22.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.21.3...0.22.0
[0.21.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.21.2...0.21.3
[0.21.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.21.1...0.21.2
[0.21.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.21.0...0.21.1
[0.21.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.7...0.21.0
[0.20.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.6...0.20.7
[0.20.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.5...0.20.6
[0.20.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.4...0.20.5
[0.20.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.3...0.20.4
[0.20.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.2...0.20.3
[0.20.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.1...0.20.2
[0.20.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.20.0...0.20.1
[0.20.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.8...0.20.0
[0.19.8]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.7...0.19.8
[0.19.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.6...0.19.7
[0.19.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.5...0.19.6
[0.19.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.4...0.19.5
[0.19.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.3...0.19.4
[0.19.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.2...0.19.3
[0.19.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.1...0.19.2
[0.19.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.19.0...0.19.1
[0.19.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.8...0.19.0
[0.18.8]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.7...0.18.8
[0.18.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.6...0.18.7
[0.18.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.5...0.18.6
[0.18.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.4...0.18.5
[0.18.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.3...0.18.4
[0.18.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.2...0.18.3
[0.18.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.1...0.18.2
[0.18.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.18.0...0.18.1
[0.18.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.17.4...0.18.0
[0.17.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.17.3...0.17.4
[0.17.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.17.2...0.17.3
[0.17.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.17.1...0.17.2
[0.17.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.17.0...0.17.1
[0.17.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.7...0.17.0
[0.16.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.6...0.16.7
[0.16.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.5...0.16.6
[0.16.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.4...0.16.5
[0.16.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.3...0.16.4
[0.16.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.2...0.16.3
[0.16.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.1...0.16.2
[0.16.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.16.0...0.16.1
[0.16.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.7...0.16.0
[0.15.7]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.6...0.15.7
[0.15.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.5...0.15.6
[0.15.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.4...0.15.5
[0.15.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.3...0.15.4
[0.15.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.2...0.15.3
[0.15.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.1...0.15.2
[0.15.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.15.0...0.15.1
[0.15.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.2...0.15.0
[0.14.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.1...0.14.2
[0.14.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.14.0...0.14.1
[0.14.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.13.0...0.14.0
[0.13.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.2...0.13.0
[0.12.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.1...0.12.2
[0.12.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.12.0...0.12.1
[0.12.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.6...0.12.0
[0.11.6]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.5...0.11.6
[0.11.5]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.4...0.11.5
[0.11.4]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.3...0.11.4
[0.11.3]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.2...0.11.3
[0.11.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.1...0.11.2
[0.11.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.11.0...0.11.1
[0.11.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.2...0.11.0
[0.10.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.1...0.10.2
[0.10.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.10.0...0.10.1
[0.10.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.9.1...0.10.0
[0.9.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.9.0...0.9.1
[0.9.0]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.1...0.9.0
[0.8.2]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.1...0.8.2
[0.8.1]: https://gitlab.freedesktop.org/gstreamer/gstreamer-rs/compare/0.8.0...0.8.1
