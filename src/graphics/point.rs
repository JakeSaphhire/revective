#![allow(dead_code)]
use serialport as sp;
use crate::graphics::{Point, Drawable, Flag};

// Point structure - 
// Smallest datapacket sent to serialport, Stored in a 4096-wide bucket by default
// FLAGS:   0x8     for nobuffer
//          0x10    for clearbuffer
//          0x40    for line
//          0x80    for point
// NOTE!!:  Nobuffers are assumed to be points for now


impl Point{
    pub fn new(flags : u8, x : u16, y : u16) -> Point {
        let px : u16; let py : u16;
        if x > 4096 { px = 4096; } else { px = x }
        if y > 4096 { py = 4096; } else { py = y}
        Point{flags, posx : px, posy : py}
    }

    pub fn make_point(x : u16, y : u16) -> Point{
        Point::new(Flag::Point as u8, x, y)
    }

    pub fn make_line(x : u16, y : u16) -> Point{
        Point::new(Flag::Line as u8, x, y)
    }

    fn pack(&self) -> u32 {
        let flag32 : u32 = (self.flags as u32) << 24;
        let x32 : u32 = (self.posx as u32) << 12;
        let y32 : u32 = self.posy as u32;
        flag32 | (x32 | y32)
    }

    pub fn bufferize(&self) -> [u8; 4] {
        self.pack().to_be_bytes()
    }

    pub fn set_x(&mut self, x : u16) -> &Self {
        if x <= 4096 { self.posx = x } else { self.posx = 4096}
        self
    }

    pub fn set_y(&mut self, y : u16) -> &Self {
        if y <= 4096 { self.posy = y } else { self.posy = 4096}
        self
    }

    pub fn send(&self, port: &mut dyn sp::SerialPort) -> std::io::Result<usize> {
        port.write(&self.bufferize())
    }

    pub fn add_flag(&mut self, flag: Flag) -> &Self {
        self.flags |= flag as u8;
        self
    }
}

impl Drawable for Point {
    fn draw(&self, pvec: &mut Vec<u8>) -> () {
        pvec.extend(self.bufferize().iter());
    }
}
