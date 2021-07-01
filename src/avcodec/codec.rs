use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::null_mut;

use libav_sys::avcodec;
use libav_sys::avcodec::avcodec_get_hw_config;

const EMPTY_STR: &str = "";

pub struct AVCodec {
    pub int_codec: *mut avcodec::AVCodec,
    pub name: &'static str,
    pub long_name: &'static str,
    pub media_type: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: isize,
    pub wrapper_name: &'static str,
}

impl From<*mut avcodec::AVCodec> for AVCodec {
    fn from(int_codec: *mut avcodec::AVCodec) -> Self {
        unsafe {
            Self {
                int_codec,
                name: get_str_or_default((*int_codec).name, EMPTY_STR),
                long_name: get_str_or_default((*int_codec).long_name, EMPTY_STR),
                media_type: (*int_codec).type_,
                id: (*int_codec).id,
                capabilities: (*int_codec).capabilities as isize,
                wrapper_name: get_str_or_default((*int_codec).wrapper_name, EMPTY_STR),
            }
        }
    }
}

pub unsafe fn get_str_or_default(name: *const c_char, default: &str) -> &str {
    if name.is_null() {
        return default;
    }
    return CStr::from_ptr(name).to_str().unwrap_or(default);
}

impl AVCodec {
    pub fn iterator() -> AVCodecIter {
        return AVCodecIter::new();
    }
    pub fn find_decoder(id: avcodec::AVCodecID) -> Option<Self> {
        unsafe {
            let codec = avcodec::avcodec_find_decoder(id);
            return if codec.is_null() {
                None
            } else {
                Some(AVCodec::from(codec))
            };
        }
    }
    pub fn find_encoder(id: avcodec::AVCodecID) -> Option<Self> {
        unsafe {
            let codec = avcodec::avcodec_find_encoder(id);
            return if codec.is_null() {
                None
            } else {
                Some(AVCodec::from(codec))
            };
        }
    }
    pub fn find_decoder_by_name(name: &str) -> Option<Self> {
        unsafe {
            let name = CString::new(name).unwrap();
            let codec = avcodec::avcodec_find_decoder_by_name(name.as_ptr());
            return if codec.is_null() {
                None
            } else {
                Some(AVCodec::from(codec))
            };
        }
    }
    pub fn find_encoder_by_name(name: &str) -> Option<Self> {
        unsafe {
            let name = CString::new(name).unwrap();
            let codec = avcodec::avcodec_find_encoder_by_name(name.as_ptr());
            return if codec.is_null() {
                None
            } else {
                Some(AVCodec::from(codec))
            };
        }
    }

    pub fn is_encoder(&self) -> bool {
        unsafe {
            if avcodec::av_codec_is_encoder(self.int_codec) == 0 {
                return false;
            }
            return true;
        }
    }
    pub fn is_decoder(&self) -> bool {
        unsafe {
            if avcodec::av_codec_is_decoder(self.int_codec) == 0 {
                return false;
            }
            return true;
        }
    }
    pub fn get_hw_config(&self, idx: i32) -> Option<AVCodecHWConfig> {
        unsafe {
            let config = avcodec_get_hw_config(self.int_codec, idx);
            if config.is_null() {
                None
            } else {
                Some(*config)
            }
        }
    }
}


pub struct AVCodecIter {
    opaque: *mut c_void,
}

impl AVCodecIter {
    fn new() -> Self {
        Self {
            opaque: null_mut::<c_void>()
        }
    }
}

impl Iterator for AVCodecIter {
    type Item = AVCodec;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let int_codec = avcodec::av_codec_iterate(&mut self.opaque);
            if int_codec.is_null() {
                return None;
            }
            return Some(AVCodec::from(int_codec as *mut avcodec::AVCodec));
        }
    }
}


#[cfg(test)]
mod tests {
    use libav_sys::avcodec::{av_hwdevice_get_type_name, avcodec_get_hw_config, AVCodecHWConfig, AVCodecID_AV_CODEC_ID_HEVC, AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_TO};
    use libav_sys::avcodec;

    use super::*;

