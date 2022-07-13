use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use std::{fs, io, path};

fn main() {
    let bounds = (500, 500);
    let mut image_buffer = vec![0; bounds.0 * bounds.1 * 3]
        .iter()
        .enumerate()
        .map(|(i, _p)| {
            let x = (i / 3) % bounds.0;
            let y = (i / 3) / bounds.0;
            let u = x as f32 / bounds.0 as f32;
            let v = y as f32 / bounds.1 as f32;
            let rgb = i % 3;

            if rgb == 0 {
                (u * 255.) as u8
            } else if rgb == 1 {
                (v * 255.) as u8
            } else {
                128
            }
        })
        .collect::<Vec<u8>>();

    save_image(
        &format!(
            "output/{}.png",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ),
        &mut image_buffer,
        bounds,
    )
    .expect("error writing image");
}

fn save_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), io::Error> {
    path::Path::new(filename).parent().and_then(|p| {
        if !p.exists() {
            let _ = fs::create_dir_all(p);
        }
        Some(())
    });

    let file = fs::File::create(filename)?;

    let encoder = PngEncoder::new(file);
    let _ = encoder.write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8);

    Ok(())
}
