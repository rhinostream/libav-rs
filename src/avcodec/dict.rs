use std::fmt::{Display, Formatter};
use std::fmt;
use std::ptr::copy;


pub struct AVDictionary {
    internal: *mut avcodec::AVDictionary,
}

impl Drop for AVDictionary {
    fn drop(&mut self) {
        unsafe {
            avcodec::av_dict_free(&mut self.internal);
        }
    }
}

pub struct AVDictionaryEntry {
    internal: *mut avcodec::AVDictionaryEntry,
    pub key: &'static str,
    pub val: &'static str,
}

impl From<*mut avcodec::AVDictionaryEntry> for AVDictionaryEntry {
    fn from(internal: *mut avcodec::AVDictionaryEntry) -> Self {
        unsafe {
            Self {
                internal,
                key: get_str_or_default((*internal).key, ""),
                val: get_str_or_default((*internal).value, ""),
            }
        }
    }
}

unsafe fn terminate_cstr(cstr: *mut c_char, len: usize) {
    *((cstr as usize + len) as *mut c_char) = '\0' as c_char;
}

impl Display for AVDictionary {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_string(':', ' ').unwrap_or(String::from("None")))
    }
}

impl AVDictionary {
    pub fn new() -> Self {
        Self {
            internal: null_mut()
        }
    }

    pub fn parse_str(&mut self, parse_str: &str, key_val_sep: &str, pairs_sep: &str, flags: i32) -> Result<i32, i32> {
        unsafe {
            let new_parse_str: *mut c_char = avcodec::av_malloc((parse_str.len() + 1) as u64) as *mut c_char;
            copy(parse_str.as_ptr() as *const c_char, new_parse_str, parse_str.len());
            terminate_cstr(new_parse_str, parse_str.len());

            let r_key_val_sep = avcodec::av_malloc((key_val_sep.len() + 1) as u64) as *mut c_char;
            let r_pairs_sep = avcodec::av_malloc((pairs_sep.len() + 1) as u64) as *mut c_char;

            copy(key_val_sep.as_ptr() as *const c_char, r_key_val_sep, key_val_sep.len());
            copy(pairs_sep.as_ptr() as *const c_char, r_pairs_sep, pairs_sep.len());

            terminate_cstr(r_key_val_sep, key_val_sep.len());
            terminate_cstr(r_pairs_sep, pairs_sep.len());

            let ret = avcodec::av_dict_parse_string(&mut self.internal, new_parse_str, r_key_val_sep, r_pairs_sep, flags);
            avcodec::av_free(new_parse_str as *mut c_void);
            avcodec::av_free(r_key_val_sep as *mut c_void);
            avcodec::av_free(r_pairs_sep as *mut c_void);
            if ret < 0 {
                Err(ret)
            } else {
                Ok(ret)
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str, flags: i32) -> Result<i32, i32> {
        unsafe {
            let new_key: *mut c_char = avcodec::av_malloc((key.len() + 1) as u64) as *mut c_char;
            let new_val: *mut c_char = avcodec::av_malloc((value.len() + 1) as u64) as *mut c_char;
            copy(key.as_ptr() as *const c_char, new_key, key.len());
            copy(value.as_ptr() as *const c_char, new_val, value.len());

            terminate_cstr(new_key, key.len());
            terminate_cstr(new_val, value.len());

            let ret = avcodec::av_dict_set(&mut self.internal, new_key, new_val, flags);
            if ret >= 0 {
                Ok(ret)
            } else {
                Err(ret)
            }
        }
    }
    pub fn set_int(&mut self, key: &str, value: i64, flags: i32) -> Result<i32, i32> {
        unsafe {
            let new_key: *mut c_char = avcodec::av_malloc((key.len() + 1) as u64) as *mut c_char;
            copy(key.as_ptr() as *const c_char, new_key, key.len());
            terminate_cstr(new_key, key.len());

            let ret = avcodec::av_dict_set_int(&mut self.internal, new_key, value, flags);
            if ret >= 0 {
                Ok(ret)
            } else {
                Err(ret)
            }
        }
    }

    pub fn get(&self, key: &str, prev: Option<&AVDictionaryEntry>, flags: i32) -> Option<AVDictionaryEntry> {
        unsafe {
            let new_key: *mut c_char = avcodec::av_malloc((key.len() + 1) as u64) as *mut c_char;
            copy(key.as_ptr() as *const c_char, new_key, key.len());
            terminate_cstr(new_key, key.len());

            let mut prev_opt = null_mut();
            if prev.is_some() {
                prev_opt = prev.unwrap().internal;
            }

            let internal_entry = avcodec::av_dict_get(self.internal, new_key, prev_opt, flags);

            avcodec::av_free(new_key as *mut c_void);
            if internal_entry.is_null() {
                None
            } else {
                Some(AVDictionaryEntry::from(internal_entry))
            }
        }
    }

    // get_string generates string representation of the dictionary
    pub fn get_string(&self, key_val_sep: char, pairs_sep: char) -> Result<String, i32> {
        unsafe {
            let mut buffer: *mut c_char = null_mut();
            let ret = avcodec::av_dict_get_string(self.internal, &mut buffer, key_val_sep as i8, pairs_sep as i8);
            if ret < 0 {
                return Err(ret);
            }
            let output = CStr::from_ptr(buffer).to_str().unwrap();
            let mut out_str = String::new();
            out_str.push_str(output);
            avcodec::av_free(buffer as *mut c_void);
            Ok(out_str)
        }
    }

    pub fn copy(&self, flags: i32) -> Result<Self, i32> {
        let mut internal = null_mut();
        unsafe {
            let ret = avcodec::av_dict_copy(&mut internal, self.internal, flags);
            if ret != 0 {
                return Err(ret);
            }
        }
        return Ok(Self { internal });
    }

    pub unsafe fn get_internal(&self) -> *mut avcodec::AVDictionary {
        return self.internal;
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn dictionary() {
        use super::*;
        let mut dict = AVDictionary::new();
        let key = "preset";
        let value = "p6";
        dict.set(key, value, 0).expect("Something went wrong");
        let parsed = dict.get_string(':', ' ').expect("unable to get string repr");
        println!("{}", dict);
        let mut dict = AVDictionary::new();
        dict.parse_str(&parsed, ":", " ", 0).expect("unable to parse string");
        assert_eq!(dict.get_string(':', ' ').unwrap(), "preset:p6");
    }
}
