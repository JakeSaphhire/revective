// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;

use crate::graphics::{Drawable, Point, Flag, Buffer, Buffers};

impl Drawable for Imageproc::Contour<u16> {
    fn draw(&self, buffer: &mut Vec<u8>, deltamode : bool) -> () {
        if deltamode {
            // Sets up the first point
            let mut points = self.points.iter();
            let anchor = points.next().unwrap();
            Point::new(0xffu8, anchor.x, anchor.y).draw(buffer, deltamode);

            for point in points {
                // Check if we need re-anchoring
                if point.x.abs_diff(anchor.x) > 254 || point.y.abs_diff(anchor.y) > 254 {
                    // Re-anchoring
                    Point::new(0xffu8, anchor.x, anchor.y).draw(buffer, deltamode);
                } else {
                    let dx : u16 = point.x - anchor.x;
                    let dy : u16 = point.y - anchor.y;
                    // See xigrek repo for details
                    buffer.push(dy as u8);
                    buffer.push(dx as u8);
                }
            }

        } else {
            // Puts full 32bit-wide points in a vector, business as usual
            for point in self.points.iter() {
                Point::new(Flag::Point as u8 | Flag::NoBuffer as u8, point.x, point.y).draw(buffer, deltamode);
            }
        }
    }
}