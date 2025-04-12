use minifb::{Window, WindowOptions};

mod graphics;

fn main() {
    let win_options = WindowOptions::default();
    let window = Window::new("banana", 1024, 780, win_options);
    let mut renderer = graphics::Renderer::new(window.unwrap());
    renderer.print();
    renderer.init();

}