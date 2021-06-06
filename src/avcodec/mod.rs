use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::null;

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

include!("codec.rs");
include!("codec_context.rs");
include!("dict.rs");
