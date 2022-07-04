// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;

use crate::graphics::{Drawable, Point, Flag, Buffer};
use std::collections::HashMap;

impl Drawable for Imageproc::Contour<u16> {
    fn draw(&self, buffer: &mut Buffer<Vec<u8>, HashMap<u8, Vec<u8>>>, pagination : bool) -> () {
        for point in self.points.iter() {
            Point::new(Flag::Point as u8 | Flag::NoBuffer as u8, point.x, point.y).draw(buffer, pagination);
        }
    }
}