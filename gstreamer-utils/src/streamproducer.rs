use std::{
    collections::HashMap,
    mem,
    sync::{atomic, Arc, Mutex, MutexGuard},
};

use gst::{glib, prelude::*};
use once_cell::sync::Lazy;
use thiserror::Error;

// Small wrapper around AtomicU64 and a Mutex, to allow it to run regular AtomicU64
// operations where supported, and fallback to a mutex where it is not. The wrapper methods
// are the ones that are needed, and not all are exposed.
#[derive(Debug)]
struct WrappedAtomicU64 {
    #[cfg(not(target_has_atomic = "64"))]
    atomic: Mutex<u64>,
    #[cfg(target_has_atomic = "64")]
    atomic: atomic::AtomicU64,
}

#[cfg(target_has_atomic = "64")]
impl WrappedAtomicU64 {
    fn new(value: u64) -> WrappedAtomicU64 {
        WrappedAtomicU64 {
            atomic: atomic::AtomicU64::new(value),
        }
    }
    fn fetch_add(&self, value: u64, order: atomic::Ordering) -> u64 {
        self.atomic.fetch_add(value, order)
    }
    fn store(&self, value: u64, order: atomic::Ordering) {
        self.atomic.store(value, order);
    }

    fn load(&self, order: atomic::Ordering) -> u64 {
        self.atomic.load(order)
    }
}

#[cfg(not(target_has_atomic = "64"))]
impl WrappedAtomicU64 {
    fn new(value: u64) -> WrappedAtomicU64 {
        WrappedAtomicU64 {
            atomic: Mutex::new(value),
        }
    }
    fn fetch_add(&self, value: u64, _order: atomic::Ordering) -> u64 {
        let mut guard = self.atomic.lock().unwrap();
        let old = *guard;
        *guard += value;
        old
    }
    fn store(&self, value: u64, _order: atomic::Ordering) {
        *self.atomic.lock().unwrap() = value;
    }
    fn load(&self, _order: atomic::Ordering) -> u64 {
        *self.atomic.lock().unwrap()
    }
}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "utilsrs-stream-producer",
        gst::DebugColorFlags::empty(),
        Some("gst_app Stream Producer interface"),
    )
});

/// The interface for transporting media data from one node
/// to another.
///
/// A producer is essentially a GStreamer `appsink` whose output
/// is sent to a set of consumers, who are essentially `appsrc` wrappers
#[derive(Debug, Clone)]
pub struct StreamProducer(Arc<StreamProducerInner>);

impl PartialEq for StreamProducer {
    fn eq(&self, other: &Self) -> bool {
        self.0.appsink.eq(&other.0.appsink)
    }
}

impl Eq for StreamProducer {}

#[derive(Debug)]
struct StreamProducerInner {
    /// The appsink to dispatch data for
    appsink: gst_app::AppSink,
    /// The pad probe on the appsink=
    appsink_probe_id: Option<gst::PadProbeId>,
    /// The consumers to dispatch data to
    consumers: Arc<Mutex<StreamConsumers>>,
}

impl Drop for StreamProducerInner {
    fn drop(&mut self) {
        if let Some(probe_id) = self.appsink_probe_id.take() {
            let pad = self.appsink.static_pad("sink").unwrap();
            pad.remove_probe(probe_id);
        }

        self.appsink
            .set_callbacks(gst_app::AppSinkCallbacks::builder().build());
    }
}

/// Link between a `StreamProducer` and a consumer, disconnecting the link on `Drop`.
/// The producer and consumer will stay alive while the link is.
#[derive(Debug)]
#[must_use]
pub struct ConsumptionLink {
    consumer: gst_app::AppSrc,
    producer: Option<StreamProducer>,
    /// number of buffers dropped because `consumer` internal queue was full
    dropped: Arc<WrappedAtomicU64>,
    /// number of buffers pushed through `consumer`
    pushed: Arc<WrappedAtomicU64>,
    /// if buffers should not be pushed to the `consumer` right now
    discard: Arc<atomic::AtomicBool>,
    /// whether the link will drop delta frames until next keyframe on discont
    wait_for_keyframe: Arc<atomic::AtomicBool>,
}

