use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

fn pixel_to_coordinate(pixel: usize, width: usize, height: usize) -> (usize, usize) {
    (pixel % width, height - 1 - pixel / width)
}

fn render(width: usize, height: usize) -> Vec<u32> {
    (0..width * height).map(|pixel| {
        let (x, y) = pixel_to_coordinate(pixel, width, height);
        let (r, g, b) = (
            x as f64 / width as f64,
            y as f64 / height as f64,
            0.2,
        );
        let (ir, ig, ib) = (
            (255.99 * r) as u8,
            (255.99 * g) as u8,
            (255.99 * b) as u8,
        );
        u32::from_ne_bytes([0, ir, ig, ib])
    }).collect()
}

fn main() {
    let mut window = Window::new("Raytracer", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| panic!("{}", e));
    
    let buffer = render(WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap_or_else(|e| panic!("{}", e));
    }
}