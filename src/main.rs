mod graphics;

fn main() {
    
    let mut renderer = graphics::Renderer::new(1024, 768);
    renderer.print();
    renderer.init();

}