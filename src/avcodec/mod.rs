use std::marker::PhantomData;

pub use libav_sys::avcodec::{
    AVCodecHWConfig,
    AVCodecID,
    AVHWDeviceContext,
    AVHWDeviceType,
    AVHWFramesContext,
    AVMediaType,
    AVPixelFormat,
    AVRational
};
use std::ptr::null;
use std::mem::size_of;

include!("codec.rs");
include!("codec_context.rs");
include!("dict.rs");