impl ConsumptionLink {
    /// Create a new disconnected `ConsumptionLink`.
    pub fn disconnected(consumer: gst_app::AppSrc) -> ConsumptionLink {
        ConsumptionLink {
            consumer,
            producer: None,
            dropped: Arc::new(WrappedAtomicU64::new(0)),
            pushed: Arc::new(WrappedAtomicU64::new(0)),
            discard: Arc::new(atomic::AtomicBool::new(false)),
            wait_for_keyframe: Arc::new(atomic::AtomicBool::new(true)),
        }
    }

    /// Replace the producer by a new one, keeping the existing consumer.
    pub fn change_producer(
        &mut self,
        new_producer: &StreamProducer,
        reset_stats: bool,
    ) -> Result<(), AddConsumerError> {
        self.disconnect();
        if reset_stats {
            self.dropped.store(0, atomic::Ordering::SeqCst);
            self.pushed.store(0, atomic::Ordering::SeqCst);
        }
        new_producer.add_consumer_internal(
            &self.consumer,
            self.dropped.clone(),
            self.pushed.clone(),
            self.discard.clone(),
            self.wait_for_keyframe.clone(),
        )?;
        self.producer = Some(new_producer.clone());
        Ok(())
    }

    /// Disconnect the consumer from the producer
    pub fn disconnect(&mut self) {
        if let Some(producer) = self.producer.take() {
            producer.remove_consumer(&self.consumer);
        }
    }

    /// number of dropped buffers because the consumer internal queue was full
    pub fn dropped(&self) -> u64 {
        self.dropped.load(atomic::Ordering::SeqCst)
    }

    /// number of buffers pushed through this link
    pub fn pushed(&self) -> u64 {
        self.pushed.load(atomic::Ordering::SeqCst)
    }

    /// if buffers are currently pushed through this link
    pub fn discard(&self) -> bool {
        self.discard.load(atomic::Ordering::SeqCst)
    }

    /// If set to `true` then no buffers will be pushed through this link
    pub fn set_discard(&self, discard: bool) {
        self.discard.store(discard, atomic::Ordering::SeqCst)
    }

    /// if the link will drop frames until the next keyframe on discont
    pub fn wait_for_keyframe(&self) -> bool {
        self.wait_for_keyframe.load(atomic::Ordering::SeqCst)
    }

    /// If set to `true` then the link will drop delta-frames until the next
    /// keyframe on discont (default behavior).
    pub fn set_wait_for_keyframe(&self, wait: bool) {
        self.wait_for_keyframe.store(wait, atomic::Ordering::SeqCst)
    }

    /// Get the GStreamer `appsrc` wrapped by this link
    pub fn appsrc(&self) -> &gst_app::AppSrc {
        &self.consumer
    }
}

impl Drop for ConsumptionLink {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[derive(Debug, Error)]
/// Error type returned when adding consumers to producers.
pub enum AddConsumerError {
    #[error("Consumer already added")]
    /// Consumer has already been added to this producer.
    AlreadyAdded,
}

impl StreamProducer {
    /// Configure a consumer `appsrc` for later use in a `StreamProducer`
    ///
    /// This is automatically called when calling `add_consumer()`.
    pub fn configure_consumer(consumer: &gst_app::AppSrc) {
        // Latency on the appsrc is set by the publisher before the first buffer
        // and whenever it changes
        consumer.set_latency(gst::ClockTime::ZERO, gst::ClockTime::NONE);
        consumer.set_format(gst::Format::Time);
        consumer.set_is_live(true);
        consumer.set_handle_segment_change(true);
        consumer.set_max_buffers(0);
        consumer.set_max_bytes(0);
        consumer.set_max_time(500 * gst::ClockTime::MSECOND);
        consumer.set_leaky_type(gst_app::AppLeakyType::Downstream);
        consumer.set_automatic_eos(false);
    }

