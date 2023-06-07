use crate::raytracing::*;
use num;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub emission: Vector3,
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub position: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Shape {
    pub material: Material,
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub shape: Shape,

    pub position: Vector3,
    pub radius: f32,
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
