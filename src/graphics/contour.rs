// Extending the contour class from imageproc for interoperability with frame.rs
use imageproc::contours as Imageproc;

use crate::graphics::{Drawable, Point, Flag, Pointable};

// Little workaround
impl Drawable for Imageproc::Contour<u16> {
    #[inline]
    fn draw(&self, buffer: &mut Vec<u8>, deltamode: bool) -> usize {
        self.points.draw(buffer, deltamode)
    }
}

impl Drawable for opencv::types::VectorOfPoint {
    #[inline]
    fn draw(&self, buffer: &mut Vec<u8>, deltamode: bool) -> usize {
        self.to_vec().draw(buffer, deltamode)
    }
}

// Generic vector of point drawing method; Allows interoperability with crate-local point struct, opencv points and imageproc points
// Pointable is u16 because we will only draw on 4000x4000
impl<T : Pointable<u16>> Drawable for Vec<T> {
    fn draw(&self, buffer: &mut Vec<u8>, deltamode : bool) -> usize {
        let mut len : usize = 0;
        let s : usize = buffer.len();
        

        if deltamode {
            // Sets up the first point
            // Where x is 0 and y is 1
            let mut points = self.iter();
            len = points.len();
            let anchor = points.next().unwrap().point();
            Point::new(0x7fu8, anchor.0, anchor.1).draw(buffer, deltamode);
            let mut prev = anchor;
            for pointable in points {
                let point = pointable.point();
                // Check if we need re-anchoring
                if point.0.abs_diff(prev.0) >= 127 || point.1.abs_diff(prev.1) >= 127 {
                    // Re-anchoring
                    print!("REANC-");
                    Point::new(0x7fu8, point.0, point.1).draw(buffer, deltamode);
                } else {
                    let dx : i8 = (point.0 as i32 - prev.0 as i32) as i8;
                    let dy : i8 = (point.1 as i32 - prev.1 as i32) as i8;
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
            for point in self.iter() {
                len += Point::new_pair(Flag::Point as u8 | Flag::NoBuffer as u8, point.point()).draw(buffer, deltamode);
            }
        }
        len
    }
}