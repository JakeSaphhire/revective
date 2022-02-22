#![allow(dead_code)]
use crate::graphics::{Drawable, Point, Frame, Flag};
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};

use image::GenericImageView;
use imageproc::contours as ImageContours;
use imageproc::point as ImagePoint;
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
    pub fn from_image(&mut self, display : &image::DynamicImage) -> (&Self, usize) {
        let mut vec = self.workbuffer();
        for pixel in display.pixels(){
            if pixel.2[0] < 127 {
                vec.push(
                    Point::new( Flag::Point as u8 | Flag::NoBuffer as u8, 
                                pixel.0 as u16, 
                                display.height() as u16 - pixel.1 as u16
                        )
                );
            }
        }
        let size = vec.len();
        //println!("{} Points to draw", size);
        (self, size)
    }

    pub fn from_image_contoured(&mut self, display : &image::GrayImage) -> (&Self, usize) {
        use std::mem as mem;
        let mut vec = self.workbuffer();
        vec.extend(
                ImageContours::find_contours::<u16>(display)
                .iter_mut().map::<Vec<ImagePoint::Point<u16>>, _>( move |contour| mem::take(contour.points.as_mut())).flatten()
                .map(|point : ImagePoint::Point<u16>| Point::new(Flag::NoBuffer as u8 | Flag::Point as u8, point.x, point.y)));
        (self, vec.len())
    }
}

impl Frame<ImageContours::Contour<u16>> {
    pub fn from_image(&mut self, display : &image::GrayImage) -> (&Self, usize) {
        let mut vec = self.workbuffer();
        vec.extend(ImageContours::find_contours::<u16>(display));
        (self, vec.len())
    }
}
