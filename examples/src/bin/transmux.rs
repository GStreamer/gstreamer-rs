// This sample demonstrates the use of manual typefinding, as well as
// the multiqueue element.
// The example takes a URI and an output file as input.
// The URI is used to construct a source element.
// To the beginning, a typefind element is plugged after the src to detect
// the container type:

// {src} - {typefind}

// After the typefind element reported the detected type, the example manually
// selects an appropriate demuxer (from a list of very few supported formats).
// The demuxer is plugged into the pipeline, every stream is linked to the multiqueue
// and piped into the matroskamux at the end of the pipeline.
// The result is then sent into the filesink. Running pipeline after typefind:

//                                  /-[audio]-\
// {src} - {typefind} - {demuxer} -|           {multiqueue} - {matroskamux} - {filesink}
//                                  \-[video]-/

use std::env;

use anyhow::Error;
use derive_more::derive::{Display, Error};
use gst::{element_error, prelude::*};

#[path = "../examples-common.rs"]
mod examples_common;

#[derive(Debug, Display, Error)]
#[display("Received error from {src}: {error} (debug: {debug:?})")]
struct ErrorMessage {
    src: glib::GString,
    error: glib::Error,
    debug: Option<glib::GString>,
}

fn example_main() -> Result<(), Error> {
    gst::init()?;

    let args: Vec<_> = env::args().collect();
    let uri: &str;
    let output_file: &str;

    if args.len() == 3 {
        uri = args[1].as_ref();
        output_file = args[2].as_ref();
    } else {
        println!("Usage: multiqueue URI output_file");
        std::process::exit(-1)
    };

    let pipeline = gst::Pipeline::default();
    let src = gst::Element::make_from_uri(gst::URIType::Src, uri, None)
        .expect("We do not seem to support this uri");
    let typefinder = gst::ElementFactory::make("typefind").build()?;
    let queue = gst::ElementFactory::make("multiqueue")
        .property("max-size-buffers", 0u32)
        .property("max-size-time", 0u64)
        .property("max-size-bytes", 1024u32 * 1024 * 100)
        .build()?;
    let muxer = gst::ElementFactory::make("matroskamux").build()?;
    let sink = gst::ElementFactory::make("filesink")
        .property("location", output_file)
        .build()?;

    // Increase the queue capacity to 100MB to avoid a stalling pipeline

    pipeline
        .add_many([&src, &typefinder, &queue, &muxer, &sink])
        .expect("failed to add elements to pipeline");

    src.link(&typefinder)?;
    muxer.link(&sink)?;

    let pipeline_clone = pipeline.clone();
    let typefinder_clone = typefinder.clone();
    typefinder.connect("have-type", false, move |values| {
        let (pipeline, typefinder) = (&pipeline_clone, &typefinder_clone);

        // Use the detected format to select between a small set of supported demuxers
        // Hint: This should probably never be done manually, for stuff like this,
        // the decodebin should be used, that does this stuff automatically and handles
        // much more corner-cases. This is just for the sake of being an example.
        let caps = values[2]
            .get::<gst::Caps>()
            .expect("typefinder \"have-type\" signal values[2]");
        let format_name = caps.structure(0).expect("Failed to get format name").name();

        let demuxer = match format_name.as_str() {
            "video/x-matroska" | "video/webm" => gst::ElementFactory::make("matroskademux")
                .build()
                .expect("matroskademux missing"),
            "video/quicktime" => gst::ElementFactory::make("qtdemux")
                .build()
                .expect("qtdemux missing"),
            _ => {
                eprintln!("Sorry, this format is not supported by this example.");
                std::process::exit(-1);
            }
        };

        // We found a supported format and created the appropriate demuxer -> link it
        pipeline
            .add(&demuxer)
            .expect("Failed to build remux pipeline");
        // We simply keep the typefinder element and pipe the data through it.
        // Removing is non-trivial since it started reading data from the pipeline
        // that the next element (the format specific demuxer) would need.
        typefinder
            .link(&demuxer)
            .expect("Failed to build remux pipeline");

        let queue_clone = queue.clone();
        let muxer_clone = muxer.clone();
        demuxer.connect_pad_added(move |demux, src_pad| {
            handle_demux_pad_added(demux, src_pad, &queue_clone, &muxer_clone)
        });
        demuxer
            .sync_state_with_parent()
            .expect("Failed to build remux pipeline");

        None
    });

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;

                return Err(ErrorMessage {
                    src: msg
                        .src()
                        .map(|s| s.path_string())
                        .unwrap_or_else(|| glib::GString::from("UNKNOWN")),
                    error: err.error(),
                    debug: err.debug(),
                }
                .into());
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

// This is the callback function called by the demuxer, when a new stream was detected.
fn handle_demux_pad_added(
    demuxer: &gst::Element,
    demux_src_pad: &gst::Pad,
    queue: &gst::Element,
    muxer: &gst::Element,
) {
    // Pipe the detected stream through our multiqueue to the muxer.
    // For that, we need to request a sink pad that fits our needs.
    let link_to_muxer = || -> Result<(), Error> {
        let queue_sink_pad = queue
            .request_pad_simple("sink_%u")
            .expect("If this happened, something is terribly wrong");
        demux_src_pad.link(&queue_sink_pad)?;
        // Now that we requested a sink pad fitting our needs from the multiqueue,
        // the multiqueue automatically created a fitting src pad on the other side.
        // sink and src pad are linked internally, so we can iterate this internal link chain
        // and dependably retrieve the src pad corresponding to our requested sink pad.
        let queue_src_pad = queue_sink_pad
            .iterate_internal_links()
            .next()?
            .expect("Failed to iterate the multiqueue's internal link chain");

        // Link the multiqueue's output for this stream to the matroskamuxer.
        // For that, we request an appropriate pad at the muxer, that fits our needs.
        let muxer_sink_pad = muxer
            .compatible_pad(&queue_src_pad, None)
            .expect("Aww, you found a format that matroska doesn't support!");
        queue_src_pad.link(&muxer_sink_pad)?;

        Ok(())
    };

    if let Err(err) = link_to_muxer() {
        element_error!(
            demuxer,
            gst::LibraryError::Failed,
            ("Failed to insert sink"),
            ["{}", err]
        );
    }
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    match examples_common::run(example_main) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {e}"),
    }
}
