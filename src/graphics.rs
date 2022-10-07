 pub mod point;
pub mod frame;
pub mod shapes;
pub mod contour;

use std::collections::VecDeque;

// Defines important traits for the graphic submodules
// Defines a method to turn points or point series into an 8bit 
pub trait Drawable{
    fn draw(&self, buffer: &mut Vec<u8>, pagination : bool) -> usize;
}

pub trait Pointable<T> {
    fn point(&self) -> (T, T);
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

impl Pointable<u16> for opencv::core::Point {
    #[inline]
    fn point(&self) -> (u16, u16) {
        let x = u16::try_from(self.x).unwrap();
        let y = u16::try_from(self.y).unwrap();
        (x, y)
    }
}

impl Pointable<u16> for imageproc::point::Point<u16> {
    #[inline]
    fn point(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

impl Pointable<u16> for Point {
    #[inline]
    fn point(&self) -> (u16, u16) {
        (self.posx, self.posy)
    }
}