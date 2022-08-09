use serialport as sp;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::graphics::{Frame, Drawable};

use opencv::{Result, prelude::*, videoio, cudaimgproc}

fn hello_cv(file: &str) -> Result<()> {

    let mut video = VideoCapture::from_file(file, CAP_ANY);
    let mut curframe = Mat::default();
    loop {
        video.read(&mut curframe)?;
        // Do something with this frame
    }
    OK(())
}