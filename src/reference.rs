use minifb::{Key, Window, WindowOptions, Scale};
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    // Criação do buffer de pixels
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    // Criação da janela - com opções específicas para macOS (Scale::X2 ajuda em telas Retina)
    let mut window = Window::new(
        "Manipulação de Pixels no macOS",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("Falha ao criar janela");

    // Tempo inicial para animações
    let start_time = Instant::now();
    
    // Loop principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed().as_secs_f32();
        
        // Preencher buffer com pixels
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                // Cor baseada em posição e tempo
                let r = ((x as f32 / WIDTH as f32) * 255.0) as u32;
                let g = ((y as f32 / HEIGHT as f32) * 255.0) as u32;
                let b = ((elapsed.sin() * 0.5 + 0.5) * 255.0) as u32;
                
                // Formato de cor: 0xRRGGBB
                buffer[index] = (r << 16) | (g << 8) | b;
            }
        }
        
        // Atualiza a janela com o buffer de pixels
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Falha ao atualizar janela");
    }
}