    /// Add an appsrc to dispatch data to.
    ///
    /// Dropping the returned `ConsumptionLink` will automatically disconnect the consumer from the producer.
    pub fn add_consumer(
        &self,
        consumer: &gst_app::AppSrc,
    ) -> Result<ConsumptionLink, AddConsumerError> {
        let dropped = Arc::new(WrappedAtomicU64::new(0));
        let pushed = Arc::new(WrappedAtomicU64::new(0));
        let discard = Arc::new(atomic::AtomicBool::new(false));
        let wait_for_keyframe = Arc::new(atomic::AtomicBool::new(true));

        self.add_consumer_internal(
            consumer,
            dropped.clone(),
            pushed.clone(),
            discard.clone(),
            wait_for_keyframe.clone(),
        )?;

        Ok(ConsumptionLink {
            consumer: consumer.clone(),
            producer: Some(self.clone()),
            dropped,
            pushed,
            discard,
            wait_for_keyframe,
        })
    }

    fn add_consumer_internal(
        &self,
        consumer: &gst_app::AppSrc,
        dropped: Arc<WrappedAtomicU64>,
        pushed: Arc<WrappedAtomicU64>,
        discard: Arc<atomic::AtomicBool>,
        wait_for_keyframe: Arc<atomic::AtomicBool>,
    ) -> Result<(), AddConsumerError> {
        let mut consumers = self.0.consumers.lock().unwrap();
        if consumers.consumers.contains_key(consumer) {
            gst::error!(
                CAT,
                obj = &self.0.appsink,
                "Consumer {} ({:?}) already added",
                consumer.name(),
                consumer
            );
            return Err(AddConsumerError::AlreadyAdded);
        }

        gst::debug!(
            CAT,
            obj = &self.0.appsink,
            "Adding consumer {} ({:?})",
            consumer.name(),
            consumer
        );

        Self::configure_consumer(consumer);

        // Forward force-keyunit events upstream to the appsink
        let srcpad = consumer.static_pad("src").unwrap();
        let fku_probe_id = srcpad
            .add_probe(
                gst::PadProbeType::EVENT_UPSTREAM,
                glib::clone!(
                    #[weak(rename_to = appsink)]
                    self.0.appsink,
                    #[upgrade_or_panic]
                    move |_pad, info| {
                        let Some(event) = info.event() else {
                            return gst::PadProbeReturn::Ok;
                        };

                        if gst_video::UpstreamForceKeyUnitEvent::parse(event).is_ok() {
                            gst::debug!(CAT, obj = &appsink, "Requesting keyframe");
                            // Do not use `gst_element_send_event()` as it takes the state lock which may lead to dead locks.
                            let pad = appsink.static_pad("sink").unwrap();
                            let _ = pad.push_event(event.clone());
                        }

                        gst::PadProbeReturn::Ok
                    }
                ),
            )
            .unwrap();

        let stream_consumer = StreamConsumer::new(
            consumer,
            fku_probe_id,
            dropped,
            pushed,
            discard,
            wait_for_keyframe,
        );

        consumers
            .consumers
            .insert(consumer.clone(), stream_consumer);

        // forward selected sticky events. We can send those now as appsrc will delay the events
        // until stream-start, caps and segment are sent.
        let events_to_forward = consumers.events_to_forward.clone();
        // drop the lock before sending events
        drop(consumers);

        let appsink_pad = self.0.appsink.static_pad("sink").unwrap();
        appsink_pad.sticky_events_foreach(|event| {
            if events_to_forward.contains(&event.type_()) {
                gst::debug!(
                    CAT,
                    obj = &self.0.appsink,
                    "forward sticky event {:?}",
                    event
                );
                consumer.send_event(event.clone());
            }

            std::ops::ControlFlow::Continue(gst::EventForeachAction::Keep)
        });

        Ok(())
    }

