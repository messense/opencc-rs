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
//!     println!("{}", cc.convert("乾坤一擲").unwrap());
//!     println!("{}", cc.convert("開放中文轉換").unwrap());
//! }
//! ```
extern crate libc;
use libc::{size_t, c_void, c_char};

use std::cell::Cell;
use std::ffi::CStr;
use std::str;

#[link(name = "opencc")]
extern {
    fn opencc_open(config: *const c_char) -> *mut c_void;
    fn opencc_close(opencc: *mut c_void);
    fn opencc_convert_utf8(opencc: *mut c_void, text: *const c_char, length: size_t) -> *mut c_char;
    fn opencc_convert_utf8_free(text: *mut c_char);
    fn opencc_error() -> *const c_char;
}

pub struct OpenCC {
    /// Configuration file
    pub config: String,
    _is_closed: Cell<bool>,
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
                _is_closed: Cell::new(false),
                libopencc: opencc_open(config_ptr as *const c_char),
            }
        }
    }

    /// Convert a text
    ///
    /// # Examples
    /// ```
    /// let cc = opencc::OpenCC::new("t2s.json");
    /// cc.convert("乾坤一擲").unwrap();
    /// cc.convert("開放中文轉換").unwrap();
    /// ```
    pub fn convert(&self, text: &str) -> Option<String> {
        if self._is_closed.get() {
            None
        } else {
            unsafe {
                let text_ptr = text.as_ptr();
                let c_ptr = opencc_convert_utf8(self.libopencc, text_ptr as *const c_char, text.len() as size_t);
                if c_ptr.is_null() {
                    None
                } else {
                    let c_str = CStr::from_ptr(c_ptr);
                    let str_buf: String = str::from_utf8(c_str.to_bytes()).unwrap().to_owned();
                    opencc_convert_utf8_free(c_ptr);
                    Some(str_buf)
                }
            }
        }
    }

    /// Close the underlying libopencc.
    /// Will be called automatically when the variable gets out of scope.
    pub fn close(&self) {
        if !self._is_closed.get() {
            unsafe {
                opencc_close(self.libopencc);
            }
            self._is_closed.set(true);
        }
    }

    /// Is the underlying libopencc closed
    pub fn is_closed(&self) -> bool {
        self._is_closed.get()
    }

    /// Returns the last error message
    ///
    /// # Safety
    ///
    /// Note that this function is the only one which is NOT thread-safe.
    pub fn last_error() -> Option<String> {
        unsafe {
            let error_ptr = opencc_error();
            if error_ptr.is_null() {
                None
            } else {
                let c_str = CStr::from_ptr(error_ptr);
                let str_buf: String = str::from_utf8(c_str.to_bytes()).unwrap().to_owned();
                Some(str_buf)
            }
        }
    }
}

impl Drop for OpenCC {
    /// Close the underlying libopencc when it been droped
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::OpenCC;

    #[test]
    fn test_simple_convert() {
        let cc = OpenCC::new("t2s.json");
        assert_eq!("乾坤一掷", &cc.convert("乾坤一擲").unwrap());
        assert_eq!("开放中文转换", &cc.convert("開放中文轉換").unwrap());
    }

    #[test]
    fn test_opencc_close() {
        let cc = OpenCC::new("t2s.json");
        assert_eq!("乾坤一掷", &cc.convert("乾坤一擲").unwrap());
        assert_eq!(false, cc.is_closed());
        cc.close();
        assert_eq!(true, cc.convert("開放中文轉換").is_none());
        assert_eq!(true, cc.is_closed());
    }

    #[test]
    fn test_opencc_last_error() {
        println!("{}", OpenCC::last_error().unwrap());
    }
}
