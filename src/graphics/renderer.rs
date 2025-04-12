use minifb::{Key, Window};
use std::time::Duration;
use std::time::Instant;

pub struct Renderer {
    buffer: Vec<u32>,
    window: Window,
    delta_time: Duration,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        Self {
            buffer: vec![0; window.get_size().0 * window.get_size().1],
            window,
            delta_time: Duration::default(),
        }
    }

    pub fn print(&mut self) {
        println!("width = {}", self.window.get_size().0);
        println!("height = {}", self.window.get_size().1);
        println!("buffer length = {}", self.buffer.len());
    }

    pub fn init(&mut self) {

        self.window.limit_update_rate(Some(Duration::from_micros(16600)));
        let mut start_time = Instant::now();
        let mut walking_pos = 0f32;

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.delta_time = Instant::now() - start_time;
            start_time = Instant::now();
            println!("delta time = {}", self.delta_time.as_micros());
            walking_pos += self.delta_time.as_secs_f32();

            for y in 0..self.window.get_size().1 {
                for x in 0..self.window.get_size().0 {
                    let index = y * self.window.get_size().0+ x;

                    let r = ((x as f32 / self.window.get_size().0 as f32) * 255.0) as u32;
                    let g = ((y as f32 / self.window.get_size().1 as f32) * 255.0) as u32;
                    let b = ((walking_pos.sin() * 0.5 + 0.5) * 255.0) as u32;
                    let pixel_color = (r << 16) | (g << 8) | b;

                    self.buffer[index] = pixel_color;
                }
            }
            self.window
                .update_with_buffer(&self.buffer, self.window.get_size().0, self.window.get_size().1)
                .expect("Panic on update window");
        }
    }
}