    fn process_sample(
        sample: gst::Sample,
        appsink: &gst_app::AppSink,
        mut consumers: MutexGuard<StreamConsumers>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let (is_discont, is_keyframe) = if let Some(buf) = sample.buffer() {
            let flags = buf.flags();

            (
                flags.contains(gst::BufferFlags::DISCONT),
                !flags.contains(gst::BufferFlags::DELTA_UNIT),
            )
        } else {
            (false, true)
        };

        gst::trace!(
            CAT,
            obj = appsink,
            "processing sample {:?}",
            sample.buffer()
        );

        let latency = consumers.current_latency;
        let latency_updated = mem::replace(&mut consumers.latency_updated, false);

        let mut needs_keyframe_request = false;

        let current_consumers = consumers
            .consumers
            .values()
            .filter_map(|consumer| {
                if let Some(latency) = latency {
                    if consumer
                        .forwarded_latency
                        .compare_exchange(
                            false,
                            true,
                            atomic::Ordering::SeqCst,
                            atomic::Ordering::SeqCst,
                        )
                        .is_ok()
                        || latency_updated
                    {
                        gst::info!(CAT, obj = appsink, "setting new latency: {latency}");
                        consumer.appsrc.set_latency(latency, gst::ClockTime::NONE);
                    }
                }

                if consumer.discard.load(atomic::Ordering::SeqCst) {
                    consumer
                        .needs_keyframe
                        .store(false, atomic::Ordering::SeqCst);
                    return None;
                }

                if is_discont
                    && !is_keyframe
                    && consumer.wait_for_keyframe.load(atomic::Ordering::SeqCst)
                {
                    // Whenever we have a discontinuity, we need a new keyframe
                    consumer
                        .needs_keyframe
                        .store(true, atomic::Ordering::SeqCst);
                }

                if !is_keyframe && consumer.needs_keyframe.load(atomic::Ordering::SeqCst) {
                    // If we need a keyframe (and this one isn't) request a keyframe upstream
                    if !needs_keyframe_request {
                        gst::debug!(CAT, obj = appsink, "Requesting keyframe for first buffer");
                        needs_keyframe_request = true;
                    }

                    consumer.dropped.fetch_add(1, atomic::Ordering::SeqCst);

                    gst::error!(
                        CAT,
                        obj = appsink,
                        "Ignoring frame for {} while waiting for a keyframe",
                        consumer.appsrc.name()
                    );
                    None
                } else {
                    consumer
                        .needs_keyframe
                        .store(false, atomic::Ordering::SeqCst);
                    consumer.pushed.fetch_add(1, atomic::Ordering::SeqCst);

                    Some(consumer.appsrc.clone())
                }
            })
            .collect::<Vec<_>>();

        drop(consumers);

        if needs_keyframe_request {
            // Do not use `gst_element_send_event()` as it takes the state lock which may lead to dead locks.
            let pad = appsink.static_pad("sink").unwrap();
            pad.push_event(
                gst_video::UpstreamForceKeyUnitEvent::builder()
                    .all_headers(true)
                    .build(),
            );
        }

        for consumer in current_consumers {
            if let Err(err) = consumer.push_sample(&sample) {
                gst::warning!(CAT, obj = appsink, "Failed to push sample: {}", err);
            }
        }
        Ok(gst::FlowSuccess::Ok)
    }

    /// Remove a consumer appsrc by id
    pub fn remove_consumer(&self, consumer: &gst_app::AppSrc) {
        let name = consumer.name();
        if self
            .0
            .consumers
            .lock()
            .unwrap()
            .consumers
            .remove(consumer)
            .is_some()
        {
            gst::debug!(
                CAT,
                obj = &self.0.appsink,
                "Removed consumer {} ({:?})",
                name,
                consumer
            );
            consumer.set_callbacks(gst_app::AppSrcCallbacks::builder().build());
        } else {
            gst::debug!(
                CAT,
                obj = &self.0.appsink,
                "Consumer {} ({:?}) not found",
                name,
                consumer
            );
        }
    }

