use std::ffi::CString;
use std::os::raw;

use crate::cameralot_capture_raw::*;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

pub struct CameraFeed {
    internal: *mut raw::c_void,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CameraFeedError {
    NotOpen,
    ReadFailed,
    RetrieveFailed,
    EmptyFrame,
    EncodingFailed,
    NoFrame
}

impl CameraFeed {
    pub fn new() -> Self {
        Self {
            internal: unsafe { camera_feed_create() }
        }
    }

    pub fn open(&mut self, index: i32) -> bool {
        unsafe {
            camera_feed_open(self.internal, index)
        }
    }

    pub fn open_api_pref(&mut self, index: i32, api: i32) -> bool {
        unsafe {
            camera_feed_open_api_pref(self.internal, index, api)
        }
    }

    pub fn is_opened(&self) -> bool {
        unsafe {
            camera_feed_is_opened(self.internal)
        }
    }

    pub fn read(&mut self, width: u32, height: u32, ext: &str) -> Result<(), CameraFeedError> {
        let c_str = CString::new(ext).unwrap();

        let status = unsafe {
            camera_feed_read(self.internal, width, height, c_str.as_ptr())
        };

        match status {
            ReadStatus::Success => Ok(()),
            ReadStatus::NotOpen => Err(CameraFeedError::NotOpen),
            ReadStatus::ReadFailed => Err(CameraFeedError::ReadFailed),
            ReadStatus::RetrieveFailed => Err(CameraFeedError::RetrieveFailed),
            ReadStatus::EmptyFrame => Err(CameraFeedError::EmptyFrame),
            ReadStatus::EncodingFailed => Err(CameraFeedError::EncodingFailed),
        }
    }
    
    pub fn get_buf(&self) -> Result<&[u8], CameraFeedError> {
        let mut share = ByteBufferShare { buffer: null_mut(), length: 0 };

        unsafe {
            let status = camera_feed_get_buf(self.internal, (&mut share) as *mut ByteBufferShare);
            
            if status {
                let s = from_raw_parts(share.buffer, share.length);

                Ok(s)
            } else {
                Err(CameraFeedError::NoFrame)
            }
        }
    }
}

impl Drop for CameraFeed {
    fn drop(&mut self) {
        unsafe {
            camera_feed_delete(self.internal);
        }
    }
}