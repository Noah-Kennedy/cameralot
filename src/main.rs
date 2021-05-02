use std::fs;
use std::process::Command;

use crate::cameralot_capture::CameraFeed;
use crate::cameralot_capture_raw::TimerData;

pub mod cameralot_capture_raw;
pub mod cameralot_capture;

const FILE_FORMAT: &str = ".jpg";
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let _ = Command::new("rm")
        .arg("-f")
        .arg("images/*").spawn().unwrap().wait().unwrap();

    let mut cap = CameraFeed::new();

    cap.open(0);

    let mut td = TimerData {
        grab_millis: 0,
        retrieve_millis: 0,
        resize_millis: 0,
        encode_millis: 0
    };

    for i in 0..100 {
        let path = format!("images/{}{}", i, FILE_FORMAT);

        let timer = std::time::Instant::now();
        cap.read(WIDTH, HEIGHT, FILE_FORMAT, &mut td).unwrap();
        let time = timer.elapsed().as_millis();

        println!("{}:\t{} millis", i, time);
        println!("\tGrab: {}", td.grab_millis);
        println!("\tRetrieve: {}", td.retrieve_millis);
        println!("\tResize: {}", td.resize_millis);
        println!("\tEncode: {}", td.encode_millis);

        let s = cap.get_buf().unwrap();

        fs::write(&path, s).unwrap();
    }
}
