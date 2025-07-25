// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ffi;
#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
use glib::translate::*;

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstTensorDataType")]
pub enum TensorDataType {
    #[doc(alias = "GST_TENSOR_DATA_TYPE_INT4")]
    Int4,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_INT8")]
    Int8,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_INT16")]
    Int16,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_INT32")]
    Int32,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_INT64")]
    Int64,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_UINT4")]
    Uint4,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_UINT8")]
    Uint8,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_UINT16")]
    Uint16,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_UINT32")]
    Uint32,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_UINT64")]
    Uint64,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT16")]
    Float16,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT32")]
    Float32,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT64")]
    Float64,
    #[doc(alias = "GST_TENSOR_DATA_TYPE_BFLOAT16")]
    Bfloat16,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_STRING")]
    String,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_BOOL")]
    Bool,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_COMPLEX64")]
    Complex64,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_COMPLEX128")]
    Complex128,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT8E4M3FN")]
    Float8e4m3fn,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT8E4M3FNUZ")]
    Float8e4m3fnuz,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT8E5M2")]
    Float8e5m2,
    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "GST_TENSOR_DATA_TYPE_FLOAT8E5M2FNUZ")]
    Float8e5m2fnuz,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[doc(hidden)]
impl IntoGlib for TensorDataType {
    type GlibType = ffi::GstTensorDataType;

    fn into_glib(self) -> ffi::GstTensorDataType {
        match self {
            Self::Int4 => ffi::GST_TENSOR_DATA_TYPE_INT4,
            Self::Int8 => ffi::GST_TENSOR_DATA_TYPE_INT8,
            Self::Int16 => ffi::GST_TENSOR_DATA_TYPE_INT16,
            Self::Int32 => ffi::GST_TENSOR_DATA_TYPE_INT32,
            Self::Int64 => ffi::GST_TENSOR_DATA_TYPE_INT64,
            Self::Uint4 => ffi::GST_TENSOR_DATA_TYPE_UINT4,
            Self::Uint8 => ffi::GST_TENSOR_DATA_TYPE_UINT8,
            Self::Uint16 => ffi::GST_TENSOR_DATA_TYPE_UINT16,
            Self::Uint32 => ffi::GST_TENSOR_DATA_TYPE_UINT32,
            Self::Uint64 => ffi::GST_TENSOR_DATA_TYPE_UINT64,
            Self::Float16 => ffi::GST_TENSOR_DATA_TYPE_FLOAT16,
            Self::Float32 => ffi::GST_TENSOR_DATA_TYPE_FLOAT32,
            Self::Float64 => ffi::GST_TENSOR_DATA_TYPE_FLOAT64,
            Self::Bfloat16 => ffi::GST_TENSOR_DATA_TYPE_BFLOAT16,
            #[cfg(feature = "v1_28")]
            Self::String => ffi::GST_TENSOR_DATA_TYPE_STRING,
            #[cfg(feature = "v1_28")]
            Self::Bool => ffi::GST_TENSOR_DATA_TYPE_BOOL,
            #[cfg(feature = "v1_28")]
            Self::Complex64 => ffi::GST_TENSOR_DATA_TYPE_COMPLEX64,
            #[cfg(feature = "v1_28")]
            Self::Complex128 => ffi::GST_TENSOR_DATA_TYPE_COMPLEX128,
            #[cfg(feature = "v1_28")]
            Self::Float8e4m3fn => ffi::GST_TENSOR_DATA_TYPE_FLOAT8E4M3FN,
            #[cfg(feature = "v1_28")]
            Self::Float8e4m3fnuz => ffi::GST_TENSOR_DATA_TYPE_FLOAT8E4M3FNUZ,
            #[cfg(feature = "v1_28")]
            Self::Float8e5m2 => ffi::GST_TENSOR_DATA_TYPE_FLOAT8E5M2,
            #[cfg(feature = "v1_28")]
            Self::Float8e5m2fnuz => ffi::GST_TENSOR_DATA_TYPE_FLOAT8E5M2FNUZ,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[doc(hidden)]
impl FromGlib<ffi::GstTensorDataType> for TensorDataType {
    unsafe fn from_glib(value: ffi::GstTensorDataType) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_TENSOR_DATA_TYPE_INT4 => Self::Int4,
            ffi::GST_TENSOR_DATA_TYPE_INT8 => Self::Int8,
            ffi::GST_TENSOR_DATA_TYPE_INT16 => Self::Int16,
            ffi::GST_TENSOR_DATA_TYPE_INT32 => Self::Int32,
            ffi::GST_TENSOR_DATA_TYPE_INT64 => Self::Int64,
            ffi::GST_TENSOR_DATA_TYPE_UINT4 => Self::Uint4,
            ffi::GST_TENSOR_DATA_TYPE_UINT8 => Self::Uint8,
            ffi::GST_TENSOR_DATA_TYPE_UINT16 => Self::Uint16,
            ffi::GST_TENSOR_DATA_TYPE_UINT32 => Self::Uint32,
            ffi::GST_TENSOR_DATA_TYPE_UINT64 => Self::Uint64,
            ffi::GST_TENSOR_DATA_TYPE_FLOAT16 => Self::Float16,
            ffi::GST_TENSOR_DATA_TYPE_FLOAT32 => Self::Float32,
            ffi::GST_TENSOR_DATA_TYPE_FLOAT64 => Self::Float64,
            ffi::GST_TENSOR_DATA_TYPE_BFLOAT16 => Self::Bfloat16,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_STRING => Self::String,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_BOOL => Self::Bool,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_COMPLEX64 => Self::Complex64,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_COMPLEX128 => Self::Complex128,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_FLOAT8E4M3FN => Self::Float8e4m3fn,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_FLOAT8E4M3FNUZ => Self::Float8e4m3fnuz,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_FLOAT8E5M2 => Self::Float8e5m2,
            #[cfg(feature = "v1_28")]
            ffi::GST_TENSOR_DATA_TYPE_FLOAT8E5M2FNUZ => Self::Float8e5m2fnuz,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstTensorDimOrder")]
pub enum TensorDimOrder {
    #[doc(alias = "GST_TENSOR_DIM_ORDER_ROW_MAJOR")]
    RowMajor,
    #[doc(alias = "GST_TENSOR_DIM_ORDER_COL_MAJOR")]
    ColMajor,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[doc(hidden)]
impl IntoGlib for TensorDimOrder {
    type GlibType = ffi::GstTensorDimOrder;

    #[inline]
    fn into_glib(self) -> ffi::GstTensorDimOrder {
        match self {
            Self::RowMajor => ffi::GST_TENSOR_DIM_ORDER_ROW_MAJOR,
            Self::ColMajor => ffi::GST_TENSOR_DIM_ORDER_COL_MAJOR,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v1_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_26")))]
#[doc(hidden)]
impl FromGlib<ffi::GstTensorDimOrder> for TensorDimOrder {
    #[inline]
    unsafe fn from_glib(value: ffi::GstTensorDimOrder) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_TENSOR_DIM_ORDER_ROW_MAJOR => Self::RowMajor,
            ffi::GST_TENSOR_DIM_ORDER_COL_MAJOR => Self::ColMajor,
            value => Self::__Unknown(value),
        }
    }
}
