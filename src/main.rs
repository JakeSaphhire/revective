mod graphics;
mod context;
mod realtime;

use crate::graphics::{Frame, Point, Flag};
use crate::context::Context;
use crate::realtime::{hello_cv2, hello_cv};

use image::io::Reader as Reader;
use image::GenericImageView;

use imageproc::contours as ImageContours;

use serialport as serial;
use std::time::Instant;
use std::sync::{Arc, Mutex, mpsc};

use opencv::{Result, prelude::*, videoio, imgproc, highgui, types};

// TODO: Librarify project? Cleanup main
// TODO: Port firmware code to STM32 (High prio!!)

// Maximum displayable size, constrained by the DAC's 12bit resolution
const MAX_SIZE: u32 = 4096;
const DRAW_SPEED: u8 = 1;

fn main() {
    // Setup the communication channel:
    let (trans, recv) = mpsc::channel();
    //test(); return;
    //test_cv();
    let mut display;
    let image = Reader::open("images/image.png")
                    .unwrap()
                    .with_guessed_format()
                    .unwrap()
                    .decode()
                    .unwrap()
                    .grayscale();

    if image.width() > MAX_SIZE || image.height() > MAX_SIZE {
        display = image.resize(MAX_SIZE, MAX_SIZE, image::imageops::FilterType::Triangle);
    } else {
        display = image;
    }
 
    let f : Frame<opencv::types::VectorOfPoint> = Frame::new();

    let vid = videoio::VideoCapture::from_file("/home/jake/storage/webm_img_collection/UN.mp4", videoio::CAP_ANY).unwrap();
    let len = vid.get(videoio::CAP_PROP_FRAME_COUNT).unwrap();
    
    let mut ctx : Context = Context::new(DRAW_SPEED as i16);
    Context::list_ports();
    ctx.sent.0 = len as i32;
    //let pt_to_draw = 
        //contour_helper(display.as_mut_luma8().expect("Impossible"), f.as_mut());
        //f.from_image(&display).1;
        //f.from_gif_contoured().1;
        //println!("{} points to draw, among which {} will actually be drawn", pt_to_draw, (pt_to_draw as i32/ DRAW_SPEED as i32));
    //test();
    let _ = realtime::realtime("/home/jake/storage/webm_img_collection/UN.mp4", trans);
    let _ = ctx.spawn_buf(f, recv, true).join().unwrap();
}

fn contour_helper(display : &mut image::GrayImage, f : &mut Frame<ImageContours::Contour<u16>>) -> usize {
    image::imageops::colorops::invert(display);
    f.from_image(display).1
}

fn test() -> () {
    let ports = serial::available_ports().expect("Failed to list ports");
    let mut port = serial::new(&ports[0].port_name, 2_000_000).open().expect("Failed to open port!");
    let points : Vec<u8> = vec![Flag::NoBuffer as u8; 640_000];
    let size = points.len()/4;
    let mut sent : usize = 0;
    let now = Instant::now();
    {
        match port.write(&points[..]) {
            Ok(v) => sent = v/4,
            Err(_e) => (),
        }
    }
    let t : f64 = now.elapsed().as_micros() as f64;
    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {}pps / {:.3}Mbps", sent, size, (sent as f32 / size as f32) * 100 as f32, t/1000f64, t/(1 as u32 *1000) as f64, 1, (sent as i64 * 1_000_000i64)/t as i64, (sent as f64 *32f64)/(t as f64));
}

fn test_cv() {
    hello_cv2("/home/jake/storage/webm_img_collection/UN.mp4").unwrap();
    panic!("Test terminated!")
}