// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#![cfg(unix)]

use gstreamer_analytics_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-analytics-1.0"];

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
            return Err(format!("compilation command {cmd:?} failed, {status}").into());
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
        Err(err) => Err(format!("{name} {err}").into()),
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
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
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
        let (name, value) = l.split_once(';').expect("Missing ';' separator");
        c_constants.push((name.to_owned(), value.to_owned()));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {rust_name}\nRust: {rust_value:?}\nC:    {c_value:?}",
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
        let (name, value) = l.split_once(';').expect("Missing first ';' separator");
        let (size, alignment) = value.split_once(';').expect("Missing second ';' separator");
        let size = size.parse().expect("Failed to parse size");
        let alignment = alignment.parse().expect("Failed to parse alignment");
        c_layouts.push((name.to_owned(), Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {rust_name:?}\nC:    {c_name:?}");
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!("Layout mismatch for {rust_name}\nRust: {rust_layout:?}\nC:    {c_layout:?}",);
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

    let mut cmd = Command::new(exe);
    cmd.stderr(Stdio::inherit());
    let out = cmd.output()?;
    if !out.status.success() {
        let (status, stdout) = (out.status, String::from_utf8_lossy(&out.stdout));
        return Err(format!("command {cmd:?} failed, {status:?}\nstdout: {stdout}").into());
    }

    Ok(String::from_utf8(out.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstAnalyticsBatchBuffer",
        Layout {
            size: size_of::<GstAnalyticsBatchBuffer>(),
            alignment: align_of::<GstAnalyticsBatchBuffer>(),
        },
    ),
    (
        "GstAnalyticsBatchMeta",
        Layout {
            size: size_of::<GstAnalyticsBatchMeta>(),
            alignment: align_of::<GstAnalyticsBatchMeta>(),
        },
    ),
    (
        "GstAnalyticsBatchStream",
        Layout {
            size: size_of::<GstAnalyticsBatchStream>(),
            alignment: align_of::<GstAnalyticsBatchStream>(),
        },
    ),
    (
        "GstAnalyticsClsMtd",
        Layout {
            size: size_of::<GstAnalyticsClsMtd>(),
            alignment: align_of::<GstAnalyticsClsMtd>(),
        },
    ),
    (
        "GstAnalyticsMtd",
        Layout {
            size: size_of::<GstAnalyticsMtd>(),
            alignment: align_of::<GstAnalyticsMtd>(),
        },
    ),
    (
        "GstAnalyticsMtdImpl",
        Layout {
            size: size_of::<GstAnalyticsMtdImpl>(),
            alignment: align_of::<GstAnalyticsMtdImpl>(),
        },
    ),
    (
        "GstAnalyticsMtdType",
        Layout {
            size: size_of::<GstAnalyticsMtdType>(),
            alignment: align_of::<GstAnalyticsMtdType>(),
        },
    ),
    (
        "GstAnalyticsODMtd",
        Layout {
            size: size_of::<GstAnalyticsODMtd>(),
            alignment: align_of::<GstAnalyticsODMtd>(),
        },
    ),
    (
        "GstAnalyticsRelTypes",
        Layout {
            size: size_of::<GstAnalyticsRelTypes>(),
            alignment: align_of::<GstAnalyticsRelTypes>(),
        },
    ),
    (
        "GstAnalyticsRelationMetaInitParams",
        Layout {
            size: size_of::<GstAnalyticsRelationMetaInitParams>(),
            alignment: align_of::<GstAnalyticsRelationMetaInitParams>(),
        },
    ),
    (
        "GstAnalyticsSegmentationMtd",
        Layout {
            size: size_of::<GstAnalyticsSegmentationMtd>(),
            alignment: align_of::<GstAnalyticsSegmentationMtd>(),
        },
    ),
    (
        "GstAnalyticsTrackingMtd",
        Layout {
            size: size_of::<GstAnalyticsTrackingMtd>(),
            alignment: align_of::<GstAnalyticsTrackingMtd>(),
        },
    ),
    (
        "GstSegmentationType",
        Layout {
            size: size_of::<GstSegmentationType>(),
            alignment: align_of::<GstSegmentationType>(),
        },
    ),
    (
        "GstTensor",
        Layout {
            size: size_of::<GstTensor>(),
            alignment: align_of::<GstTensor>(),
        },
    ),
    (
        "GstTensorDataType",
        Layout {
            size: size_of::<GstTensorDataType>(),
            alignment: align_of::<GstTensorDataType>(),
        },
    ),
    (
        "GstTensorDimOrder",
        Layout {
            size: size_of::<GstTensorDimOrder>(),
            alignment: align_of::<GstTensorDimOrder>(),
        },
    ),
    (
        "GstTensorLayout",
        Layout {
            size: size_of::<GstTensorLayout>(),
            alignment: align_of::<GstTensorLayout>(),
        },
    ),
    (
        "GstTensorMeta",
        Layout {
            size: size_of::<GstTensorMeta>(),
            alignment: align_of::<GstTensorMeta>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("GST_ANALYTICS_MTD_TYPE_ANY", "0"),
    ("(guint) GST_ANALYTICS_REL_TYPE_ANY", "2147483647"),
    ("(guint) GST_ANALYTICS_REL_TYPE_CONTAIN", "4"),
    ("(guint) GST_ANALYTICS_REL_TYPE_IS_PART_OF", "2"),
    ("(guint) GST_ANALYTICS_REL_TYPE_NONE", "0"),
    ("(guint) GST_ANALYTICS_REL_TYPE_N_TO_N", "16"),
    ("(guint) GST_ANALYTICS_REL_TYPE_RELATE_TO", "8"),
    (
        "GST_CAPS_FEATURE_META_GST_ANALYTICS_BATCH_META",
        "meta:GstAnalyticsBatchMeta",
    ),
    ("GST_INF_RELATION_SPAN", "-1"),
    ("(gint) GST_SEGMENTATION_TYPE_INSTANCE", "1"),
    ("(gint) GST_SEGMENTATION_TYPE_SEMANTIC", "0"),
    ("(gint) GST_TENSOR_DATA_TYPE_BFLOAT16", "13"),
    ("(gint) GST_TENSOR_DATA_TYPE_BOOL", "15"),
    ("(gint) GST_TENSOR_DATA_TYPE_COMPLEX128", "17"),
    ("(gint) GST_TENSOR_DATA_TYPE_COMPLEX64", "16"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT16", "10"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT32", "11"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT64", "12"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT8E4M3FN", "18"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT8E4M3FNUZ", "19"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT8E5M2", "20"),
    ("(gint) GST_TENSOR_DATA_TYPE_FLOAT8E5M2FNUZ", "21"),
    ("(gint) GST_TENSOR_DATA_TYPE_INT16", "2"),
    ("(gint) GST_TENSOR_DATA_TYPE_INT32", "3"),
    ("(gint) GST_TENSOR_DATA_TYPE_INT4", "0"),
    ("(gint) GST_TENSOR_DATA_TYPE_INT64", "4"),
    ("(gint) GST_TENSOR_DATA_TYPE_INT8", "1"),
    ("(gint) GST_TENSOR_DATA_TYPE_STRING", "14"),
    ("(gint) GST_TENSOR_DATA_TYPE_UINT16", "7"),
    ("(gint) GST_TENSOR_DATA_TYPE_UINT32", "8"),
    ("(gint) GST_TENSOR_DATA_TYPE_UINT4", "5"),
    ("(gint) GST_TENSOR_DATA_TYPE_UINT64", "9"),
    ("(gint) GST_TENSOR_DATA_TYPE_UINT8", "6"),
    ("(gint) GST_TENSOR_DIM_ORDER_COL_MAJOR", "1"),
    ("(gint) GST_TENSOR_DIM_ORDER_ROW_MAJOR", "0"),
    ("(gint) GST_TENSOR_LAYOUT_CONTIGUOUS", "0"),
];
