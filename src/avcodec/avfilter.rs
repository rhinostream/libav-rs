pub struct AVFilter {
    internal: *const avcodec::AVFilter,
}

impl AVFilter {
    pub fn get_by_name(name: &str) -> Result<Self, ()> {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let filter = avcodec::avfilter_get_by_name(c_name.as_ptr());
            drop(c_name);
            if filter.is_null() {
                return Err(());
            }

            return Ok(Self {
                internal: filter
            });
        }
    }

    pub fn get_internal(&self) -> &avcodec::AVFilter {
        unsafe {
            return &*self.internal;
        }
    }
}

pub struct AVFilterInOut {
    internal: *mut avcodec::AVFilterInOut,
}

impl Drop for AVFilterInOut {
    fn drop(&mut self) {
        unsafe {
            avcodec::avfilter_inout_free(&mut self.internal)
        }
    }
}

impl AVFilterInOut {
    pub fn new() -> Self {
        unsafe {
            let internal = avcodec::avfilter_inout_alloc();
            return Self {
                internal
            };
        }
    }
    pub fn get_internals(&mut self) -> &mut avcodec::AVFilterInOut {
        unsafe {
            return &mut *self.internal;
        }
    }
}


pub struct AVFilterGraph {
    internal: *mut avcodec::AVFilterGraph,
}

impl Drop for AVFilterGraph {
    fn drop(&mut self) {
        unsafe {
            avcodec::avfilter_graph_free(&mut self.internal);
        }
    }
}

impl AVFilterGraph {
    pub fn new() -> Self {
        unsafe {
            let internal = avcodec::avfilter_graph_alloc();
            return Self {
                internal,
            };
        }
    }
    pub fn create_filter(&mut self, filter: &AVFilter, name: Option<&str>, args: Option<&str>, opaque: *mut c_void) -> Result<AVFilterContext, i32> {
        unsafe {
            let mut filter_ctx = null_mut();
            let mut c_name = null_mut();
            let mut c_args = null_mut();
            if name.is_some() {
                c_name = CString::new(name.unwrap()).unwrap().into_raw();
            }
            if args.is_some() {
                c_args = CString::new(args.unwrap()).unwrap().into_raw();
            }
            let ret = avcodec::avfilter_graph_create_filter(&mut filter_ctx, filter.internal, c_name, c_args, opaque, self.internal);
            if !c_name.is_null() {
                let _ = CString::from_raw(c_name);
            }
            if !c_args.is_null() {
                let _ = CString::from_raw(c_args);
            }

            if ret < 0 {
                return Err(ret);
            }
            return Ok(AVFilterContext { internal: filter_ctx });
        }
    }
    pub fn remove_from_graph(&mut self, ctx: AVFilterContext) {
        unsafe {
            avcodec::avfilter_free(ctx.internal);
        }
    }
    pub fn parse_str(&self, filters: &str, inputs: &mut AVFilterInOut, outputs: &mut AVFilterInOut) -> Result<(), i32> {
        unsafe {
            let c_filters = CString::new(filters).unwrap();
            let ret = avcodec::avfilter_graph_parse_ptr(self.internal, c_filters.as_ptr(), &mut inputs.internal, &mut outputs.internal, null_mut());
            if ret < 0 {
                return Err(ret);
            }
            let _ = c_filters;
        }
        return Ok(());
    }
    pub fn config(&self) -> Result<(), i32> {
        unsafe {
            let ret = avcodec::avfilter_graph_config(self.internal, null_mut());
            if ret < 0 {
                return Err(ret);
            }
            return Ok(());
        }
    }
    pub fn add_frame_flags(&self, buffersrc_ctx: &mut AVFilterContext, frame: &mut AVFrame, flags: i32) -> Result<(), i32> {
        unsafe {
            let fr = avcodec::av_frame_alloc();
            avcodec::av_frame_ref(fr, frame.internal);
            let ret = avcodec::av_buffersrc_add_frame_flags(buffersrc_ctx.internal, fr, flags);
            if ret < 0 {
                return Err(ret);
            }
        }
        return Ok(());
    }
    pub fn get_frame_flags(&self, buffersink_ctx: &mut AVFilterContext, frame: &mut AVFrame, flags: i32) -> Result<(), i32> {
        unsafe {
            let ret = avcodec::av_buffersink_get_frame_flags(buffersink_ctx.internal, frame.internal, flags);
            if ret < 0 {
                return Err(ret);
            }
        }
        return Ok(());
    }

    pub fn buffersrc_set(&self, ctx: &mut AVFilterContext, par: &AVBufferSrcParameters) -> Result<(), i32> {
        unsafe {
            let ret = avcodec::av_buffersrc_parameters_set(ctx.internal, par.internal);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(())
            }
        }
    }

    pub fn buffersink_get_hw_frames_ctx(&self, ctx: &AVFilterContext) -> AVBufferRef<AVHWFramesContext> {
        unsafe {
            return AVBufferRef::from(avcodec::av_buffersink_get_hw_frames_ctx(ctx.internal));
        }
    }
}

