extern crate libc;
use libc::{size_t, c_void, c_char};

use std::ffi::CStr;
use std::str;

#[link(name = "opencc")]
extern {
    fn opencc_open(config: *const c_char) -> *mut c_void;
    fn opencc_close(opencc: *mut c_void);
    fn opencc_convert_utf8(opencc: *mut c_void, text: *const c_char, length: size_t) -> *mut c_char;
    fn opencc_convert_utf8_free(text: *mut c_char);
}

pub struct OpenCC {
    pub config: String,
}

impl OpenCC {
    pub fn new(config: &str) -> OpenCC {
        OpenCC {
            config: config.to_string(),
        }
    }

    pub fn convert(&self, text: &str) -> String {
        unsafe {
            let config_ptr = self.config.as_ptr();
            let opencc = opencc_open(config_ptr as *const c_char);
            let text_ptr = text.as_ptr();
            let c_ptr = opencc_convert_utf8(opencc, text_ptr as *const c_char, text.len() as size_t);
            let c_str = CStr::from_ptr(c_ptr);
            let str_buf: String = str::from_utf8(c_str.to_bytes()).unwrap().to_owned();
            opencc_convert_utf8_free(c_ptr);
            opencc_close(opencc);
            str_buf
        }
    }
}

#[test]
fn test_convert() {
    let opencc = OpenCC::new("t2s.json");
    let result = opencc.convert("乾坤一擲");
    assert_eq!("乾坤一掷".to_string(), result);
}
