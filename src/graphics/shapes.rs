#![allow(dead_code)]
use crate::graphics::{Drawable, Point, Shape, Flag};

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
    fn draw(&self, pvec : &mut Vec<u8>) -> () {
        for point in self.vertices.iter() {
            point.draw(pvec);
        }
    }
}