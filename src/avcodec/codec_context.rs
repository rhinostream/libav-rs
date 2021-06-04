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
    pub fn new(codec: AVCodec) -> Self {
        Self {
            internal: unsafe { avcodec::avcodec_alloc_context3(codec.int_codec as *const avcodec::AVCodec) }
        }
    }
    pub fn set_parameters(&self, parameters: &mut AVCodecParameters) -> Result<i32, i32> {
        unsafe {
            let ret = avcodec::avcodec_parameters_to_context(self.internal, parameters);
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
                r_dict = dict.unwrap().internal
            }
            let ret = avcodec::avcodec_open2(self.internal, codec.int_codec, &mut r_dict);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }
}

pub struct AVBufferRef<T> {
    internal: *mut avcodec::AVBufferRef,
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
        let ret = av_hwframe_get_buffer(hw_frame_ctx.internal, frame.internal, flags);
        if ret < 0 {
            Err(ret)
        } else {
            Ok(ret)
        }
    }
}
