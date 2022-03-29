// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;

use crate::graphics::{Drawable, Point, Flag};


impl Drawable for Imageproc::Contour<u16> {
    fn draw(&self, pvec : &mut Vec<u8>) -> () {
        for point in self.points.iter() {
            Point::new(Flag::Point as u8 | Flag::NoBuffer as u8, point.x, point.y).draw(pvec);
        }
    }
}