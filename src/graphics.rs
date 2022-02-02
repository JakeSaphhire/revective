pub mod point;
pub mod frame;
pub mod shapes;

use serialport;
use std::sync::{Mutex, MutexGuard};
//Defines important traits for the graphic submodules

pub trait Drawable{
    fn draw (&self, port: &mut dyn serialport::SerialPort) -> std::io::Result<usize>;
}

// Trait to distinguish points and shapes
pub trait IsPoint {
    fn new(flags : u8, x : u16, y : u16) -> Point;
}

// Structures!!
pub struct Point{
    pub flags: u8,
    posx : u16,
    posy : u16,
}

pub struct Frame<T: Drawable> {
    // TODO: Add Size and Depth info
    // Bi-state flag, false points to drawbuffer, true points to the workbuffer
    flip : bool,
    draw_vec: Mutex<Vec<T>>,
    work_vec: Mutex<Vec<T>>,
}

// Wrapper over points vector to provide simple image processing routine
struct Shape {
    vertices : Vec<Point>
}

pub enum Flag {
    NoBuffer    = 0x8,
    ClearBuffer = 0x10,
    Line        = 0x40,
    Point       = 0x80,
}