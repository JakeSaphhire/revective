#![allow(dead_code)]
use serialport as sp;
use std::time::Instant;
use std::thread;

use crate::graphics as Graphics;
use crate::graphics::{Point, Frame};

pub struct Context<T : Graphics::Drawable>{
    screen: Frame<T>,
    port: Box<dyn sp::SerialPort>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl<T : Graphics::Drawable> Context<T> {
    pub fn new(f :  Frame<T>, r : i16) -> Context<T>{
        let ports = sp::available_ports().expect("Failed to list ports");
        Context {   screen : f, 
                    port : sp::new(&ports[0].port_name, 115200).open().expect("Failed to open port!"), 
                    sent : (0,0), 
                    ratio : r,
                }
    }

    pub fn set_ratio(&mut self, r : i16) -> &Context<T> {
        self.ratio = r;
        self
    }
}

impl Context<Point> {
    pub fn list_ports(){
        for port in sp::available_ports().expect("Failed to list ports") {
            println!("{}", port.port_name)
        }
    }
}

impl<T : Graphics::Drawable + Send +'static> Context<T>{
    pub fn spawn(mut self) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        thread::spawn(move || -> Result<(), std::io::Error> {
            use std::ops::{Deref, DerefMut};
            const LOOP_AMOUNT : u8 = 10;
            let mut frame = self.screen;
            let mut i : u16 = 0;
            let mut times : Vec<u128> = Vec::new();
            loop {
                if i >= LOOP_AMOUNT as u16 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {} pt/s", self.sent.0, self.sent.1, (self.sent.0 as f32 / self.sent.1 as f32) * 100 as f32, t, t/LOOP_AMOUNT as f64, times.len(), (self.sent.0  * 1000)/t as i32);
                    return Ok(());
                } else {
                    if i == 0 {
                        frame.swap();
                    }
                    i += 1;
                }
                let now = Instant::now();
                {
                    let mut flip : i16 = 0;
                    for drawable in frame.drawbuffer().deref().iter() {
                        if flip == self.ratio {
                            self.sent.1 += 1;
                            if match self.port.bytes_to_write() {
                                Ok(r) => r,
                                Err(_e) => return Ok(()),
                            } != 0 {
                                thread::sleep(std::time::Duration::from_nanos(1));
                            }
                            match drawable.draw(self.port.deref_mut()) {
                                Ok(v) => {self.sent.0 += v as i32;}
                                Err(_e) => {/*println!("{:?}", _e)*/}, 
                            }
                            flip = 0;
                        } else {
                            flip += 1;
                        } 
                    }
                }
                times.push(now.elapsed().as_millis()); 
                //frame.drawbuffer().deref_mut().clear();
            }
        }
        )
    }
}

impl Context<Point> {
    pub fn spawn_buf(mut self) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        thread::spawn(move || -> Result<(), std::io::Error> {
            use std::ops::{Deref, DerefMut};
            const LOOP_AMOUNT : u16 = 1000;
            let mut frame = self.screen;
            let mut i : u16 = 0;
            let mut times : Vec<u128> = Vec::new();
            loop {
                if i >= LOOP_AMOUNT as u16 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {}pps", self.sent.0, self.sent.1, (self.sent.0 as f32 / self.sent.1 as f32) * 100 as f32, t, t/LOOP_AMOUNT as f64, times.len(), (self.sent.0 as i64 * 1000i64)/t as i64);
                    return Ok(());
                } else {
                    if i == 0 {
                        frame.swap();
                    }
                    i += 1;
                }
                let now = Instant::now();
                {
                    let buffer : Vec<u8> = frame.drawbuffer().iter().step_by(self.ratio as usize).map(|point : &Point| point.bufferize()).flatten().collect();
                    self.sent.1 += buffer.len() as i32/4;
                    while self.port.bytes_to_write().ok() != Some(0) {}
                    match self.port.write(&buffer[..]) { 
                        Ok(v) => self.sent.0 += v as i32/4,
                        Err(_e) => (),
                    };
                }
                times.push(now.elapsed().as_millis()); 
                //frame.drawbuffer().deref_mut().clear();
            }
        }
        )
    }
}