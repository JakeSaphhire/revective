use serialport as sp;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

use crate::graphics::{Frame, Drawable, Pointable};

use opencv::{Result, prelude::*, videoio, imgproc, highgui};
use opencv::core::{BorderTypes, Point, Size, Scalar};

pub fn hello_cv(file: &str) -> Result<()> {
    // OpenCV setup
    let mut video = videoio::VideoCapture::from_file(file, videoio::CAP_ANY).expect("Failed to get frame");
    let frame = highgui::named_window("Transformed", highgui::WINDOW_FULLSCREEN)?;
    let mut curframe = Mat::default();
    loop {
        video.read(&mut curframe)?;
        let mut newfr = Mat::default();
        // Do something with this frame
        //imgproc::gaussian_blur(&curframe, &mut newfr, Size::new(25, 25), 10f64, 10f64, BorderTypes::BORDER_CONSTANT as i32)?;
        let mut z = Mat::new_size_with_default(curframe.size()?, opencv::core::CV_32F, Scalar::from((0.,0.,0.)))?;
        imgproc::circle(&mut z, opencv::core::Point::new(0,0), 250, Scalar::from((255., 255., 255.)), -1, imgproc::LINE_8, 0)?;
        highgui::imshow("Transformed", &z)?;
        let key = highgui::wait_key(0)?;

        if key == 113 {
            break;
        }
    }

    Ok(())
}

// 2nd OpenCV test function.
// Draws contours onto a frame
pub fn hello_cv2(file: &str) -> Result<()> {
    // OpenCV setup
    let mut video = videoio::VideoCapture::from_file(file, videoio::CAP_ANY).expect("Failed to get frame");
    let mut curframe = Mat::default();
    let frame = highgui::named_window("Image", highgui::WINDOW_FULLSCREEN)?;
    loop {
        // Reads image
        video.read(&mut curframe)?;
        let mut newfr = Mat::default();
        // Turns the image binary
        imgproc::cvt_color(&curframe, &mut newfr, imgproc::COLOR_BGR2GRAY, 0)?;
        imgproc::threshold(&newfr, &mut curframe, 128f64, 255f64, imgproc::THRESH_OTSU)?;
        
        let mut contours = opencv::types::VectorOfVectorOfPoint::new();
        let mut cframe = Mat::default();
        // Send the new vector off
        imgproc::canny(&curframe, &mut cframe, 0.1, 0.1*2.0, 3, true)?;
        
        let mut hier = Mat::default();
        imgproc::find_contours_with_hierarchy(&cframe, &mut contours, &mut hier, imgproc::RETR_TREE, imgproc::CHAIN_APPROX_NONE, opencv::core::Point::new(0,0)).expect("Failed to get contours");
        println!("length of {}", contours.get(1).unwrap().len());
        let mut newnewfr = Mat::new_size_with_default(curframe.size()?, opencv::core::CV_32F, Scalar::from((0.,0.,0.)))?;

        /*
        for contour in &contours {
            for point in contour {
                print!("[{}, {}]", point.x, point.y);
                imgproc::circle(&mut newnewfr, point, 1, Scalar::from((255., 255., 255.)), -1, imgproc::LINE_8, 0)?;
            }
            print!("\n");
        }
        */
        imgproc::draw_contours(&mut newnewfr, &contours, -1, Scalar::from((255., 255., 255.)), 1, imgproc::LINE_8, &hier, i32::MAX, opencv::core::Point::new(0,0))?;
        println!("Size : {}", newnewfr.total());
        highgui::imshow("Image", &newnewfr).expect("Failed to create window");
        let key = highgui::wait_key(0)?;

        if key == 113 {
            break;
        }
    }

    Ok(())
}

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

