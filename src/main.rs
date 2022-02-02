mod graphics;
mod context;

use crate::graphics::{Frame, Point};
use context::Context;

fn main() -> () {
    let mut f : Frame<Point> = Frame::new();
    let _ = f.from_image().unwrap();
    let ctx : Context<Point> = Context::new(f, 8);

    Context::list_ports();
    let _ = ctx.spawn().join().unwrap();
}