    /// configure event types the appsink should forward to all its consumers (default: `Eos`).
    pub fn set_forward_events(&self, events_to_forward: impl IntoIterator<Item = gst::EventType>) {
        self.0.consumers.lock().unwrap().events_to_forward =
            events_to_forward.into_iter().collect();
    }

    /// configure whether the preroll sample should be forwarded (default: `true`)
    pub fn set_forward_preroll(&self, forward_preroll: bool) {
        self.0.consumers.lock().unwrap().forward_preroll = forward_preroll;
    }

    /// Get the GStreamer `appsink` wrapped by this producer
    pub fn appsink(&self) -> &gst_app::AppSink {
        &self.0.appsink
    }

    /// Signals an error on all consumers
    pub fn error(&self, error: &gst::glib::Error, debug: Option<&str>) {
        let consumers = self.0.consumers.lock().unwrap();

        for consumer in consumers.consumers.keys() {
            let mut msg_builder =
                gst::message::Error::builder_from_error(error.clone()).src(consumer);
            if let Some(debug) = debug {
                msg_builder = msg_builder.debug(debug);
            }

            let _ = consumer.post_message(msg_builder.build());
        }
    }

    /// The last sample produced by this producer.
    pub fn last_sample(&self) -> Option<gst::Sample> {
        self.0.appsink.property("last-sample")
    }
}

impl<'a> From<&'a gst_app::AppSink> for StreamProducer {
    fn from(appsink: &'a gst_app::AppSink) -> Self {
        let consumers = Arc::new(Mutex::new(StreamConsumers {
            current_latency: None,
            latency_updated: false,
            consumers: HashMap::new(),
            // it would make sense to automatically forward more events such as Tag but that would break
            // with older GStreamer, see https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/4297
            events_to_forward: vec![gst::EventType::Eos, gst::EventType::Gap],
            forward_preroll: true,
            just_forwarded_preroll: false,
        }));

        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(glib::clone!(
                    #[strong]
                    consumers,
                    move |appsink| {
                        let mut consumers = consumers.lock().unwrap();

                        let sample = match appsink.pull_sample() {
                            Ok(sample) => sample,
                            Err(_err) => {
                                gst::debug!(CAT, obj = appsink, "Failed to pull sample");
                                return Err(gst::FlowError::Flushing);
                            }
                        };

                        let just_forwarded_preroll =
                            mem::replace(&mut consumers.just_forwarded_preroll, false);

                        if just_forwarded_preroll {
                            return Ok(gst::FlowSuccess::Ok);
                        }

                        StreamProducer::process_sample(sample, appsink, consumers)
                    }
                ))
                .new_preroll(glib::clone!(
                    #[strong]
                    consumers,
                    move |appsink| {
                        let mut consumers = consumers.lock().unwrap();

                        let sample = match appsink.pull_preroll() {
                            Ok(sample) => sample,
                            Err(_err) => {
                                gst::debug!(CAT, obj = appsink, "Failed to pull preroll");
                                return Err(gst::FlowError::Flushing);
                            }
                        };

                        if consumers.forward_preroll {
                            consumers.just_forwarded_preroll = true;

                            StreamProducer::process_sample(sample, appsink, consumers)
                        } else {
                            Ok(gst::FlowSuccess::Ok)
                        }
                    }
                ))
                .new_event(glib::clone!(
                    #[strong]
                    consumers,
                    move |appsink| {
                        match appsink
                            .pull_object()
                            .map(|obj| obj.downcast::<gst::Event>())
                        {
                            Ok(Ok(event)) => {
                                let (events_to_forward, appsrcs) = {
                                    // clone so we don't keep the lock while pushing events
                                    let consumers = consumers.lock().unwrap();
                                    let events = consumers.events_to_forward.clone();
                                    let appsrcs =
                                        consumers.consumers.keys().cloned().collect::<Vec<_>>();

                                    (events, appsrcs)
                                };

                                if events_to_forward.contains(&event.type_()) {
                                    for appsrc in appsrcs {
                                        appsrc.send_event(event.clone());
                                    }
                                }
                            }
                            Ok(Err(_)) => {} // pulled another unsupported object type, ignore
                            Err(_err) => gst::warning!(CAT, obj = appsink, "Failed to pull event"),
                        }

                        false
                    }
                ))
                .eos(glib::clone!(
                    #[strong]
                    consumers,
                    move |appsink| {
                        let stream_consumers = consumers.lock().unwrap();

                        if stream_consumers
                            .events_to_forward
                            .contains(&gst::EventType::Eos)
                        {
                            let current_consumers = stream_consumers
                                .consumers
                                .values()
                                .map(|c| c.appsrc.clone())
                                .collect::<Vec<_>>();
                            drop(stream_consumers);

                            for consumer in current_consumers {
                                gst::debug!(
                                    CAT,
                                    obj = appsink,
                                    "set EOS on consumer {}",
                                    consumer.name()
                                );
                                let _ = consumer.end_of_stream();
                            }
                        } else {
                            gst::debug!(CAT, obj = appsink, "don't forward EOS to consumers");
                        }
                    }
                ))
                .build(),
        );

