use std::fs;
use std::process::Command;

use crate::cameralot_capture::CameraFeed;

pub mod cameralot_capture_raw;
pub mod cameralot_capture;

fn main() {
    let _ = Command::new("rm")
        .arg("-f")
        .arg("images/*").spawn().unwrap().wait().unwrap();

    let mut cap = CameraFeed::new();

    cap.open(0);

    for i in 0..100 {
        let path = format!("images/{}.png", i);

        let timer = std::time::Instant::now();
        cap.read(1280, 720, ".png").unwrap();
        let time = timer.elapsed().as_millis();

        println!("{}:\t{} millis", i, time);

        let s = cap.get_buf().unwrap();

        fs::write(&path, s).unwrap();
    }
}
