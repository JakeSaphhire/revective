#![allow(dead_code)]
use serialport as sp;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::graphics::{Frame, Drawable};

pub struct Context{
    port: Box<dyn sp::SerialPort>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl Context {
    pub fn new(r : i16) -> Context{
        let ports = sp::available_ports().expect("Failed to list ports");
        Context {
                    port : sp::new(&ports[0].port_name, 2000000).open().expect("Failed to open port!"), 
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
    
    pub fn spawn_buf<T : Drawable + Send + 'static>(mut self, frame: Arc<Mutex<Frame<T>>>, deltamode: bool) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        thread::spawn(move || -> Result<(), std::io::Error> {
            print!("started buf");
            const LOOP_AMOUNT : u16 = 10;
            let mut i : u16 = 0;
            let mut times : Vec<u128> = Vec::new();

            let mut total_len : usize = 0;
            loop {
                if i >= LOOP_AMOUNT as u16 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {}ms ({}ms per frame) for {} frames at {}pps", self.sent.0, self.sent.1, (self.sent.0 as f32 / self.sent.1 as f32) * 100 as f32, t/1000f64, t/(LOOP_AMOUNT as u32 *1000) as f64, times.len(), (total_len as i64 * 1_000_000i64)/t as i64);
                    break;
                } else {
                    i += 1;
                }
                let now = Instant::now();
                {   
                    let mut buffer : Vec<u8> = Vec::<u8>::new();
                    // TODO: deltaxy!!! (DONE!)
                    let mut len : usize = 0;
                    // Leaves shortmode
                    buffer.extend_from_slice(&[0x7fu8; 4]);
                    frame.lock().unwrap().current_frame().iter().step_by(self.ratio as usize).for_each( |drawable| len += drawable.draw(&mut buffer, deltamode));
                    self.sent.1 += buffer.len() as i32;
                    // specially for contours

                    println!("Compression ratio: {}/{} ({}%) for {} points", buffer.len(), len * 4, (buffer.len() as f32)/(len as f32 * 4 as f32) * 100f32, len);
                    // For the purposes of debugging: 
                     
                    while self.port.bytes_to_write().ok() != Some(0) {}
                    match self.port.write(&buffer[..]) { 
                        Ok(v) => self.sent.0 += v as i32,
                        Err(_e) => (),
                    };
                    total_len += len;
                }
                times.push(now.elapsed().as_micros());
                
            }
            //frame.pop_drawn();
            Ok(())
        }
        )
    }
}

