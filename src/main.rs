mod context;
mod graphics;


fn main() -> () {
    let mut f : graphics::frame::Frame = graphics::frame::Frame::new();
    let _ = f.from_image().unwrap();
    let ctx : context::Context = context::Context::new(f, 1);

    context::Context::list_ports();
    let _ = ctx.spawn().join().unwrap();
}