        let sinkpad = appsink.static_pad("sink").unwrap();
        let appsink_probe_id = sinkpad
            .add_probe(
                gst::PadProbeType::EVENT_UPSTREAM,
                glib::clone!(
                    #[strong]
                    consumers,
                    move |_pad, info| {
                        let Some(event) = info.event() else {
                            return gst::PadProbeReturn::Ok;
                        };

                        let gst::EventView::Latency(event) = event.view() else {
                            return gst::PadProbeReturn::Ok;
                        };

                        let latency = event.latency();
                        let mut consumers = consumers.lock().unwrap();
                        consumers.current_latency = Some(latency);
                        consumers.latency_updated = true;

                        gst::PadProbeReturn::Ok
                    }
                ),
            )
            .unwrap();

        StreamProducer(Arc::new(StreamProducerInner {
            appsink: appsink.clone(),
            appsink_probe_id: Some(appsink_probe_id),
            consumers,
        }))
    }
}

/// Wrapper around a HashMap of consumers, exists for thread safety
/// and also protects some of the producer state
#[derive(Debug)]
struct StreamConsumers {
    /// The currently-observed latency
    current_latency: Option<gst::ClockTime>,
    /// Whether the consumers' appsrc latency needs updating
    latency_updated: bool,
    /// The consumers, AppSrc pointer value -> consumer
    consumers: HashMap<gst_app::AppSrc, StreamConsumer>,
    /// What events should be forwarded to consumers
    events_to_forward: Vec<gst::EventType>,
    /// Whether the preroll sample should be forwarded at all
    forward_preroll: bool,
    /// Whether we just forwarded the preroll sample. When we did we want to
    /// discard the next sample from on_new_sample as it would cause us to
    /// otherwise push out the same sample twice to consumers.
    just_forwarded_preroll: bool,
}

/// Wrapper around a consumer's `appsrc`
#[derive(Debug)]
struct StreamConsumer {
    /// The GStreamer `appsrc` of the consumer
    appsrc: gst_app::AppSrc,
    /// The id of a pad probe that intercepts force-key-unit events
    fku_probe_id: Option<gst::PadProbeId>,
    /// Whether an initial latency was forwarded to the `appsrc`
    forwarded_latency: atomic::AtomicBool,
    /// Whether a first buffer has made it through, used to determine
    /// whether a new key unit should be requested. Only useful for encoded
    /// streams.
    needs_keyframe: Arc<atomic::AtomicBool>,
    /// number of buffers dropped because `appsrc` internal queue was full
    dropped: Arc<WrappedAtomicU64>,
    /// number of buffers pushed through `appsrc`
    pushed: Arc<WrappedAtomicU64>,
    /// if buffers should not be pushed to the `appsrc` right now
    discard: Arc<atomic::AtomicBool>,
    /// whether the consumer should drop delta frames until next keyframe on discont
    wait_for_keyframe: Arc<atomic::AtomicBool>,
}

