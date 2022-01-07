use serialport as sp;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::Mutex;
use crate::graphics::frame;

pub struct Context<'a> {
    screen: Mutex<&'a frame::Frame<'a>>,
    port: Box<dyn sp::SerialPort>,
    pub sent : (i32, i32),
    pub ratio : i16
}

impl<'a> Context<'a> {
    pub fn new(f : &'a frame::Frame) -> Context<'a>{
        let c : Context;
        let ports = sp::available_ports().expect("Failed to list ports");
        c.port = sp::new(&ports[0].port_name, 115200).open().expect("Failed to open port");
        c.ratio = 0; c.sent = (0,0); c.screen = Mutex::new(f);
        c
    }

    pub fn setRatio(&self, r : i16) -> &Context<'a> {
        self.ratio = r;
        &self
    }

    // Todo
    pub fn debuginfo(){}
    fn draw(){}
}

impl Context<'static>{
    pub fn spawn(&self) -> thread::JoinHandle<Result<(), std::io::Error>> {
        let flip : i16 = 0;
        let i : i16 = 0;
        let times : Vec<u128> = Vec::new();
        let context = thread::spawn(|| -> Result<(), std::io::Error> {
            let frameptr = self.screen.get_mut().unwrap();
            loop {
                if i >= 1000 {return Ok(());} else {i += 1;}

                let guard = self.screen.lock().unwrap();
                frameptr.swap();
                std::mem::drop(guard);

                let now = Instant::now();
                for point in frameptr.drawbuffer.iter() {
                    match point {
                        None => (),
                        Some(pt) => {
                            if flip == self.ratio {
                                self.sent.1 += 1;
                                if match self.port.bytes_to_write() {
                                    Ok(r) => r,
                                    Err(_e) => return Ok(()),
                                } != 0 {
                                    thread::sleep(std::time::Duration::from_nanos(1));
                                }
                                match pt.send(self.port.as_mut()) {
                                    Ok(_v) => {self.sent.0 += 1;}
                                    Err(_e) => (), 
                                }
                                flip = 0;
                            } else {
                                flip += 1;
                            } 
                        }
                    }   
                }
                times.push(now.elapsed().as_millis());
                frameptr.clear();
        }}
        );
        context
    }
}