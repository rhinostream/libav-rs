use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{null, slice_from_raw_parts};

pub use libav_sys::avcodec::{
    AVCodecHWConfig,
    AVCodecID,
    AVHWDeviceContext,
    AVHWDeviceType,
    AVHWFramesContext,
    AVMediaType,
    AVPixelFormat,
    AVRational,
};
#[allow(unused)]
use log::{error, info};
use crate::avcodec_sys::_float_const;

include!("codec.rs");
include!("codec_context.rs");
include!("dict.rs");
include!("avfilter.rs");
