// This example demonstrates how events can be created and sent to the pipeline.
// What this example does is scheduling a timeout on the main loop, and
// sending an EOS message on the bus from there - telling the pipeline
// to shut down. Once that event is processed by everything, the EOS message
// is going to be sent and we catch that one to shut down everything.

// GStreamer's bus is an abstraction layer above an arbitrary main loop.
// This makes sure that GStreamer can be used in conjunction with any existing
// other framework (GUI frameworks, mostly) that operate their own main loops.
// Main idea behind the bus is the simplification between the application and
// GStreamer, because GStreamer is heavily threaded underneath.

// Any thread can post messages to the bus, which is essentially a thread-safe
// queue of messages to process. When a new message was sent to the bus, it
// will wake up the main loop implementation underneath it (which will then
// process the pending messages from the main loop thread).

// An application itself can post messages to the bus aswell.
// This makes it possible, e.g., to schedule an arbitrary piece of code
// to run in the main loop thread - avoiding potential threading issues.

use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    // This creates a pipeline by parsing the gst-launch pipeline syntax.
    let pipeline = gst::parse::launch("audiotestsrc ! identity name=capsmut ! fakesink").unwrap();
    let bus = pipeline.bus().unwrap();

    // This is a contrived example to mutate events. This would normally be code inside an element,
    // which might transform caps to reflect transformation in the data
    let identity = pipeline
        .downcast_ref::<gst::Bin>()
        .unwrap()
        .by_name("capsmut")
        .unwrap();
    let _ = identity.static_pad("sink").unwrap().add_probe(
        gst::PadProbeType::EVENT_DOWNSTREAM,
        move |_, probe_info| {
            let Some(e) = probe_info.event() else {
                return gst::PadProbeReturn::Ok;
            };

            if e.type_() != gst::EventType::Caps {
                return gst::PadProbeReturn::Ok;
            };

            let mut ev = probe_info.take_event().unwrap();
            let ev_ref = ev.make_mut();

            let gst::EventViewMut::Caps(caps) = ev_ref.view_mut() else {
                unreachable!()
            };

            caps.structure_mut().set("custom-field", true);
            identity
                .static_pad("src")
                .unwrap()
                .push_event(ev_ref.to_owned());

            gst::PadProbeReturn::Drop
        },
    );

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Need to move a new reference into the closure.
    // !!ATTENTION!!:
    // It might seem appealing to use pipeline.clone() here, because that greatly
    // simplifies the code within the callback. What this actually does, however, is creating
    // a memory leak. The clone of a pipeline is a new strong reference on the pipeline.
    // Storing this strong reference of the pipeline within the callback (we are moving it in!),
    // which is in turn stored in another strong reference on the pipeline is creating a
    // reference cycle.
    // DO NOT USE pipeline.clone() TO USE THE PIPELINE WITHIN A CALLBACK
    let pipeline_weak = pipeline.downgrade();
    // Add a timeout to the main loop. This closure will be executed
    // in an interval of 5 seconds. The return value of the handler function
    // determines whether the handler still wants to be called:
    // - glib::ControlFlow::Break - stop calling this handler, remove timeout
    // - glib::ControlFlow::Continue- continue calling this handler
    glib::timeout_add_seconds(5, move || {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let Some(pipeline) = pipeline_weak.upgrade() else {
            return glib::ControlFlow::Break;
        };

        println!("sending eos");

        // We create an EndOfStream event here, that tells all elements to drain
        // their internal buffers to their following elements, essentially draining the
        // whole pipeline (front to back). It ensuring that no data is left unhandled and potentially
        // headers were rewritten (e.g. when using something like an MP4 or Matroska muxer).
        // The EOS event is handled directly from this very thread until the first
        // queue element is reached during pipeline-traversal, where it is then queued
        // up and later handled from the queue's streaming thread for the elements
        // following that queue.
        // Once all sinks are done handling the EOS event (and all buffers that were before the
        // EOS event in the pipeline already), the pipeline would post an EOS message on the bus,
        // essentially telling the application that the pipeline is completely drained.
        pipeline.send_event(gst::event::Eos::new());

        // Remove this handler, the pipeline will shutdown anyway, now that we
        // sent the EOS event.
        glib::ControlFlow::Break
    });

    //bus.add_signal_watch();
    //bus.connect_message(None, move |_, msg| {
    let main_loop_clone = main_loop.clone();
    // This sets the bus's signal handler (don't be mislead by the "add", there can only be one).
    // Every message from the bus is passed through this function. Its returnvalue determines
    // whether the handler wants to be called again. If glib::ControlFlow::Break is returned, the
    // handler is removed and will never be called again. The mainloop still runs though.
    let _bus_watch = bus
        .add_watch(move |_, msg| {
            use gst::MessageView;

            let main_loop = &main_loop_clone;
            match msg.view() {
                MessageView::Eos(..) => {
                    println!("received eos");
                    // An EndOfStream event was sent to the pipeline, so we tell our main loop
                    // to stop execution here.
                    main_loop.quit()
                }
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                    main_loop.quit();
                }
                _ => (),
            };

            // Tell the mainloop to continue executing this callback.
            glib::ControlFlow::Continue
        })
        .expect("Failed to add bus watch");

    // Operate GStreamer's bus, facilliating GLib's mainloop here.
    // This function call will block until you tell the mainloop to quit
    // (see above for how to do this).
    main_loop.run();

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
