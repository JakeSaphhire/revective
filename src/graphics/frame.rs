#![allow(dead_code)]
use crate::graphics::{Drawable, Point, Frame};
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};
use image::{imageops::*, GenericImageView};
// The frame is a tunnel to send the points vector
// the points buffer is the current working-drawing buffer,
// the buffer vector is the one being drawn points into while the engine runs
// Buffers are rotated every drawing loop. The structure is concurrently accessed

impl<T: Drawable> Frame<T> {
    pub fn new() -> Frame<T> {
        Frame {flip : true, draw_vec : Mutex::new(Vec::new()), work_vec : Mutex::new(Vec::new())}
    }

    pub fn drawbuffer(&self) -> MutexGuard<Vec<T>> {
        if self.flip {
            return self.draw_vec.lock().unwrap();
        } else {
            return self.work_vec.lock().unwrap();
        }
    }

    pub fn workbuffer(&self) -> MutexGuard<Vec<T>> {
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
        let _mut_ = self.work_vec.lock();
        let _mut__ = self.draw_vec.lock();
        self.flip = !self.flip;
    }

    //TODO: Add line drawing methods and point drawing methods!!!
}

impl Frame<Point> {
    pub fn from_image(&mut self) -> Result<(), image::ImageError> {
        let image = image::io::Reader::open("images/image.png")?.with_guessed_format()?.decode()?.grayscale();
        let altered_display = contrast(&image, 0.5f32);
        let display = image::DynamicImage::ImageRgba8(altered_display);
        let mut vec = self.workbuffer();
        for pixel in display.pixels(){
            if pixel.2[0] < 127 {
                vec.push(
                    Point::new( 0b00001000, 
                            ((pixel.0 as f32/display.width() as f32)*4096f32) as u16, 
                            4096u16-((pixel.1 as f32/display.height() as f32)*4096f32) as u16
                        )
                );
            }
        }
        Ok(())
    }
}
