#![allow(dead_code)]
use crate::graphics::{Drawable, Point, Shape, Flag, Buffer};
use std::collections::HashMap;

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

    // TODO
    pub fn scale(&mut self, _scale_factor : f32) -> &Self {
        self
    }
}

impl Drawable for Shape {
    fn draw(&self, buffer: &mut Buffer<Vec<u8>, HashMap<u8, Vec<u8>>>, pagination : bool) -> () {
        for point in self.vertices.iter() {
            point.draw(buffer, pagination);
        }
    }
}