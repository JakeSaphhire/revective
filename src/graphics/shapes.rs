use crate::graphics::{Drawable, Point};
use serialport as Serial;

// Wrapper over points vector to provide simple image processing routine
struct Shape {
    vertices : Vec<Point>
}

impl Shape {
    fn new() -> Shape {
        Shape {vertices : Vec::new()}
    }
}

impl Drawable for Shape {
    fn draw(&self, port: &mut dyn Serial::SerialPort) -> std::io::Result<usize>{
        // TODO : Shape drawing routine
        Ok(1)
    }
}