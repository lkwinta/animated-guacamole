extern crate glium;

fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let window = glium::glutin::window::WindowBuilder::new()
        .with_siz(1024, 768)
        .with_title("Hello world");


}