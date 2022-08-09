// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;

use crate::graphics::{Drawable, Point, Flag};

impl Drawable for Imageproc::Contour<u16> {
    fn draw(&self, buffer: &mut Vec<u8>, deltamode : bool) -> usize {
        let mut len : usize = 0;
        let s : usize = buffer.len();
        
        if deltamode {
            // Sets up the first point
            let mut points = self.points.iter();
            len = points.len();
            let anchor = points.next().unwrap();
            Point::new(0x7fu8, anchor.x, anchor.y).draw(buffer, deltamode);
            let mut prev = anchor;
            for point in points {
                // Check if we need re-anchoring
                if point.x.abs_diff(prev.x) >= 127 || point.y.abs_diff(prev.y) >= 127 {
                    // Re-anchoring
                    print!("REANC-");
                    Point::new(0x7fu8, point.x, point.y).draw(buffer, deltamode);
                } else {
                    let dx : i8 = (point.x as i32 - prev.x as i32) as i8;
                    let dy : i8 = (point.y as i32 - prev.y as i32) as i8;
                    // See xigrek repo for details
                    buffer.push(dy as u8);
                    buffer.push(dx as u8);
                }
                prev = point;
            }
            
            for i in 0..( ( buffer.len() - s) % 4) {
                buffer.push(0);
            }
            let size_diff = buffer.len() - s;
            println!("Size of buffer to print : {} bytes, divisible by 4?: {}, Compression of {}/{} ({}%)", size_diff, (size_diff % 4) == 0, size_diff, len*4, size_diff as f32/(len as f32 * 4f32) * 100f32);
        } else {
            // Puts full 32bit-wide points in a vector, business as usual
            for point in self.points.iter() {
                len += Point::new(Flag::Point as u8 | Flag::NoBuffer as u8, point.x, point.y).draw(buffer, deltamode);
            }
        }
        len
    }
}