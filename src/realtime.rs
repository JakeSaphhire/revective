use serialport as sp;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

use crate::graphics::{Frame, Drawable, Pointable};

use opencv::{Result, prelude::*, videoio, imgproc, highgui};
use opencv::core::{BorderTypes, Point, Size, Scalar};

pub fn realtime(file: &'static str, sender: mpsc::Sender<Vec<opencv::types::VectorOfPoint>> ) -> thread::JoinHandle<Result<()>> {
    thread::spawn(move || -> Result<()> {
        let mut video = videoio::VideoCapture::from_file(file, videoio::CAP_ANY)?;
        let mut curframe = Mat::default();
        let frame = highgui::named_window("Image", highgui::WINDOW_FULLSCREEN)?;

        let length = video.get(videoio::CAP_PROP_FRAME_COUNT)?;
        let mut index = 0f64;
        loop {
            video.read(&mut curframe)?;
            index += 1f64;
            let mut newfr = Mat::default();
            // Turns the image binary
            imgproc::cvt_color(&curframe, &mut newfr, imgproc::COLOR_BGR2GRAY, 0)?;
            imgproc::threshold(&newfr, &mut curframe, 128f64, 255f64, imgproc::THRESH_OTSU)?;
            
            let mut contours = opencv::types::VectorOfVectorOfPoint::new();
            let mut cframe = Mat::default();
            imgproc::canny(&curframe, &mut cframe, 0.1, 0.1*2.0, 3, true)?;
            println!("{} of {}", index, length);
            /*
            highgui::imshow("Image", &curframe).expect("Failed to create window");
            let key = highgui::wait_key(1)?;

            if key == 113 {
                return Ok(());
            }*/
            let mut hier = Mat::default();
            imgproc::find_contours_with_hierarchy(&cframe, &mut contours, &mut hier, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_NONE, opencv::core::Point::new(0,0)).expect("Failed to get contours");

            sender.send(contours.to_vec());
        }
    })
}

