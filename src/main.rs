use std::fs;
use std::os::raw;
use std::process::Command;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

use crate::cameralot_capture::*;

pub mod cameralot_capture;

fn main() {
    let _ = Command::new("rm")
        .arg("-f")
        .arg("images/*").spawn().unwrap().wait().unwrap();

    unsafe {
        let cap = cameralot_capture::camera_feed_create();

        camera_feed_open(cap, 0);

        let mut buf = ByteBufferShare {
            buffer: null_mut(),
            length: 0,
        };

        for i in 0..100 {
            let path = format!("images/{}.png", i);

            let status = camera_feed_read(
                cap,
                1280, 720,
                ".png\0".as_ptr() as *const raw::c_char,
            );

            println!("{}: {:?}", i, status);

            if status == ReadStatus::Success {
                let _ = camera_feed_get_buf(cap, (&mut buf) as *mut ByteBufferShare);
                let s = from_raw_parts(buf.buffer, buf.length);
                fs::write(&path, s).unwrap();
            }
        }

        camera_feed_delete(cap);
    }
}