impl StreamConsumer {
    /// Create a new consumer
    fn new(
        appsrc: &gst_app::AppSrc,
        fku_probe_id: gst::PadProbeId,
        dropped: Arc<WrappedAtomicU64>,
        pushed: Arc<WrappedAtomicU64>,
        discard: Arc<atomic::AtomicBool>,
        wait_for_keyframe: Arc<atomic::AtomicBool>,
    ) -> Self {
        let needs_keyframe = Arc::new(atomic::AtomicBool::new(
            wait_for_keyframe.load(atomic::Ordering::SeqCst),
        ));
        let needs_keyframe_clone = needs_keyframe.clone();
        let wait_for_keyframe_clone = wait_for_keyframe.clone();
        let dropped_clone = dropped.clone();

        appsrc.set_callbacks(
            gst_app::AppSrcCallbacks::builder()
                .enough_data(move |appsrc| {
                    gst::debug!(
                        CAT,
                        obj = appsrc,
                        "consumer {} ({:?}) is not consuming fast enough, old samples are getting dropped",
                        appsrc.name(),
                        appsrc,
                    );

                    needs_keyframe_clone.store(wait_for_keyframe_clone.load(atomic::Ordering::SeqCst), atomic::Ordering::SeqCst);
                    dropped_clone.fetch_add(1, atomic::Ordering::SeqCst);
                })
                .build(),
        );

        StreamConsumer {
            appsrc: appsrc.clone(),
            fku_probe_id: Some(fku_probe_id),
            forwarded_latency: atomic::AtomicBool::new(false),
            needs_keyframe,
            dropped,
            pushed,
            discard,
            wait_for_keyframe,
        }
    }
}

impl Drop for StreamConsumer {
    fn drop(&mut self) {
        if let Some(fku_probe_id) = self.fku_probe_id.take() {
            let srcpad = self.appsrc.static_pad("src").unwrap();
            srcpad.remove_probe(fku_probe_id);
        }
    }
}

impl PartialEq for StreamConsumer {
    fn eq(&self, other: &Self) -> bool {
        self.appsrc.eq(&other.appsrc)
    }
}

impl Eq for StreamConsumer {}

impl std::hash::Hash for StreamConsumer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.appsrc, state);
    }
}

impl std::borrow::Borrow<gst_app::AppSrc> for StreamConsumer {
    #[inline]
    fn borrow(&self) -> &gst_app::AppSrc {
        &self.appsrc
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{Arc, Mutex},
    };

    use futures::{
        channel::{mpsc, mpsc::Receiver},
        SinkExt, StreamExt,
    };
    use gst::prelude::*;

    use crate::{ConsumptionLink, StreamProducer};

    fn create_producer() -> (
        gst::Pipeline,
        gst_app::AppSrc,
        gst_app::AppSink,
        StreamProducer,
    ) {
        let producer_pipe =
            gst::parse::launch("appsrc name=producer_src ! appsink name=producer_sink")
                .unwrap()
                .downcast::<gst::Pipeline>()
                .unwrap();
        let producer_sink = producer_pipe
            .by_name("producer_sink")
            .unwrap()
            .downcast::<gst_app::AppSink>()
            .unwrap();

        (
            producer_pipe.clone(),
            producer_pipe
                .by_name("producer_src")
                .unwrap()
                .downcast::<gst_app::AppSrc>()
                .unwrap(),
            producer_sink.clone(),
            StreamProducer::from(&producer_sink),
        )
    }

