#![allow(dead_code)]

mod raytracing;

use crate::raytracing::*;
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rgb::Zeroable;
use std::{fs, io, path};

const BOUNDS: (usize, usize) = (500, 500);
const SAMPLES_PER_PIXEL: u8 = 8;

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

fn random_direction() -> Vector3 {
    let mut rng = thread_rng();

    Vector3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    )
    .normalize()
}

fn background(direction: Vector3) -> Color {
    let t = 0.5 * (direction.normalize().y + 1.0);
    let one = Color::new(1.0, 1.0, 1.0);
    one + (Color::new(0.5, 0.7, 1.0) - one) * t
}

fn trace(ray: Ray, intersection: Option<Intersection>) -> Color {
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
            position: Vector3::new(0., -1001., 0.),
            radius: 1000.,
        },
    ];
    // let point_light = Vector3::new(2., 5., 2.);
    let directional_light = Vector3::new(2., 5., 2.).normalize();

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
        let target = m.position + m.normal + random_direction();
        trace(
            Ray {
                origin: m.position,
                direction: target - m.position,
            },
            min,
        ) * 0.5
    } else if let Some(i) = intersection {
        i.material.color * directional_light.dot(&i.normal)
    } else {
        background(ray.direction)
    }
}

fn render() -> Vec<Color8> {
    vec![0; BOUNDS.0 * BOUNDS.1]
        .par_iter()
        .enumerate()
        .map(|(i, _p)| {
            (0..SAMPLES_PER_PIXEL)
                .into_iter()
                .fold(Color8::zeroed(), |p, _| {
                    let mut rng = thread_rng();

                    let x = i % BOUNDS.0;
                    let y = i / BOUNDS.0;
                    let rx = rng.gen::<f32>();
                    let ry = rng.gen::<f32>();
                    let u = (x as f32 + rx) / BOUNDS.0 as f32;
                    let v = 1. - (y as f32 + ry) / BOUNDS.1 as f32;

                    let ray = Ray {
                        origin: Vector3::new(0., 0., 5.),
                        direction: Vector3::new(u - 0.5, v - 0.5, -1.).normalize(),
                    };
                    p + trace(ray, None).to_color8() / SAMPLES_PER_PIXEL
                })
        })
        .collect::<Vec<Color8>>()
}

fn main() {
    let mut image_buffer = render();

    save_image(
        &format!(
            "output/{}.png",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ),
        &mut image_buffer,
        BOUNDS,
    )
    .expect("error writing image");
}
