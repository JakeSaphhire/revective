pub mod point;
pub mod frame;
pub mod shapes;
pub mod contour;

use std::sync::Mutex;
use std::collections::{VecDeque, HashMap};

// Defines important traits for the graphic submodules
// Defines a method to turn points or point series into an 8bit 
pub trait Drawable{
    fn draw(&self, buffer: &mut Buffer<Vec<u8>, HashMap<u8, Vec<u8>>>, pagination : bool) -> ();
}

pub enum Buffer<T, S> {
    V(T),
    M(S)
}

// Structure Definitions
pub struct Point{
    pub flags: u8,
    posx : u16,
    posy : u16,
}

pub struct Frame<T: Drawable> {
    // TODO: Add Size and Depth info
    // Bi-state flag, false points to drawbuffer, true points to the workbuffer
    flip : bool,
    framebuffer: VecDeque<Vec<T>>
}

// Wrapper over points vector to provide simple image processing routine
struct Shape {
    vertices : Vec<Point>
}


// Simple Flag enum to avoid using binary literals in code
pub enum Flag {
    NoBuffer    = 0x8,
    ClearBuffer = 0x10,
    Line        = 0x40,
    Point       = 0x80,
}

// Puts two 4bit values in a single u8. Format:
// 0000.0000
// ^^^^ ^^^^
// x val y val
pub fn bitconcat(mut x : u8, mut y : u8) -> Result<u8, std::num::IntErrorKind> {
    if x > 15 || y > 15 {
        return Err(std::num::IntErrorKind::InvalidDigit);
    } else {
        y = 0b00001111 & y;
        x = x << 4;
        return Ok(x | y)
    }
}