#![allow(dead_code)]
#![allow(unused_imports)]
use crate::graphics::{Drawable, Point, Frame, Flag};
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};
use std::collections::VecDeque;

use image::GenericImageView;
use image::DynamicImage;
use imageproc::contours as ImageContours;
use imageproc::point as ImagePoint;
// The frame is a tunnel to send the points vector
// the points buffer is the current working-drawing buffer,
// the buffer vector is the one being drawn points into while the engine runs
// Buffers are rotated every drawing loop. The structure is concurrently accessed



impl<T: Drawable> Frame<T> {
    pub fn new() -> Frame<T> {
        Frame {flip : true, framebuffer : VecDeque::new()}
    }
    
    pub fn current_frame(&self) -> &Vec<T> {
        self.framebuffer.front().expect("Expected ref")
    }

    pub fn new_frame(&mut self) -> &mut Vec<T> 
    {
        self.framebuffer.push_back(Vec::new());
        self.framebuffer.back_mut().expect("Expected ref")
    }

    pub fn pop_drawn(&mut self) -> () {
        self.framebuffer.pop_front();
    }
} 


impl<T : Drawable> AsMut<Frame<T>> for Frame<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Frame<T>
    {
        self
    }
}

impl Frame<Point> {
    pub fn from_image(&mut self, display : &image::DynamicImage) -> (&Self, usize) {
        let vec = self.new_frame();
        for pixel in display.pixels(){
            if pixel.2[0] < 127 {
                vec.push(
                    Point::new( Flag::Point as u8 | Flag::NoBuffer as u8, 
                                ((pixel.0 as f32/display.width() as f32)*4096f32) as u16, 
                                4096u16 - ((pixel.1 as f32/display.height() as f32)*4096f32) as u16
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
        let vec = self.new_frame();
        vec.extend(
                ImageContours::find_contours::<u16>(display)
                .iter_mut().map::<Vec<ImagePoint::Point<u16>>, _>( move |contour| mem::take(contour.points.as_mut())).flatten()
                .map(|point : ImagePoint::Point<u16>| Point::new(Flag::NoBuffer as u8 | Flag::Point as u8, 
                    ((point.x as f32/display.width() as f32)*4096f32) as u16, 
                    4096u16 - ((point.y as f32/display.height() as f32)*4096f32) as u16
                )));
        let retsize = vec.len();
        (self, retsize)
    }

    pub fn from_gif_contoured(&mut self) -> (&Self, usize) {
        use std::mem as mem;
        use std::fs::File;
        use image::{ImageDecoder, AnimationDecoder};
        use image::codecs::gif::GifDecoder;

        let vec = self.new_frame();
        // Opening the gif - from image.rs documentation
        let file = File::open("images/gif.gif").unwrap();
        let decode = GifDecoder::new(file).unwrap();
        let height = decode.dimensions().1;
        let width = decode.dimensions().0;
        let frames = decode.into_frames();
        // Get height


        for frame in frames {
            match frame {
                Ok(f) => vec.extend(
                    ImageContours::find_contours::<u16>(&image::DynamicImage::ImageRgba8(f.into_buffer()).into_luma8())
                    .iter_mut().map::<Vec<ImagePoint::Point<u16>>, _>( move |contour| mem::take(contour.points.as_mut())).flatten()
                    .map(|point : ImagePoint::Point<u16>| Point::new(Flag::NoBuffer as u8 | Flag::Point as u8, 
                        ((point.x as f32/width as f32)*4096f32) as u16, 
                        4096u16 - ((point.y as f32/height as f32)*4096f32) as u16
                    ))),
                Err(e) => println!("Skipped frame: {}", e)
            }
        }
        let retsize = vec.len();
        (self, retsize)
    }

    
}

impl Frame<ImageContours::Contour<u16>> {
    pub fn from_image(&mut self, display : &image::GrayImage) -> (&Self, usize) {
        let vec = self.new_frame();
        vec.extend(ImageContours::find_contours::<u16>(display));
        let retsize = vec.len();
        (self, retsize)
    }
}
