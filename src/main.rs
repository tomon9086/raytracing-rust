#![allow(dead_code)]

mod raytracing;

use crate::raytracing::*;
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use std::{fs, io, path};

trait ToColor8 {
    fn to_color8(&self) -> Color8;
}

impl ToColor8 for Color {
    fn to_color8(&self) -> Color8 {
        self.iter().map(|c| (c * 255.) as u8).collect::<Color8>()
    }
}

fn save_image(filename: &str, pixels: &[Color8], bounds: (usize, usize)) -> Result<(), io::Error> {
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

fn main() {
    let bounds = (500, 500);

    let scene = [
        Sphere {
            shape: Shape {
                material: Material {
                    color: Color::new(1., 0., 0.),
                    emission: Vector3::new(0., 0., 0.),
                },
            },
            position: Vector3::new(0., 0., 0.),
            radius: 1.,
        },
        Sphere {
            shape: Shape {
                material: Material {
                    color: Color::new(1., 1., 1.),
                    emission: Vector3::new(0., 0., 0.),
                },
            },
            position: Vector3::new(0., -100001., 0.),
            radius: 100000.,
        },
    ];
    // let point_light = Vector3::new(2., 5., 2.);
    let directional_light = Vector3::new(2., 5., 2.).normalize();

    let mut image_buffer = vec![0; bounds.0 * bounds.1]
        .iter()
        .enumerate()
        .map(|(i, _p)| {
            let x = i % bounds.0;
            let y = i / bounds.0;
            let u = x as f32 / bounds.0 as f32;
            let v = 1. - y as f32 / bounds.1 as f32;

            let ray = Ray {
                origin: Vector3::new(0., 0., 5.),
                direction: Vector3::new(u - 0.5, v - 0.5, -1.).normalize(),
            };
            let mut min: Option<Intersection> = None;
            for shape in scene {
                let intersection: Option<Intersection> = shape.intersect(&ray);

                min = match (min, intersection) {
                    (Some(m), Some(i)) => {
                        if m.distance > i.distance {
                            Some(i)
                        } else {
                            min
                        }
                    }
                    (None, Some(i)) => Some(i),
                    _ => min,
                }
            }
            if let Some(m) = min {
                (m.material.color * (directional_light.dot(&m.normal))).to_color8()
            } else {
                Color8::new(0, 0, 0)
            }
        })
        .collect::<Vec<Color8>>();

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
