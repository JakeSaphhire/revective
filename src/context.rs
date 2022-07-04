#![allow(dead_code)]
use serialport as sp;
use std::time::Instant;
use std::{thread, collections::HashMap};
use std::sync::{Arc, Mutex};

use crate::graphics::{Point, Frame, Drawable, Buffer};

pub struct Context{
    port: Box<dyn sp::SerialPort>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl Context {
    pub fn new(r : i16) -> Context{
        let ports = sp::available_ports().expect("Failed to list ports");
        Context {
                    port : sp::new(&ports[0].port_name, 450000).open().expect("Failed to open port!"), 
                    sent : (0,0), 
                    ratio : r,
                }
    }

    pub fn set_ratio(&mut self, r : i16) -> &Context {
        self.ratio = r;
        self
    }

    pub fn list_ports(){
        for port in sp::available_ports().expect("Failed to list ports") {
            println!("{}", port.port_name)
        }
    }
}


impl Context {
    
    pub fn spawn_buf<T : Drawable + Send + 'static>(mut self, frame: Arc<Mutex<Frame<T>>>) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        thread::spawn(move || -> Result<(), std::io::Error> {
            const LOOP_AMOUNT : u16 = 1;
            let mut i : u16 = 0;
            let mut times : Vec<u128> = Vec::new();
            loop {
                if i >= LOOP_AMOUNT as u16 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {}pps", self.sent.0, self.sent.1, (self.sent.0 as f32 / self.sent.1 as f32) * 100 as f32, t/1000f64, t/(LOOP_AMOUNT as u32 *1000) as f64, times.len(), (self.sent.0 as i64 * 1_000_000i64)/t as i64);
                    break;
                } else {
                    i += 1;
                }
                let now = Instant::now();
                {   
                    let mut buffer : Vec<u8> = Vec::<u8>::new();
                    //frame.lock().unwrap().current_frame().iter().for_each( |drawable| drawable.draw(buffer.as_mut()));
                    // TODO: Pagination!
                    
                    let mut map : HashMap<u8, Vec<u8>> = HashMap::new();
                    for i in 0..255 {
                        map.insert(i as u8, Vec::<u8>::new());
                    }
                    let mut fbuf : Buffer<Vec<u8>, HashMap<u8, Vec<u8>>> = Buffer::M(map);
                    frame.lock().unwrap().current_frame().iter().for_each( |drawable| drawable.draw(&mut fbuf, true));
                    

                    self.sent.1 += buffer.len() as i32/4; 
                    while self.port.bytes_to_write().ok() != Some(0) {}
                    match self.port.write(&buffer[..]) { 
                        Ok(v) => self.sent.0 += v as i32/4,
                        Err(_e) => (),
                    };
                }
                times.push(now.elapsed().as_micros());
                
            }
            //frame.pop_drawn();
            Ok(())
        }
        )
    }
}