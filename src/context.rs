#![allow(dead_code)]
use serialport as sp;
use std::time::Instant;
use std::thread;
use std::sync::mpsc;

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
    
    pub fn spawn_buf<T : Drawable + Send + 'static>(mut self, mut frame: Frame<T>, receiver : mpsc::Receiver<Vec<T>> , deltamode: bool) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        thread::spawn(move || -> Result<(), std::io::Error> {
            print!("started buf");
            let mut length = self.sent.0;
            let mut done : bool = false;

            const LOOP_AMOUNT : u16 = 1;
            let mut i : u16 = 0;
            let mut times : Vec<u128> = Vec::new();

            let mut total_len : usize = 0;
            loop {
                // Check if we should run the loop at all
                if done || i >= length as u16 {
                    let t : f64 = times.iter().sum::<u128>() as f64;
                    println!("Sent ratio: {}/{} ({}%), in {:.3}ms ({:.3}ms per frame - {:.3} fps ) for {} frames at {}pps", 
                            self.sent.0, self.sent.1, (self.sent.0 as f32 / self.sent.1 as f32) * 100 as f32, t/1000f64, t/(length as u32 *1000) as f64, (1f64/(t/(length as u32 *1000) as f64))*1000f64, times.len(), (total_len as i64 * 1_000_000i64)/t as i64);
                    break;
                } else {
                    i += 1;
                }
                let now = Instant::now();
                {   
                    /*
                    let newframes = receiver.try_iter();
                    for newframe in newframes {
                        frame.push_frame(newframe);
                    }
                    */
                    let newframe = receiver.recv().unwrap();
                    frame.push_frame(newframe);

                    let mut buffer : Vec<u8> = Vec::<u8>::new();
                    let mut len : usize = 0;
                    // Leaves shortmode
                    buffer.extend_from_slice(&[0x7fu8; 4]);
                    frame.current_frame().iter().step_by(self.ratio as usize).for_each( |drawable| len += drawable.draw(&mut buffer, deltamode));
                    self.sent.1 += buffer.len() as i32;
                    // specially for contours

                    println!("Compression ratio: {}/{} ({}%) for {} points, {} of {} - {:.2}%", 
                        buffer.len(), len * 4, (buffer.len() as f32)/(len as f32 * 4 as f32) * 100f32, len, i, length, (i as f64 * 100f64)/(length as f64));
                    // For the purposes of debugging: 
                    let m = buffer.len() % 64;
                    if m != 0 {
                        buffer.append(&mut vec![0xde as u8; 64-m]);
                    }
                    while self.port.bytes_to_write().ok() != Some(0) {}
                    match self.port.write(&buffer[..]) { 
                        Ok(v) => self.sent.0 += v as i32,
                        Err(_e) => (),
                    };
                    total_len += len;
                }
                times.push(now.elapsed().as_micros());
                frame.pop_drawn();
            }
            
            Ok(())
        }
        )
    }
}

