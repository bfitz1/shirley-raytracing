mod vector;

use vector::Vector;
use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
    let opts = WindowOptions { scale: Scale::X4, ..WindowOptions::default() };
    let mut window = match Window::new("Raytracer", WIDTH, HEIGHT, opts) {
        Ok(win) => win,
        Err(err) => panic!("Unable to create window: {}", err),
    };

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (p, i) in buffer.iter_mut().enumerate() {
            let col = Vector::new(
                (p % WIDTH) as f64 / WIDTH as f64,
                (p / WIDTH) as f64 / HEIGHT as f64,
                51.0
            );
            *i = u32::from_be_bytes([
                0x00,
                (col.x * 255.99) as u8,
                ((1.0 - col.y) * 255.99) as u8,
                col.z as u8
            ]);
        }
        window.update_with_buffer(&buffer).unwrap();
    }
}
