// Generated by gir (https://github.com/gtk-rs/gir @ 78e3d3c22343)
// from gir-files (https://github.com/gtk-rs/gir-files @ 5502d32880f5)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ ec8a582cdebb)
// DO NOT EDIT

use gstreamer_tag_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-tag-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstTagDemux",
        Layout {
            size: size_of::<GstTagDemux>(),
            alignment: align_of::<GstTagDemux>(),
        },
    ),
    (
        "GstTagDemuxClass",
        Layout {
            size: size_of::<GstTagDemuxClass>(),
            alignment: align_of::<GstTagDemuxClass>(),
        },
    ),
    (
        "GstTagDemuxResult",
        Layout {
            size: size_of::<GstTagDemuxResult>(),
            alignment: align_of::<GstTagDemuxResult>(),
        },
    ),
    (
        "GstTagImageType",
        Layout {
            size: size_of::<GstTagImageType>(),
            alignment: align_of::<GstTagImageType>(),
        },
    ),
    (
        "GstTagLicenseFlags",
        Layout {
            size: size_of::<GstTagLicenseFlags>(),
            alignment: align_of::<GstTagLicenseFlags>(),
        },
    ),
    (
        "GstTagMux",
        Layout {
            size: size_of::<GstTagMux>(),
            alignment: align_of::<GstTagMux>(),
        },
    ),
    (
        "GstTagMuxClass",
        Layout {
            size: size_of::<GstTagMuxClass>(),
            alignment: align_of::<GstTagMuxClass>(),
        },
    ),
    (
        "GstTagXmpWriterInterface",
        Layout {
            size: size_of::<GstTagXmpWriterInterface>(),
            alignment: align_of::<GstTagXmpWriterInterface>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("GST_TAG_ACOUSTID_FINGERPRINT", "chromaprint-fingerprint"),
    ("GST_TAG_ACOUSTID_ID", "acoustid-id"),
    ("GST_TAG_CAPTURING_CONTRAST", "capturing-contrast"),
    (
        "GST_TAG_CAPTURING_DIGITAL_ZOOM_RATIO",
        "capturing-digital-zoom-ratio",
    ),
    (
        "GST_TAG_CAPTURING_EXPOSURE_COMPENSATION",
        "capturing-exposure-compensation",
    ),
    ("GST_TAG_CAPTURING_EXPOSURE_MODE", "capturing-exposure-mode"),
    (
        "GST_TAG_CAPTURING_EXPOSURE_PROGRAM",
        "capturing-exposure-program",
    ),
    ("GST_TAG_CAPTURING_FLASH_FIRED", "capturing-flash-fired"),
    ("GST_TAG_CAPTURING_FLASH_MODE", "capturing-flash-mode"),
    ("GST_TAG_CAPTURING_FOCAL_LENGTH", "capturing-focal-length"),
    (
        "GST_TAG_CAPTURING_FOCAL_LENGTH_35_MM",
        "capturing-focal-length-35mm",
    ),
    ("GST_TAG_CAPTURING_FOCAL_RATIO", "capturing-focal-ratio"),
    (
        "GST_TAG_CAPTURING_GAIN_ADJUSTMENT",
        "capturing-gain-adjustment",
    ),
    ("GST_TAG_CAPTURING_ISO_SPEED", "capturing-iso-speed"),
    ("GST_TAG_CAPTURING_METERING_MODE", "capturing-metering-mode"),
    ("GST_TAG_CAPTURING_SATURATION", "capturing-saturation"),
    (
        "GST_TAG_CAPTURING_SCENE_CAPTURE_TYPE",
        "capturing-scene-capture-type",
    ),
    ("GST_TAG_CAPTURING_SHARPNESS", "capturing-sharpness"),
    ("GST_TAG_CAPTURING_SHUTTER_SPEED", "capturing-shutter-speed"),
    ("GST_TAG_CAPTURING_SOURCE", "capturing-source"),
    ("GST_TAG_CAPTURING_WHITE_BALANCE", "capturing-white-balance"),
    ("GST_TAG_CDDA_CDDB_DISCID", "discid"),
    ("GST_TAG_CDDA_CDDB_DISCID_FULL", "discid-full"),
    ("GST_TAG_CDDA_MUSICBRAINZ_DISCID", "musicbrainz-discid"),
    (
        "GST_TAG_CDDA_MUSICBRAINZ_DISCID_FULL",
        "musicbrainz-discid-full",
    ),
    ("GST_TAG_CMML_CLIP", "cmml-clip"),
    ("GST_TAG_CMML_HEAD", "cmml-head"),
    ("GST_TAG_CMML_STREAM", "cmml-stream"),
    ("(gint) GST_TAG_DEMUX_RESULT_AGAIN", "1"),
    ("(gint) GST_TAG_DEMUX_RESULT_BROKEN_TAG", "0"),
    ("(gint) GST_TAG_DEMUX_RESULT_OK", "2"),
    ("GST_TAG_ID3V2_HEADER_SIZE", "10"),
    ("GST_TAG_IMAGE_HORIZONTAL_PPI", "image-horizontal-ppi"),
    ("(gint) GST_TAG_IMAGE_TYPE_ARTIST", "6"),
    ("(gint) GST_TAG_IMAGE_TYPE_BACK_COVER", "2"),
    ("(gint) GST_TAG_IMAGE_TYPE_BAND_ARTIST_LOGO", "17"),
    ("(gint) GST_TAG_IMAGE_TYPE_BAND_ORCHESTRA", "8"),
    ("(gint) GST_TAG_IMAGE_TYPE_COMPOSER", "9"),
    ("(gint) GST_TAG_IMAGE_TYPE_CONDUCTOR", "7"),
    ("(gint) GST_TAG_IMAGE_TYPE_DURING_PERFORMANCE", "13"),
    ("(gint) GST_TAG_IMAGE_TYPE_DURING_RECORDING", "12"),
    ("(gint) GST_TAG_IMAGE_TYPE_FISH", "15"),
    ("(gint) GST_TAG_IMAGE_TYPE_FRONT_COVER", "1"),
    ("(gint) GST_TAG_IMAGE_TYPE_ILLUSTRATION", "16"),
    ("(gint) GST_TAG_IMAGE_TYPE_LEAD_ARTIST", "5"),
    ("(gint) GST_TAG_IMAGE_TYPE_LEAFLET_PAGE", "3"),
    ("(gint) GST_TAG_IMAGE_TYPE_LYRICIST", "10"),
    ("(gint) GST_TAG_IMAGE_TYPE_MEDIUM", "4"),
    ("(gint) GST_TAG_IMAGE_TYPE_NONE", "-1"),
    ("(gint) GST_TAG_IMAGE_TYPE_PUBLISHER_STUDIO_LOGO", "18"),
    ("(gint) GST_TAG_IMAGE_TYPE_RECORDING_LOCATION", "11"),
    ("(gint) GST_TAG_IMAGE_TYPE_UNDEFINED", "0"),
    ("(gint) GST_TAG_IMAGE_TYPE_VIDEO_CAPTURE", "14"),
    ("GST_TAG_IMAGE_VERTICAL_PPI", "image-vertical-ppi"),
    (
        "(guint) GST_TAG_LICENSE_CREATIVE_COMMONS_LICENSE",
        "16777216",
    ),
    (
        "(guint) GST_TAG_LICENSE_FREE_SOFTWARE_FOUNDATION_LICENSE",
        "33554432",
    ),
    ("(guint) GST_TAG_LICENSE_PERMITS_DERIVATIVE_WORKS", "4"),
    ("(guint) GST_TAG_LICENSE_PERMITS_DISTRIBUTION", "2"),
    ("(guint) GST_TAG_LICENSE_PERMITS_REPRODUCTION", "1"),
    ("(guint) GST_TAG_LICENSE_PERMITS_SHARING", "8"),
    ("(guint) GST_TAG_LICENSE_PROHIBITS_COMMERCIAL_USE", "65536"),
    (
        "(guint) GST_TAG_LICENSE_PROHIBITS_HIGH_INCOME_NATION_USE",
        "131072",
    ),
    ("(guint) GST_TAG_LICENSE_REQUIRES_ATTRIBUTION", "512"),
    ("(guint) GST_TAG_LICENSE_REQUIRES_COPYLEFT", "4096"),
    ("(guint) GST_TAG_LICENSE_REQUIRES_LESSER_COPYLEFT", "8192"),
    ("(guint) GST_TAG_LICENSE_REQUIRES_NOTICE", "256"),
    ("(guint) GST_TAG_LICENSE_REQUIRES_SHARE_ALIKE", "1024"),
    ("(guint) GST_TAG_LICENSE_REQUIRES_SOURCE_CODE", "2048"),
    ("GST_TAG_MUSICAL_KEY", "musical-key"),
    (
        "GST_TAG_MUSICBRAINZ_ALBUMARTISTID",
        "musicbrainz-albumartistid",
    ),
    ("GST_TAG_MUSICBRAINZ_ALBUMID", "musicbrainz-albumid"),
    ("GST_TAG_MUSICBRAINZ_ARTISTID", "musicbrainz-artistid"),
    (
        "GST_TAG_MUSICBRAINZ_RELEASEGROUPID",
        "musicbrainz-releasegroupid",
    ),
    (
        "GST_TAG_MUSICBRAINZ_RELEASETRACKID",
        "musicbrainz-releasetrackid",
    ),
    ("GST_TAG_MUSICBRAINZ_TRACKID", "musicbrainz-trackid"),
    ("GST_TAG_MUSICBRAINZ_TRMID", "musicbrainz-trmid"),
];
