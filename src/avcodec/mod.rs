use std::marker::PhantomData;

pub use libav_sys::avcodec::{
    AVCodecHWConfig,
    AVCodecID,
    AVCodecParameters,
    AVHWDeviceContext,
    AVHWDeviceType,
    AVHWFramesContext,
    AVMediaType,
    AVPixelFormat,
};
use std::ptr::null;
use std::mem::size_of;

include!("codec.rs");
include!("codec_context.rs");
include!("dict.rs");
