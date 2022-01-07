use crate::graphics::point;
use image::{imageops::*, GenericImageView};
// The frame is a tunnel to send the points vector
// the points buffer is the current working-drawing buffer,
// the buffer vector is the one being drawn points into while the engine runs
pub struct Frame<'a> {
    pub drawbuffer : &'a Vec<Option<point::Point>>,
    pub workbuffer : &'a Vec<Option<point::Point>>,
    points_vec: Vec<Option<point::Point>>,
    buffer_vec: Vec<Option<point::Point>>,
    debug: String
}

impl Frame<'_> {
    pub fn new<'a>() -> Frame<'a> {
        let f : Frame;
        f.points_vec = Vec::new();
        f.buffer_vec = Vec::new();
        f.drawbuffer = &f.points_vec;
        f.workbuffer = &f.buffer_vec;

        f.debug = "".to_string();
        f
    }

    pub fn from_image(&self, filename : &String) -> Result<(), image::ImageError> {
        let npoints : i32 = 0;
        let image = image::io::Reader::open("images/image.png")?.with_guessed_format()?.decode()?.grayscale();
        let altered_display = contrast(&image, 0.5f32);
        let display = image::DynamicImage::ImageRgba8(altered_display); 
        self.buffer_vec.extend(display.pixels().map(
            |pixel| {
                if pixel.2[0] < 127 {
                    npoints += 1;
                    Some(point::Point::new(0b00001000, ((pixel.0 as f32/display.width() as f32)*4096f32) as u16, 2048u16-((pixel.1 as f32/display.height() as f32)*2048f32) as u16))
                } else {
                    None
                }
            }
        ));
        Ok(())
    }

    pub fn clear(&self){
        self.drawbuffer.clear();
    }

    pub fn clear_work(&self){
        self.workbuffer.clear();
    }

    pub fn swap(&self){
        let temp = self.drawbuffer;
        self.drawbuffer = temp;
        self.workbuffer = temp;
    }

    //TODO: Add line drawing methods and point drawing methods!!!
}