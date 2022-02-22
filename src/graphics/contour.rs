// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;
use serialport as Serial;

use crate::graphics::{Drawable, Point, Flag};


impl Drawable for Imageproc::Contour<u16> {
    fn draw(&self, port : &mut dyn Serial::SerialPort) -> std::io::Result<usize> {
        let mut pt : usize = 0;
        for point in self.points.iter() {
            match Point::new(Flag::Point as u8 | Flag::NoBuffer as u8, point.x, point.y).draw(port) {
                Ok(_v) => pt += 1,
                Err(_e) => (),
            };
        }
        Ok(pt)
    }
}