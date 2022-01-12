use serialport as sp;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::Mutex;
use crate::graphics::frame;

pub struct Context{
    screen: frame::Frame,
    port: Box<dyn sp::SerialPort>,
    times : Vec<u128>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl Context {
    pub fn new(f :  frame::Frame, r : i16) -> Context{
        let ports = sp::available_ports().expect("Failed to list ports");
        Context {   screen : f, 
                    port : sp::new(&ports[0].port_name, 115200).open().expect("Failed to open port!"), 
                    sent : (0,0), 
                    ratio : r,
                    times: Vec::new()
                }
    }

    pub fn list_ports(){
        for port in sp::available_ports().expect("Failed to list ports") {
            println!("{}", port.port_name)
        }
    }

    pub fn debug(&self){
        println!("{:.2}", self.sent.0 as f32 / self.sent.1 as f32)
    }

    pub fn setRatio(&mut self, r : i16) -> &Context {
        self.ratio = r;
        self
    }

    // Todo
    pub fn debuginfo(){}
    fn draw(){}
}

impl Context{
    pub fn spawn(mut self) -> thread::JoinHandle<Result<(), std::io::Error>> {    
        let context = thread::spawn(move || -> Result<(), std::io::Error> {
            println!("{:#?}", Some(self.port.name()));
            let mut frame = self.screen;
            use std::ops::{Deref, DerefMut};
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
                            match point.send(self.port.deref_mut()) {
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