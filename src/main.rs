use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
use nalgebra;
use num;
use rgb::RGB;
use std::{fs, io, path};

type Color = RGB<u8>;
type Vector3 = nalgebra::Vector3<f32>;

const EPS: f32 = 0.001;

fn main() {
    let bounds = (500, 500);

    let scene = [
        Sphere {
            shape: Shape {
                material: Material {
                    color: Vector3::new(1., 0., 0.),
                    emission: Vector3::new(0., 0., 0.),
                },
            },
            position: Vector3::new(0., 0., 0.),
            radius: 1.,
        },
        Sphere {
            shape: Shape {
                material: Material {
                    color: Vector3::new(1., 1., 1.),
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
                let color_vec = m.material.color * (directional_light.dot(&m.normal));
                Color {
                    r: (color_vec.x * 255.) as u8,
                    g: (color_vec.y * 255.) as u8,
                    b: (color_vec.z * 255.) as u8,
                }
            } else {
                Color { r: 0, g: 0, b: 0 }
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

struct Ray {
    origin: Vector3,
    direction: Vector3,
}

#[derive(Copy, Clone)]
struct Material {
    // TODO: Color型にしたい
    color: Vector3,
    emission: Vector3,
}

#[derive(Copy, Clone)]
struct Intersection {
    position: Vector3,
    normal: Vector3,
    distance: f32,
    material: Material,
}

#[derive(Copy, Clone)]
struct Shape {
    material: Material,
}

trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone)]
struct Sphere {
    shape: Shape,

    position: Vector3,
    radius: f32,
    // material: Material,
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let po = ray.origin - self.position;
        let b = ray.direction.dot(&po);

        let c = num::pow(po.norm(), 2) - self.radius * self.radius;
        let det = b * b - c;
        if det < 0. {
            return None;
        }
        let t1 = -b - num::Float::sqrt(det);
        let t2 = -b + num::Float::sqrt(det);
        if t1 < EPS && t2 < EPS {
            return None;
        }
        let distance = if t1 > EPS { t1 } else { t2 };
        let position = ray.origin + ray.direction * distance;
        let normal = (position - self.position).normalize();

        Some(Intersection {
            position: position,
            normal: normal,
            distance: distance,
            material: self.shape.material,
        })
    }
}
