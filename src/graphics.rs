pub mod point;
pub mod frame;
pub mod shapes;
pub mod contour;

use std::sync::Mutex;
use std::collections::VecDeque;

// Defines important traits for the graphic submodules
// Defines a method to turn points or point series into an 8bit 
pub trait Drawable{
    fn draw (&self, pvec : &mut Vec<u8>) -> ();
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
