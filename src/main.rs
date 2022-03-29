mod graphics;
mod context;

use crate::graphics::{Frame, Point};
use crate::context::Context;

use image::io::Reader as Reader;
use image::GenericImageView;

// Maximum displayable size, constrained by the DAC's 12bit resolution
const MAX_SIZE: u32 = 4096;
const DRAW_SPEED: u8 = 1;

fn main() {
    let display;
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

    let mut f : Frame<Point> = Frame::new();
    let pt_to_draw = 
        //f.from_image_contoured(display.as_luma8().unwrap()).1;
        f.from_image(&display).1;
        //f.from_gif_contoured().1;
    let ctx : Context<Point> = Context::new(f, DRAW_SPEED as i16);
    println!("{} points to draw, among which {} will actually be drawn", pt_to_draw, (pt_to_draw as i32/ DRAW_SPEED as i32));

    Context::list_ports();
    let _ = ctx.spawn_buf().join().unwrap();
}