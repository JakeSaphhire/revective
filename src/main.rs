mod graphics;
mod context;

use crate::graphics::{Frame, Point, Flag};
use crate::context::Context;

use image::io::Reader as Reader;
use image::GenericImageView;

use imageproc::contours as ImageContours;

use serialport as serial;
use std::time::Instant;
use std::sync::{Arc, Mutex};

// TODO: Implement paginated drawing method: DONE
// - Add pagination flag to point flags
// - Modify point.rs drawing method to account for pagination
// - Implement option in context.rs

// TODO: OpenCV bindings and compatibility

// TODO: (Try) Owning reference system for frame/framebuffer instead of mutex
// TODO: Librarify project? Cleanup main; try games
// TODO: Port firmware code to STM32 (High prio!!)

// Maximum displayable size, constrained by the DAC's 12bit resolution
const MAX_SIZE: u32 = 4096;
const DRAW_SPEED: u8 = 1;

fn main() {
    //test(); return;

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
    /*
    let mut saved = image::GrayImage::new(display.width(), display.height());
    image::imageops::colorops::invert(display.as_mut_luma8().expect("Impossible"));
    for point in ImageContours::find_contours_with_threshold::<u32>(display.as_luma8().unwrap(), 128).iter_mut().map::<Vec<imageproc::point::Point::<u32>>, _>(|contour| std::mem::take(contour.points.as_mut())).flatten()  {
        saved.put_pixel(point.x, point.y, image::Luma([255]));
    }
    saved.save("images/images-cont.png");
    return;
    */
 
    let mut f : Frame<ImageContours::Contour<u16>> = Frame::new();
    
    let ctx : Context = Context::new(DRAW_SPEED as i16);
    Context::list_ports();
    let pt_to_draw = 
        contour_helper(display.as_mut_luma8().expect("Impossible"), f.as_mut());
        //f.from_image(&display).1;
        //f.from_gif_contoured().1;
        println!("{} points to draw, among which {} will actually be drawn", pt_to_draw, (pt_to_draw as i32/ DRAW_SPEED as i32));
    test();
    let _ = ctx.spawn_buf(Arc::new(Mutex::new(f)), false).join().unwrap();
}

fn contour_helper(display : &mut image::GrayImage, f : &mut Frame<ImageContours::Contour<u16>>) -> usize {
    image::imageops::colorops::invert(display);
    f.from_image(display).1
}

fn test() -> () {
    let ports = serial::available_ports().expect("Failed to list ports");
    let mut port = serial::new(&ports[0].port_name, 2_000_000).open().expect("Failed to open port!");
    let points : Vec<u8> = vec![Flag::NoBuffer as u8; 300_000];
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
    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {}pps", sent, size, (sent as f32 / size as f32) * 100 as f32, t/1000f64, t/(1 as u32 *1000) as f64, 1, (sent as i64 * 1_000_000i64)/t as i64);
}