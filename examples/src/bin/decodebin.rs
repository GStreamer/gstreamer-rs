// This example demonstrates the use of the decodebin element
// The decodebin element tries to automatically detect the incoming
// format and to autoplug the appropriate demuxers / decoders to handle it.
// and decode it to raw audio, video or subtitles.
// Before the pipeline hasn't been prerolled, the decodebin can't possibly know what
// format it gets as its input. So at first, the pipeline looks like this:

// {filesrc} - {decodebin}

// As soon as the decodebin has detected the stream format, it will try to decode every
// contained stream to its raw format.
// The application connects a signal-handler to decodebin's pad-added signal, which tells us
// whenever the decodebin provided us with another contained (raw) stream from the input file.

// This application supports audio and video streams. Video streams are
// displayed using an autovideosink, and audiostreams are played back using autoaudiosink.
// So for a file that contains one audio and one video stream,
// the pipeline looks like the following:

//                        /-[audio]-{audioconvert}-{audioresample}-{autoaudiosink}
// {filesrc}-{decodebin}-|
//                        \-[video]-{viceoconvert}-{videoscale}-{autovideosink}

// Both auto-sinks at the end automatically select the best available (actual) sink. Since the
// selection of available actual sinks is platform specific
// (like using pulseaudio for audio output on linux, e.g.),
// we need to add the audioconvert and audioresample elements before handing the stream to the
// autoaudiosink, because we need to make sure, that the stream is always supported by the actual sink.
// Especially Windows APIs tend to be quite picky about samplerate and sample-format.
// The same applies to videostreams.

use std::{
    env,
    sync::{Arc, Mutex},
};

use anyhow::Error;
use derive_more::derive::{Display, Error};
use gst::{element_error, element_warning, prelude::*};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

#[derive(Clone, Debug, glib::Boxed)]
#[boxed_type(name = "ErrorValue")]
struct ErrorValue(Arc<Mutex<Option<Error>>>);

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: decodebin file_path");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::default();
    let src = gst::ElementFactory::make("filesrc")
        .property("location", uri)
        .build()?;
    let decodebin = gst::ElementFactory::make("decodebin").build()?;

    pipeline.add_many([&src, &decodebin])?;
    gst::Element::link_many([&src, &decodebin])?;

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
    // Connect to decodebin's pad-added signal, that is emitted whenever
    // it found another stream from the input file and found a way to decode it to its raw format.
    // decodebin automatically adds a src-pad for this raw stream, which
    // we can use to build the follow-up pipeline.
    decodebin.connect_pad_added(move |dbin, src_pad| {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let Some(pipeline) = pipeline_weak.upgrade() else {
            return;
        };

        // Try to detect whether the raw stream decodebin provided us with
        // just now is either audio or video (or none of both, e.g. subtitles).
        let (is_audio, is_video) = {
            let media_type = src_pad.current_caps().and_then(|caps| {
                caps.structure(0).map(|s| {
                    let name = s.name();
                    (name.starts_with("audio/"), name.starts_with("video/"))
                })
            });

            match media_type {
                None => {
                    element_warning!(
                        dbin,
                        gst::CoreError::Negotiation,
                        ("Failed to get media type from pad {}", src_pad.name())
                    );

                    return;
                }
                Some(media_type) => media_type,
            }
        };

        // We create a closure here, calling it directly below it, because this greatly
        // improves readability for error-handling. Like this, we can simply use the
        // ?-operator within the closure, and handle the actual error down below where
        // we call the insert_sink(..) closure.
        let insert_sink = |is_audio, is_video| -> Result<(), Error> {
            if is_audio {
                // decodebin found a raw audiostream, so we build the follow-up pipeline to
                // play it on the default audio playback device (using autoaudiosink).
                let queue = gst::ElementFactory::make("queue").build()?;
                let convert = gst::ElementFactory::make("audioconvert").build()?;
                let resample = gst::ElementFactory::make("audioresample").build()?;
                let sink = gst::ElementFactory::make("autoaudiosink").build()?;

                let elements = &[&queue, &convert, &resample, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                // !!ATTENTION!!:
                // This is quite important and people forget it often. Without making sure that
                // the new elements have the same state as the pipeline, things will fail later.
                // They would still be in Null state and can't process data.
                for e in elements {
                    e.sync_state_with_parent()?;
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the audio stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad)?;
            } else if is_video {
                // decodebin found a raw videostream, so we build the follow-up pipeline to
                // display it using the autovideosink.
                let queue = gst::ElementFactory::make("queue").build()?;
                let convert = gst::ElementFactory::make("videoconvert").build()?;
                let scale = gst::ElementFactory::make("videoscale").build()?;
                let sink = gst::ElementFactory::make("autovideosink").build()?;

                let elements = &[&queue, &convert, &scale, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                for e in elements {
                    e.sync_state_with_parent()?
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the video stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad)?;
            }

            Ok(())
        };

        // When adding and linking new elements in a callback fails, error information is often sparse.
        // GStreamer's built-in debugging can be hard to link back to the exact position within the code
        // that failed. Since callbacks are called from random threads within the pipeline, it can get hard
        // to get good error information. The macros used in the following can solve that. With the use
        // of those, one can send arbitrary rust types (using the pipeline's bus) into the mainloop.
        // What we send here is unpacked down below, in the iteration-code over sent bus-messages.
        // Because we are using the failure crate for error details here, we even get a backtrace for
        // where the error was constructed. (If RUST_BACKTRACE=1 is set)
        if let Err(err) = insert_sink(is_audio, is_video) {
            // The following sends a message of type Error on the bus, containing our detailed
            // error information.
            element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                details: gst::Structure::builder("error-details")
                            .field("error",
                                   ErrorValue(Arc::new(Mutex::new(Some(err)))))
                            .build()
            );
        }
    });

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    // This code iterates over all messages that are sent across our pipeline's bus.
    // In the callback ("pad-added" on the decodebin), we sent better error information
    // using a bus message. This is the position where we get those messages and log
    // the contained information.
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;

                match err.details() {
                    // This bus-message of type error contained our custom error-details struct
                    // that we sent in the pad-added callback above. So we unpack it and log
                    // the detailed error information here. details contains a glib::SendValue.
                    // The unpacked error is the converted to a Result::Err, stopping the
                    // application's execution.
                    Some(details) if details.name() == "error-details" => details
                        .get::<&ErrorValue>("error")
                        .unwrap()
                        .clone()
                        .0
                        .lock()
                        .unwrap()
                        .take()
                        .map(Result::Err)
                        .expect("error-details message without actual error"),
                    _ => Err(ErrorMessage {
                        src: msg
                            .src()
                            .map(|s| s.path_string())
                            .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                        error: err.error(),
                        debug: err.debug(),
                    }
                    .into()),
                }?;
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.src().map(|s| s.path_string()),
                    s.old(),
                    s.current(),
                    s.pending()
                );
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}
