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
            let ret = avcodec::av_buffersrc_add_frame_flags(buffersrc_ctx.internal, frame.get_internal(), flags);
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
}

pub struct AVFilterContext {
    internal: *mut avcodec::AVFilterContext,
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

#[cfg(test)]
pub mod test_filter {
    use super::*;

    #[test]
    pub fn test_filters() {
        let mut graph = AVFilterGraph::new();
        let buffer_src = AVFilter::get_by_name("buffer").unwrap();
        let buffer_sink = AVFilter::get_by_name("buffersink").unwrap();
        let buffer_src_ctx = graph.create_filter(&buffer_src, Some("in"),
                                                 Some(format!("video_size=3840x2160:pix_fmt={}:time_base=1/60:pixel_aspect=3840/2160", avcodec::AVPixelFormat_AV_PIX_FMT_CUDA).as_str()), null_mut());
        if buffer_src_ctx.is_err() {
            let err = buffer_src_ctx.err().unwrap();
            println!("error: {}", err_str(err));
            panic!();
        }
        let buffer_src_ctx = buffer_src_ctx.unwrap();
        let _buffer_sink_ctx = graph.create_filter(&buffer_sink, Some("out"), Some(format!("pix_fmts={}", avcodec::AVPixelFormat_AV_PIX_FMT_NV12).as_str()), null_mut());
        let mut inputs = AVFilterInOut::new();
        let mut outputs = AVFilterInOut::new();

        let inp_int = inputs.get_internals();

        inp_int.name = av_strdup("in");
        inp_int.filter_ctx = buffer_src_ctx.internal;
        inp_int.pad_idx = 0;
        inp_int.next = null_mut();

        let out_int = outputs.get_internals();
        out_int.name = av_strdup("out");
        out_int.filter_ctx = buffer_src_ctx.internal;
        out_int.pad_idx = 0;
        out_int.next = null_mut();

        graph.parse_str("scale_npp=1280:720", &mut inputs, &mut outputs).unwrap();
    }
}
