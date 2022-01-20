#![allow(dead_code)]
use serialport as sp;
use std::time::Instant;
use std::thread;
use crate::graphics::frame;
use crate::graphics as Graphics;

pub struct Context<T : Graphics::Draw>{
    screen: frame::Frame<T>,
    port: Box<dyn sp::SerialPort>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl<T : Graphics::Draw> Context<T> {
    pub fn new(f :  frame::Frame<T>, r : i16) -> Context<T>{
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

impl Context<Graphics::point::Point> {
    pub fn list_ports(){
        for port in sp::available_ports().expect("Failed to list ports") {
            println!("{}", port.port_name)
        }
    }
}

impl<T : Graphics::Draw + Send + 'static> Context<T>{
    pub fn spawn(mut self) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        let context = thread::spawn(move || -> Result<(), std::io::Error> {
            use std::ops::{Deref, DerefMut};
            
            println!("{:#?}", Some(self.port.name()));
            let mut frame = self.screen;
            let mut flip : i16 = 0;
            let mut i : i16 = 0;
            let mut times : Vec<u128> = Vec::new();
            loop {
                if i >= 1 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames", self.sent.0, self.sent.1, self.sent.0 as f32 / self.sent.1 as f32, t, t/1000 as f64, times.len());
                    return Ok(());
                } else if i == 0 {
                    frame.swap();
                    i += 1;
                } else {
                    i += 1;
                }
                let now = Instant::now();
                {
                    for point in frame.drawbuffer().deref().iter() {
                        if flip == self.ratio {
                            self.sent.1 += 1;
                            if match self.port.bytes_to_write() {
                                Ok(r) => r,
                                Err(_e) => return Ok(()),
                            } != 0 {
                                thread::sleep(std::time::Duration::from_nanos(1));
                            }
                            match point.draw(self.port.deref_mut()) {
                                Ok(_v) => {self.sent.0 += 1;}
                                Err(_e) => {println!("{:?}", _e)}, 
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
        );
        context
    }
}