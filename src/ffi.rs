//! Safe bindings to FFI.

use std::ffi::{c_void, CString};

extern "C" {
    fn ope_comments_create() -> *mut c_void;
    fn ope_comments_add(
        comments: *mut c_void,
		tag: *const c_void, // TEXT
		val: *const c_void, // TEXT
	) -> i32;
    fn ope_encoder_create_pull(
        comments: *mut c_void,
		rate: i32,
		channels: i32,
		family: i32,
		error: *mut i32,
	) -> *mut c_void;
    fn ope_comments_destroy(comments: *mut c_void) -> ();
    fn ope_encoder_drain(enc: *mut c_void) -> i32;
    fn ope_encoder_destroy(enc: *mut c_void) -> ();
    fn ope_encoder_write(
        enc: *mut c_void,
		pcm: *const i16,
		samples_per_channel: i32
	) -> i32;
    fn ope_encoder_get_page(
     	enc: *mut c_void,
		page: *mut *mut u8,
		len: *mut i32,
		flush: i32,
	) -> i32;
}

pub struct Comments(*mut c_void);

impl Comments {
    pub fn new() -> Self {
        Self(unsafe{ope_comments_create()})
    }

    pub fn add(&self, tag: &str, val: &str) -> i32 {
        let tag = CString::new(tag).expect("CString::new failed");
        let val = CString::new(val).expect("CString::new failed");

        unsafe {
            ope_comments_add(self.0, tag.as_ptr() as *const _, val.as_ptr() as *const _)
        }
    }
}

impl Drop for Comments {
    fn drop(&mut self) {
        unsafe {
            ope_comments_destroy(self.0)
        }
    }
}

pub struct Encoder(*mut c_void);

impl Encoder {
    pub fn new(comments: &Comments, rate: i32, channels: i32, family: i32)
        -> Result<Self, i32>
    {
        let mut error = std::mem::MaybeUninit::uninit();
        let enc = unsafe {
            ope_encoder_create_pull(
                comments.0,
                rate,
                channels,
                family,
                error.as_mut_ptr(),
            )
        };

        if enc.is_null() {
            unsafe {
                Err(error.assume_init())
            }
        } else {
            Ok(Self(enc))
        }
    }

    pub fn add(&self, samples: &[(i16, i16)], channels: i32) {
        if samples.is_empty() { return }
        unsafe {
            ope_encoder_write(self.0, &samples[0].0, samples.len() as i32 / channels);
        }
    }

    pub fn get(&self) -> Option<&[u8]> {
        let mut page_data = std::mem::MaybeUninit::uninit();
        let mut page_size = std::mem::MaybeUninit::uninit();

        unsafe {
            let page_available = ope_encoder_get_page(self.0, page_data.as_mut_ptr(), page_size.as_mut_ptr(), 0);
            if page_available != 0 {
                Some(std::slice::from_raw_parts(page_data.assume_init(), page_size.assume_init() as usize))
            } else {
                None
            }
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            ope_encoder_drain(self.0);
            ope_encoder_destroy(self.0);
        }
    }
}
