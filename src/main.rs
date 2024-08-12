use gif::{DecodeOptions};
use minifb::{Key, Window, WindowOptions};
use rayon::iter::ParallelIterator;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = PathBuf::from("test.gif");

    play_gif(&input_path)?;
    Ok(())
}

fn play_gif(input_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);

    let file = File::open(input_path)?;
    let mut decoder = options.clone().read_info(std::io::BufReader::new(file))?;

    let mut window = Window::new(
        "GIF Player",
        decoder.width() as usize,
        decoder.height() as usize,
        WindowOptions::default(),
    )?;

    let mut buffer = vec![0u32; decoder.width() as usize * decoder.height() as usize];
    let mut last_update = Instant::now();
    let mut paused = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }
        if !paused && last_update.elapsed() >= Duration::from_millis(100) {
            if let Ok(Some(frame)) = decoder.read_next_frame() {
                for (i, chunk) in frame.buffer.chunks(4).enumerate() {
                    let r = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let b = chunk[2] as u32;
                    let a = chunk[3] as u32;
                    buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
                }
                window.update_with_buffer(&buffer, decoder.width() as usize, decoder.height() as usize)?;
            } else {
                let file = File::open(input_path)?;
                decoder = options.clone().read_info(std::io::BufReader::new(file))?;
            }
            last_update = Instant::now();
        } else {
            window.update();
        }
    }

    Ok(())
}