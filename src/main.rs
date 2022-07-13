use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use rgb::RGB;
use std::{fs, io, path};

type Color = RGB<u8>;

fn main() {
    let bounds = (500, 500);
    let mut image_buffer = vec![0; bounds.0 * bounds.1]
        .iter()
        .enumerate()
        .map(|(i, _p)| {
            let x = i % bounds.0;
            let y = i / bounds.0;
            let u = x as f32 / bounds.0 as f32;
            let v = y as f32 / bounds.1 as f32;

            Color {
                r: (u * 255.) as u8,
                g: (v * 255.) as u8,
                b: 128,
            }
        })
        .collect::<Vec<Color>>();

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

fn save_image(filename: &str, pixels: &[Color], bounds: (usize, usize)) -> Result<(), io::Error> {
    path::Path::new(filename).parent().and_then(|p| {
        if !p.exists() {
            let _ = fs::create_dir_all(p);
        }
        Some(())
    });

    let file = fs::File::create(filename)?;

    let encoder = PngEncoder::new(file);
    let _ = encoder.write_image(
        &pixels
            .iter()
            .flat_map(|p| [p.r, p.g, p.b])
            .collect::<Vec<u8>>(),
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Rgb8,
    );

    Ok(())
}