    struct Consumer {
        pipeline: gst::Pipeline,
        src: gst_app::AppSrc,
        sink: gst_app::AppSink,
        receiver: Mutex<Receiver<gst::Sample>>,
        connected: Mutex<bool>,
    }

    impl Consumer {
        fn new(id: &str) -> Self {
            let pipeline = gst::parse::launch(&format!("appsrc name={id} ! appsink name=sink"))
                .unwrap()
                .downcast::<gst::Pipeline>()
                .unwrap();

            let (sender, receiver) = mpsc::channel::<gst::Sample>(1000);
            let sender = Arc::new(Mutex::new(sender));
            let sink = pipeline
                .by_name("sink")
                .unwrap()
                .downcast::<gst_app::AppSink>()
                .unwrap();

            sink.set_callbacks(
                gst_app::AppSinkCallbacks::builder()
                    // Add a handler to the "new-sample" signal.
                    .new_sample(move |appsink| {
                        // Pull the sample in question out of the appsink's buffer.
                        let sender_clone = sender.clone();
                        futures::executor::block_on(
                            sender_clone
                                .lock()
                                .unwrap()
                                .send(appsink.pull_sample().unwrap()),
                        )
                        .unwrap();

                        Ok(gst::FlowSuccess::Ok)
                    })
                    .build(),
            );

            Self {
                pipeline: pipeline.clone(),
                src: pipeline
                    .by_name(id)
                    .unwrap()
                    .downcast::<gst_app::AppSrc>()
                    .unwrap(),
                sink,
                receiver: Mutex::new(receiver),
                connected: Mutex::new(false),
            }
        }

        fn connect(&self, producer: &StreamProducer) -> ConsumptionLink {
            {
                let mut connected = self.connected.lock().unwrap();
                *connected = true;
            }

            producer.add_consumer(&self.src).unwrap()
        }

        fn disconnect(&self, producer: &StreamProducer) {
            {
                let mut connected = self.connected.lock().unwrap();
                *connected = false;
            }

            producer.remove_consumer(&self.src);
        }
    }

    #[test]
    fn simple() {
        gst::init().unwrap();

        let (producer_pipe, producer_src, _producer_sink, producer) = create_producer();
        producer_pipe
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");

        let mut consumers: Vec<Consumer> = Vec::new();
        let consumer = Consumer::new("consumer1");
        let link1 = consumer.connect(&producer);
        consumer
            .pipeline
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");
        consumers.push(consumer);

        let consumer = Consumer::new("consumer2");
        let link2 = consumer.connect(&producer);
        consumer
            .pipeline
            .set_state(gst::State::Playing)
            .expect("Couldn't set producer pipeline state");
        consumers.push(consumer);

        assert!(producer.last_sample().is_none());

        for i in 0..10 {
            let caps = gst::Caps::from_str(&format!("test,n={i}")).unwrap();
            producer_src.set_caps(Some(&caps));
            producer_src.push_buffer(gst::Buffer::new()).unwrap();

            for consumer in &consumers {
                if *consumer.connected.lock().unwrap() {
                    let sample =
                        futures::executor::block_on(consumer.receiver.lock().unwrap().next())
                            .expect("Received an empty buffer?");
                    sample.buffer().expect("No buffer on the sample?");
                    assert_eq!(sample.caps(), Some(caps.as_ref()));
                } else {
                    debug_assert!(
                        consumer
                            .sink
                            .try_pull_sample(gst::ClockTime::from_nseconds(0))
                            .is_none(),
                        "Disconnected consumer got a new sample?!"
                    );
                }
            }

            if i == 5 {
                consumers.first().unwrap().disconnect(&producer);
            }
        }

        assert!(producer.last_sample().is_some());

        assert_eq!(link1.pushed(), 6);
        assert_eq!(link1.dropped(), 0);
        assert_eq!(link2.pushed(), 10);
        assert_eq!(link2.dropped(), 0);
    }
}
