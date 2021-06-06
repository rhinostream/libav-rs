pub struct AVCodecParameters {
    pub internal: *mut avcodec::AVCodecParameters,
}

impl Drop for AVCodecParameters {
    fn drop(&mut self) {
        unsafe {
            avcodec::avcodec_parameters_free(&mut self.internal);
        }
    }
}

impl AVCodecParameters {
    pub fn new() -> Self {
        unsafe {
            Self {
                internal: avcodec::avcodec_parameters_alloc()
            }
        }
    }

    pub fn from(ctx: &mut AVCodecContext) -> Result<Self, i32> {
        let params = Self::new();
        unsafe {
            let ret = avcodec::avcodec_parameters_from_context(params.internal, ctx.internal);
            if ret < 0 {
                return Err(ret);
            }
        }
        return Ok(params);
    }
}

pub struct AVCodecContext {
    internal: *mut avcodec::AVCodecContext,
}

impl Drop for AVCodecContext {
    fn drop(&mut self) {
        unsafe {
            avcodec::avcodec_free_context(&mut self.internal as *mut *mut avcodec::AVCodecContext);
        }
    }
}

impl AVCodecContext {
    pub fn new(codec: &AVCodec) -> Self {
        Self {
            internal: unsafe { avcodec::avcodec_alloc_context3(codec.int_codec as *const avcodec::AVCodec) }
        }
    }
    pub fn set_parameters(&self, parameters: &mut AVCodecParameters) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_parameters_to_context(self.internal, parameters.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
    pub fn open2(&self, codec: &AVCodec, dict: Option<&mut AVDictionary>) -> Result<i32, i32> {
        unsafe {
            let mut r_dict = null_mut();
            if dict.is_some() {
                    r_dict = &mut dict.unwrap().internal
            }
            let ret = avcodec::avcodec_open2(self.internal, codec.int_codec, r_dict);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
    pub fn send_frame(&self, frame: &AVFrame) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_send_frame(self.internal, frame.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
    pub fn send_packet(&self, pkt: &AVPacket) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_send_packet(self.internal, pkt.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
    pub fn receive_pkt(&self, pkt: &mut AVPacket) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_receive_packet(self.internal, pkt.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
    pub fn receive_frame(&self, frame: &mut AVFrame) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_receive_frame(self.internal, frame.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }

    pub fn get_internal(&self)->&mut avcodec::AVCodecContext{
        return unsafe{&mut *(self.internal)}
    }
}

pub struct AVBufferRef<T> {
    pub internal: *mut avcodec::AVBufferRef,
    phantom: PhantomData<T>,
}

impl<T> From<*mut avcodec::AVBufferRef> for AVBufferRef<T> {
    fn from(internal: *mut avcodec::AVBufferRef) -> Self {
        Self { internal, phantom: Default::default() }
    }
}

impl<T> Drop for AVBufferRef<T> {
    fn drop(&mut self) {
        unsafe { avcodec::av_buffer_unref(&mut self.internal) }
    }
}

impl<T> AVBufferRef<T> {
    pub fn get_data(&self) -> Option<&mut T> {
        unsafe {
            if self.internal.is_null() {
                return None;
            }
            let x: *mut T = (*self.internal).data.cast();
            let x = &mut *x;
            return Some(x);
        }
    }
}


pub struct AVFrame {
    internal: *mut avcodec::AVFrame,
}

impl AVFrame {
    pub fn new() -> Self {
        unsafe {
            Self { internal: avcodec::av_frame_alloc() }
        }
    }
}

impl Drop for AVFrame {
    fn drop(&mut self) {
        unsafe { avcodec::av_frame_free(&mut self.internal) }
    }
}

pub fn hwdevice_ctx_create(typ: AVHWDeviceType, device: &str, opts: Option<&AVDictionary>, flags: i32) -> Result<AVBufferRef<AVHWDeviceContext>, i32> {
    unsafe {
        let mut buf = null_mut();
        let mut raw_opts = null_mut();
        if opts.is_some() {
            raw_opts = opts.unwrap().internal;
        }
        let device = CString::new(device).unwrap();
        let ret = avcodec::av_hwdevice_ctx_create(&mut buf, typ, device.as_ptr(), raw_opts, flags);
        if ret < 0 {
            return Err(ret);
        }

        return Ok(AVBufferRef::from(buf));
    }
}

pub fn hwframe_ctx_alloc(hw_device_ctx: &mut AVBufferRef<AVHWDeviceContext>) -> AVBufferRef<AVHWFramesContext> {
    unsafe {
        let frame_ctx = avcodec::av_hwframe_ctx_alloc(hw_device_ctx.internal);
        AVBufferRef::from(frame_ctx)
    }
}

pub fn hwframe_ctx_init(hw_frame_ctx: &mut AVBufferRef<AVHWFramesContext>) -> Result<i32, i32> {
    unsafe {
        let ret = avcodec::av_hwframe_ctx_init(hw_frame_ctx.internal);
        if ret < 0 {
            Err(ret)
        } else {
            Ok(ret)
        }
    }
}


pub fn hwframe_get_buffer(hw_frame_ctx: &mut AVBufferRef<AVHWFramesContext>, frame: &mut AVFrame, flags: i32) -> Result<i32, i32> {
    unsafe {
        let ret = avcodec::av_hwframe_get_buffer(hw_frame_ctx.internal, frame.internal, flags);
        if ret < 0 {
            Err(ret)
        } else {
            Ok(ret)
        }
    }
}

pub fn hwdevice_get_hwframe_constraints(buf_ref: &mut AVBufferRef<AVHWDeviceContext>, _hwconfig: Option<*const c_void>) -> AVHWFramesConstraints {
    unsafe {
        return AVHWFramesConstraints::from(avcodec::av_hwdevice_get_hwframe_constraints(buf_ref.internal, null()));
    }
}

pub struct AVHWFramesConstraints {
    pub valid_hw_formats: Vec<AVPixelFormat>,
    pub valid_sw_formats: Vec<AVPixelFormat>,

    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
}

impl From<*mut avcodec::AVHWFramesConstraints> for AVHWFramesConstraints {
    fn from(constraints: *mut avcodec::AVHWFramesConstraints) -> Self {
        unsafe {
            let valid_hw_formats = get_vector((*constraints).valid_hw_formats);
            let valid_sw_formats = get_vector((*constraints).valid_sw_formats);

            Self {
                valid_hw_formats,
                valid_sw_formats,
                min_width: (*constraints).min_width,
                min_height: (*constraints).min_height,
                max_width: (*constraints).max_width,
                max_height: (*constraints).max_height,
            }
        }
    }
}


fn get_vector(fmts: *mut AVPixelFormat) -> Vec<AVPixelFormat> {
    unsafe {
        let mut out_fmts = Vec::new();
        let data_size = size_of::<AVPixelFormat>();
        let mut idx = 0;
        loop {
            let fmt = *((fmts as usize + (data_size * idx)) as *mut AVPixelFormat);
            if fmt == avcodec::AVPixelFormat_AV_PIX_FMT_NONE {
                break;
            }
            out_fmts.push(fmt);
            idx += 1;
        }
        return out_fmts;
    }
}

pub fn pix_fmt_to_name(pix_fmt: AVPixelFormat) -> String {
    let mut out = String::new();
    unsafe {
        let buf = avcodec::av_get_pix_fmt_name(pix_fmt);
        let stri = CStr::from_ptr(buf as *mut c_char);
        out.push_str(stri.to_str().unwrap());
        return out;
    }
}
