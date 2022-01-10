use serialport as sp;
use std::time::{Duration, Instant};
use std::thread;
mod context;
mod graphics;


fn main() -> () {
    let mut f : graphics::frame::Frame = graphics::frame::Frame::new();
    let ctx : context::Context = context::Context::new(f, 5);
    ctx.spawn();
}