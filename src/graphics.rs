pub mod point;
pub mod frame;
pub mod shapes;

use serialport;
use point::Point;

//Defines important traits for the graphic submodules

pub trait Draw {
    fn draw (&self, port: &mut dyn serialport::SerialPort) -> std::io::Result<usize>;
}

// Trait to distinguish points and shapes
pub trait IsPoint {
    fn new(flags : u8, x : u16, y : u16) -> Point;
}