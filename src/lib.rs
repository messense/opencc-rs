//! OpenCC binding for Rust
//!
//! You should install [OpenCC 1.0.x](https://github.com/BYVoid/OpenCC) library first.
//!
//! Supported configurations:
//!
//! * `s2t.json` Simplified Chinese to Traditional Chinese
//! * `t2s.json` Traditional Chinese to Simplified Chinese
//! * `s2tw.json` Simplified Chinese to Traditional Chinese (Taiwan Standard)
//! * `tw2s.json` Traditional Chinese (Taiwan Standard) to Simplified Chinese
//! * `s2hk.json` Simplified Chinese to Traditional Chinese (Hong Kong Standard)
//! * `hk2s.json` Traditional Chinese (Hong Kong Standard) to Simplified Chinese
//! * `s2twp.json` Simplified Chinese to Traditional Chinese (Taiwan Standard) with Taiwanese idiom
//! * `tw2sp.json` Traditional Chinese (Taiwan Standard) to Simplified Chinese with Mainland Chinese idiom
//! * `t2tw.json` Traditional Chinese (OpenCC Standard) to Taiwan Standard
//! * `t2hk.json` Traditional Chinese (OpenCC Standard) to Hong Kong Standard
//!
//! # Examples
//! ```
//! extern crate opencc;
//!
//! use opencc::OpenCC;
//!
//! fn main() {
//!     let cc = OpenCC::new("t2s.json");
//!     println!("{}", cc.convert("乾坤一擲"));
//!     println!("{}", cc.convert("開放中文轉換"));
//! }
//! ```
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
    libopencc: *mut c_void,
}

impl OpenCC {
    /// Constructs a new `OpenCC`
    ///
    /// # Examples
    /// ```
    /// let cc = opencc::OpenCC::new("t2s.json");
    /// ```
    pub fn new(config: &str) -> OpenCC {
        let _config = config.to_string();
        let config_ptr = _config.as_ptr();
        unsafe {
            OpenCC {
                config: _config,
                libopencc: opencc_open(config_ptr as *const c_char),
            }
        }
    }

    /// Convert a text
    ///
    /// # Examples
    /// ```
    /// let cc = opencc::OpenCC::new("t2s.json");
    /// cc.convert("乾坤一擲");
    /// cc.convert("開放中文轉換");
    /// ```
    pub fn convert(&self, text: &str) -> String {
        unsafe {
            let text_ptr = text.as_ptr();
            let c_ptr = opencc_convert_utf8(self.libopencc, text_ptr as *const c_char, text.len() as size_t);
            let c_str = CStr::from_ptr(c_ptr);
            let str_buf: String = str::from_utf8(c_str.to_bytes()).unwrap().to_owned();
            opencc_convert_utf8_free(c_ptr);
            str_buf
        }
    }

    pub fn close(&self) {
        unsafe {
            opencc_close(self.libopencc);
        }
    }
}

impl Drop for OpenCC {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
fn test_convert() {
    let opencc = OpenCC::new("t2s.json");
    assert_eq!("乾坤一掷".to_string(), opencc.convert("乾坤一擲"));
    assert_eq!("开放中文转换".to_string(), opencc.convert("開放中文轉換"));
}
