use minifb::{Key, Window, WindowOptions};
use std::time::Instant;
use std::time::Duration;

struct Pixel {
    x: u16,
    y: u16,
    argb: u32,
}

struct Renderer {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    window: Window,
    delta_time: Duration
}

impl Renderer {

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
            window: Window::new(
                "Banana Engine",
                width as usize,
                height as usize,
                WindowOptions {
                    resize: true,
                    scale: minifb::Scale::X2, 
                    ..WindowOptions::default()
                },
            ).expect("Falha ao criar janela"),
            delta_time : Duration::default(),
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32)  {
        self.width = width;
        self.height = height;
    }

    pub fn print(&mut self) {
        println!("width = {}", self.width);
        println!("height = {}", self.height);
        println!("buffer length = {}", self.buffer.len());
    }
    
    pub fn init(&mut self) {
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        let mut start_time = Instant::now();

        let mut walking_pos = 0 as f32;

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {

            self.delta_time = Instant::now() - start_time;
            start_time = Instant::now();
            println!("delta time = {}", self.delta_time.as_micros());
            walking_pos += self.delta_time.as_secs_f32();

            for y in 0..self.height {
                for x in 0..self.width {
                    let index = y * self.width + x;

                    let r = ((x as f32 / self.width as f32) * 255.0) as u32;
                    let g = ((y as f32 / self.height as f32) * 255.0) as u32;
                    let b = ((walking_pos.sin() * 0.5 + 0.5) * 255.0) as u32;
                    let pixel_color = (r << 16) | (g << 8) | b;

                    self.buffer[index as usize] = pixel_color;
                }
            }
            self.window
                .update_with_buffer(&self.buffer, self.width as usize, self.height as usize)
                .expect("Falha ao atualizar janela");
        }
    }
    
}

fn main() {
    
    let mut renderer = Renderer::new(1024, 768);
    renderer.print();
    renderer.init();
    
}