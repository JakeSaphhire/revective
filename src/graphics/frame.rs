use crate::graphics::point;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};
use image::{imageops::*, GenericImageView};
// The frame is a tunnel to send the points vector
// the points buffer is the current working-drawing buffer,
// the buffer vector is the one being drawn points into while the engine runs
// Buffers are rotated every drawing loop. The structure is concurrently accessed
pub struct Frame {
    // Bi-state flag, false points to drawbuffer, true points to the workbuffer
    flip : bool,
    draw_vec: Mutex<Vec<Option<point::Point>>>,
    work_vec: Mutex<Vec<Option<point::Point>>>,
    debug: String
}

impl Frame {
    pub fn new<'a>() -> Frame {
        let mut f = Frame {flip : false, draw_vec : Mutex::new(Vec::new()), work_vec : Mutex::new(Vec::new()), debug : "".to_string()};
        f
    }

    pub fn from_image(&mut self, filename : &String) -> Result<(), image::ImageError> {
        let mut npoints : i32 = 0;
        let image = image::io::Reader::open("images/image.png")?.with_guessed_format()?.decode()?.grayscale();
        let altered_display = contrast(&image, 0.5f32);
        let display = image::DynamicImage::ImageRgba8(altered_display); 
        self.workbuffer().deref_mut().extend(display.pixels().map(
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

    pub fn drawbuffer(&self) -> MutexGuard<Vec<Option<point::Point>>> {
        if self.flip {
            return self.draw_vec.lock().unwrap();
        } else {
            return self.work_vec.lock().unwrap();
        }
    }

    pub fn workbuffer(&self) -> MutexGuard<Vec<Option<point::Point>>> {
        if self.flip {
            return self.work_vec.lock().unwrap();
        } else {
            return self.draw_vec.lock().unwrap();
        }
    }

    pub fn clear(&mut self){
        self.drawbuffer().deref_mut().clear();
    }

    pub fn clear_work(&mut self){
        self.workbuffer().deref_mut().clear();
    }

    pub fn swap(&mut self){
        self.drawbuffer(); self.workbuffer();
        self.flip = !self.flip;
    }

    //TODO: Add line drawing methods and point drawing methods!!!
}