pub struct AVFilterContext {
    internal: *mut avcodec::AVFilterContext,
}

impl AVFilterContext {
    pub fn set_hw_device_ctx(&mut self, device_ctx: &AVBufferRef<AVHWDeviceContext>) {
        unsafe {
            (*self.internal).hw_device_ctx = avcodec::av_buffer_ref(device_ctx.internal);
        }
    }
    pub fn get_internal(&mut self) -> &mut avcodec::AVFilterContext {
        unsafe {
            return &mut *self.internal;
        }
    }
}

pub fn av_strdup(s: &str) -> *mut c_char {
    unsafe {
        let str = CString::new(s).unwrap();
        return avcodec::av_strdup(str.as_ptr());
    }
}

pub fn av_strfree(s: *mut c_char) {
    unsafe {
        avcodec::av_free(s as *mut c_void);
    }
}

impl AVFilterContext {
    #[allow(unused)]
    fn opt_set_int_list(&self, opt_name: &str, list: Vec<i32>, search_flags: i32) -> Result<(), i32> {
        unsafe {
            let c_name = CString::new(opt_name).unwrap();
            let ret = avcodec::av_opt_set_bin(self.internal as *mut c_void, c_name.as_ptr(), list.as_ptr() as *const u8, (list.len() * size_of::<i32>()) as i32, search_flags);
            if ret < 0 {
                return Err(ret);
            }
        }
        return Ok(());
    }
}

pub struct AVBufferSrcParameters {
    internal: *mut avcodec::AVBufferSrcParameters,
}


impl Drop for AVBufferSrcParameters {
    fn drop(&mut self) {
        unsafe { avcodec::av_freep(&mut self.internal as *mut _ as *mut c_void) }
    }
}

impl AVBufferSrcParameters {
    pub fn new() -> Self {
        unsafe {
            Self {
                internal: avcodec::av_buffersrc_parameters_alloc(),
            }
        }
    }
    pub fn get_mut(&mut self) -> &mut avcodec::AVBufferSrcParameters {
        unsafe {
            &mut *self.internal
        }
    }
    pub fn set_hw_frames_context(&mut self, buf: &AVBufferRef<AVHWFramesContext>) {
        unsafe { (*self.internal).hw_frames_ctx = avcodec::av_buffer_ref(buf.internal); }
    }
}

#[cfg(test)]
pub mod test_filter {
    use super::*;

    #[test]
    pub fn test_filters() {
        let mut graph = AVFilterGraph::new();
        let buffer_src = AVFilter::get_by_name("buffer").unwrap();
        let buffer_sink = AVFilter::get_by_name("buffersink").unwrap();
        let buffer_src_ctx = graph.create_filter(&buffer_src, Some("in"),
                                                 Some(format!("video_size=3840x2160:pix_fmt={}:time_base=1/60", avcodec::AVPixelFormat_AV_PIX_FMT_CUDA).as_str()), null_mut());
        if buffer_src_ctx.is_err() {
            let err = buffer_src_ctx.err().unwrap();
            println!("error: {}", err_str(err));
            panic!();
        }
        let mut buffer_src_ctx = buffer_src_ctx.unwrap();
        let mut buffer_sink_ctx = graph.create_filter(&buffer_sink, Some("out"), None, null_mut()).unwrap();
        let mut inputs = AVFilterInOut::new();
        let mut outputs = AVFilterInOut::new();


        let out_ = outputs.get_internals();

        out_.name = av_strdup("in");
        out_.filter_ctx = buffer_src_ctx.internal;
        out_.pad_idx = 0;
        out_.next = null_mut();

        let in_ = inputs.get_internals();
        in_.name = av_strdup("out");
        in_.filter_ctx = buffer_sink_ctx.internal;
        in_.pad_idx = 0;
        in_.next = null_mut();

        let mut device = hwdevice_ctx_create(avcodec::AVHWDeviceType_AV_HWDEVICE_TYPE_CUDA, "", None, 0).unwrap();
        let mut hw_frames_ctx = hwframe_ctx_alloc(&mut device);
        let mut frame_ctx = hw_frames_ctx.get_data().unwrap();
        frame_ctx.height = 2560;
        frame_ctx.width = 1440;
        frame_ctx.format = avcodec::AVPixelFormat_AV_PIX_FMT_CUDA;
        frame_ctx.sw_format = avcodec::AVPixelFormat_AV_PIX_FMT_YUV444P;
        hwframe_ctx_init(&mut hw_frames_ctx).unwrap();
        graph.parse_str("scale_cuda=1280:720", &mut inputs, &mut outputs).unwrap();
        let mut params_t = AVBufferSrcParameters::new();
        params_t.set_hw_frames_context(&hw_frames_ctx);
        graph.buffersrc_set(&mut buffer_src_ctx, &params_t).unwrap();
        assert_eq!(0, graph.config().err().unwrap_or(0));
    }
}
