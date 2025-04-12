use minifb::{Key, Window};
use std::time::Duration;
use std::time::Instant;

pub struct Renderer {
    buffer: Vec<u32>,
    window: Window,
    delta_time: Duration,
    width: usize,
    height: usize
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        Self {
            buffer: vec![0; window.get_size().0 * window.get_size().1],
            width: window.get_size().0,
            height: window.get_size().1,
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

        let mut start_time = Instant::now();
        let mut walking_pos = 0f32;
        let mut fps = 0f32;
        let mut frame_time = Instant::now();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.delta_time = Instant::now() - start_time;
            start_time = Instant::now();
            walking_pos += self.delta_time.as_secs_f32();

            for y in 0..self.height {
                for x in 0..self.width {

                    let index = y * self.width+ x;

                    let r = ((x as f32 / self.width as f32) * 255.0) as u32;
                    let g = ((y as f32 / self.height as f32) * 255.0) as u32;
                    let b = ((walking_pos.sin() * 0.5 + 0.5) * 255.0) as u32;
                    let pixel_color = (r << 16) | (g << 8) | b;

                    self.buffer[index] = pixel_color;

                }
            }

            if frame_time.elapsed() > Duration::from_nanos(1_000_000_000) {
                println!("fps = {}", fps);
                fps = 0.0;
                frame_time = Instant::now();
            }

            fps += 1.0;
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .expect("Panic on update window");
        }


    }
}

