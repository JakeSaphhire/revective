use crate::graphics::point;


// Wrapper over points vector to provide simple image processing routine
struct Shape {
    vertices : Vec<point::Point>
}

impl Shape {
    fn new() -> Shape {
        Shape {vertices : Vec::new()}
    }
}