    #[test]
    fn avcodec_iter() {
        for codec in AVCodec::iterator() {
            println!("name: {} long_name: {}, wrapper_name: {}", codec.name, codec.long_name, codec.wrapper_name);
        }
    }

    #[test]
    fn avcodec_find_codec_by_id() {
        let codec = AVCodec::find_encoder(AVCodecID_AV_CODEC_ID_HEVC).unwrap();
        println!("name: {} long_name: {}, wrapper_name: {}", codec.name, codec.long_name, codec.wrapper_name);
    }

    #[test]
    fn avcodec_find_codec_by_name() {
        let codec = AVCodec::find_encoder_by_name("h264_nvenc").unwrap();
        println!("name: {} long_name: {}, wrapper_name: {}", codec.name, codec.long_name, codec.wrapper_name);
    }

    #[test]
    fn avcodec_get_device_type() {
        unsafe {
            let codec = AVCodec::find_encoder_by_name("h264_nvenc").unwrap();
            let mut i = 0;
            loop {
                let hwconfig: *const AVCodecHWConfig = avcodec_get_hw_config(codec.int_codec as *const avcodec::AVCodec, i);
                if hwconfig.is_null() {
                    break;
                }
                i += 1;
                println!("pixel fmt: {}", (*hwconfig).pix_fmt);
                println!("device Type: {}", get_str_or_default(av_hwdevice_get_type_name((*hwconfig).device_type), EMPTY_STR));
            }
        }
    }

    #[test]
    fn avcodec_hw_init() {
        let codec = AVCodec::find_encoder_by_name("h264_nvenc").unwrap();
        let hw_config = codec.get_hw_config(0).unwrap();
        let mut hw_ctx = hwdevice_ctx_create(hw_config.device_type, "", None, 0).unwrap();
        let constraints = hwdevice_get_hwframe_constraints(&mut hw_ctx, None);

        println!("SW formats");

        for fmt in constraints.valid_sw_formats {
            println!("{}", pix_fmt_to_name(fmt))
        }

        println!("HW formats");
        for fmt in constraints.valid_hw_formats {
            println!("{}", pix_fmt_to_name(fmt))
        }
        let mut hw_frame_ctx = hwframe_ctx_alloc(&mut hw_ctx);
        let mut frame_ctx = hw_frame_ctx.get_data().unwrap();
        frame_ctx.height = 2560;
        frame_ctx.width = 1440;
        frame_ctx.format = avcodec::AVPixelFormat_AV_PIX_FMT_CUDA;
        frame_ctx.sw_format = avcodec::AVPixelFormat_AV_PIX_FMT_BGR0;
        hwframe_ctx_init(&mut hw_frame_ctx).unwrap();
        let mut frame = AVFrame::new();
        hwframe_get_buffer(&mut hw_frame_ctx, &mut frame, 0).unwrap();
        unsafe {
            let mut buf: *mut AVPixelFormat = null_mut();
            avcodec::av_hwframe_transfer_get_formats(hw_frame_ctx.internal, AVHWFrameTransferDirection_AV_HWFRAME_TRANSFER_DIRECTION_TO, &mut buf, 0);
            println!("TRANSFER SUPPORTED!");
            for fmt in get_vector(buf) {
                println!("{}", pix_fmt_to_name(fmt))
            }
        }
    }
}

pub struct AVPacket {
    internal: *mut avcodec::AVPacket,
}

impl Drop for AVPacket {
    fn drop(&mut self) {
        unsafe { avcodec::av_packet_free(&mut self.internal); }
    }
}

impl AVPacket {
    #[allow(dead_code)]
    pub fn new() -> Self {
        unsafe {
            let packet = avcodec::av_packet_alloc();
            Self { internal: packet }
        }
    }
    pub fn get_internal(&self) -> &mut avcodec::AVPacket {
        return unsafe { &mut *self.internal };
    }
    pub fn get_data(&self) -> &[u8] {
        unsafe { &(*slice_from_raw_parts(self.get_internal().data, self.get_internal().size as usize)) }
    }
    pub fn unref(&mut self) {
        unsafe {
            avcodec::av_packet_unref(self.internal);
        }
    }
}
