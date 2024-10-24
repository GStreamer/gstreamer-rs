// This example demonstrates the use of GStreamer's pad probe APIs.
// Probes are callbacks that can be installed by the application and will notify
// the application about the states of the dataflow. Those are mostly used for
// changing pipelines dynamically at runtime or for inspecting/modifying buffers or events

//                 |-[probe]
//                /
// {audiotestsrc} - {fakesink}
#![allow(clippy::question_mark)]

use byte_slice_cast::*;
use gst::prelude::*;

#[path = "../examples-common.rs"]
mod examples_common;

fn example_main() {
    gst::init().unwrap();

    // Parse the pipeline we want to probe from a static in-line string.
    // Here we give our audiotestsrc a name, so we can retrieve that element
    // from the resulting pipeline.
    let pipeline = gst::parse::launch(&format!(
        "audiotestsrc name=src ! audio/x-raw,format={},channels=1 ! fakesink",
        gst_audio::AUDIO_FORMAT_S16
    ))
    .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    // Get the audiotestsrc element from the pipeline that GStreamer
    // created for us while parsing the launch syntax above.
    let src = pipeline.by_name("src").unwrap();
    // Get the audiotestsrc's src-pad.
    let src_pad = src.static_pad("src").unwrap();
    // Add a probe handler on the audiotestsrc's src-pad.
    // This handler gets called for every buffer that passes the pad we probe.
    src_pad.add_probe(gst::PadProbeType::BUFFER, |_, probe_info| {
        // Interpret the data sent over the pad as one buffer
        let Some(buffer) = probe_info.buffer() else {
            return gst::PadProbeReturn::Ok;
        };

        // At this point, buffer is only a reference to an existing memory region somewhere.
        // When we want to access its content, we have to map it while requesting the required
        // mode of access (read, read/write).
        // This type of abstraction is necessary, because the buffer in question might not be
        // on the machine's main memory itself, but rather in the GPU's memory.
        // So mapping the buffer makes the underlying memory region accessible to us.
        // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
        let map = buffer.map_readable().unwrap();

        // We know what format the data in the memory region has, since we requested
        // it by setting the appsink's caps. So what we do here is interpret the
        // memory region we mapped as an array of signed 16 bit integers.
        let samples = if let Ok(samples) = map.as_slice_of::<i16>() {
            samples
        } else {
            return gst::PadProbeReturn::Ok;
        };

        // For buffer (= chunk of samples), we calculate the root mean square:
        let sum: f64 = samples
            .iter()
            .map(|sample| {
                let f = f64::from(*sample) / f64::from(i16::MAX);
                f * f
            })
            .sum();
        let rms = (sum / (samples.len() as f64)).sqrt();
        println!("rms: {rms}");

        gst::PadProbeReturn::Ok
    });

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn main() {
    // examples_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    examples_common::run(example_main);
}
