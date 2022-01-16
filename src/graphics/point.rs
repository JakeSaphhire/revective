#![allow(dead_code)]
use serialport as sp;

pub struct Point{
    pub flags: u8,
    posx : u16,
    posy : u16,
}

impl Point{
    pub fn new(flags : u8, x : u16, y : u16) -> Point {
        let px : u16; let py : u16;
        if x > 4096 { px = 4096; } else { px = x }
        if y > 4096 { py = 4096; } else { py = y}
        Point{flags: flags, posx : px, posy : py}
    }

    fn pack(&self) -> u32 {
        let flag32 : u32 = (self.flags as u32) << 24;
        let x32 : u32 = (self.posx as u32) << 12;
        let y32 : u32 = self.posy as u32;
        flag32 | (x32 | y32)
    }

    fn bufferize(&self) -> [u8; 4] {
        self.pack().to_be_bytes()
    }

    pub fn set_x(&mut self, x : u16) -> &Self {
        if x <= 4096 { self.posx = x } else { self.posx = 4096}
        return self;
    }

    pub fn set_y(&mut self, y : u16) -> &Self {
        if y <= 4096 { self.posy = y } else { self.posy = 4096}
        return self;
    }

    pub fn send(&self, port: &mut dyn sp::SerialPort) -> std::io::Result<usize> {
        port.write(&self.bufferize())
    }

}


