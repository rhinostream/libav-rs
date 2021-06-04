pub use libav_sys::avcodec::{
    AVCodecHWConfig,
    AVCodecID,
    AVHWDeviceType,
    AVMediaType,
    AVPixelFormat,
    AVCodecParameters,
};
use libav_sys::avcodec::{av_hwframe_get_buffer, AVHWDeviceContext, AVHWFramesContext};
use std::marker::PhantomData;
include!("codec.rs");
include!("codec_context.rs");
include!("dict.rs");
