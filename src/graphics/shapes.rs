#![allow(dead_code)]
use crate::graphics::{Drawable, Point, Shape, Flag};
use serialport as Serial;

impl Shape {
    fn new() -> Shape {
        Shape {vertices : Vec::new()}
    }

    pub fn new_shape(points : &[(u16, u16)]) -> Shape {
        let mut rect = Shape::new();
        rect.vertices.extend(
            points.iter().map(
                |point| {
                    Point::new(Flag::Line as u8, point.0, point.1)
                }
            )
        );
        rect
    }

    pub fn translate(&mut self, vector : (u16, u16)) -> &Self {
        let _ : Vec<_> = self.vertices.iter_mut().map(
            |vertex| {
                vertex.posx += vector.0;
                vertex.posy += vector.1;
            }
        ).collect();
        self
    }

    pub fn scale(&mut self, _scale_factor : f32) -> &Self {
        self
    }
}

impl Drawable for Shape {
    fn draw(&self, port: &mut dyn Serial::SerialPort) -> std::io::Result<usize>{
        let mut pts_sent = 0;
        for point in self.vertices.iter() {
            match point.draw(port) {
                Ok(v) => pts_sent += v,
                Err(_e) => (),
            }
        }
        Ok(pts_sent)
    }
}