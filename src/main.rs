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

    loop {
        let d = Vector3::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );

        if d.norm_squared() < 1.0 {
            return d;
        }
    }
}

fn background(direction: Vector3) -> Color {
    let t = 0.5 * (direction.normalize().y + 1.0);
    let one = Color::new(1.0, 1.0, 1.0);
    one + (Color::new(0.5, 0.7, 1.0) - one) * t
}

fn trace(ray: Ray, prev_intersection: Option<Intersection>) -> Color {
    let mut scene = Scene::new();
    scene.push(Box::new(Sphere {
        shape: Shape {
            material: Material {
                color: Color::new(1., 0., 0.),
                emission: Vector3::new(0., 0., 0.),
            },
        },
        position: Vector3::new(0., 0., 0.),
        radius: 1.,
    }));
    scene.push(Box::new(Sphere {
        shape: Shape {
            material: Material {
                color: Color::new(1., 1., 1.),
                emission: Vector3::new(0., 0., 0.),
            },
        },
        position: Vector3::new(0., -1001., 0.),
        radius: 1000.,
    }));

    let scene_intersection = scene.intersect(&ray);

    let directional_light = Vector3::new(1., 1., 1.).normalize();
    // ?: 単色光にしたとき、単色の球がまっくろになる（グレースケールにならない）のは正しいのか？
    let directional_light_color = Color::new(1.0, 1.0, 1.0);

    if let Some(si) = scene_intersection {
        let reflection_coefficient = 0.5;
        let target = si.position + si.normal + random_direction();
        if let Some(pi) = prev_intersection {
            // 反射元の衝突点を光源とみなす
            return (pi.material.color
                * si.material.color
                * (pi.position - si.position).dot(&si.normal)
                * reflection_coefficient
                + trace(
                    Ray {
                        origin: si.position,
                        direction: target - si.position,
                    },
                    scene_intersection,
                ))
                * 0.5 // average
                * reflection_coefficient;
        } else {
            (directional_light_color
                * si.material.color
                * directional_light.dot(&si.normal)
                * reflection_coefficient
                + trace(
                    Ray {
                        origin: si.position,
                        direction: target - si.position,
                    },
                    scene_intersection,
                ))
                * 0.5 // average
                * reflection_coefficient
        }
    } else if let Some(pi) = prev_intersection {
        let shadow_ray = Ray {
            origin: pi.position + 0.001 * pi.normal,
            direction: directional_light,
        };
        let shadow_intersection = scene.intersect(&shadow_ray);

        if let Some(_) = shadow_intersection {
            // 衝突点から光源までにオブジェクトが存在する = 影
            Color::zeroed()
        } else {
            return directional_light_color * pi.material.color * directional_light.dot(&pi.normal);
        }
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
                    p + trace(ray, None).srgb().to_color8() / SAMPLES_PER_PIXEL
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
