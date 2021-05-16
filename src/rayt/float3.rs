#[derive(Debug, PartialEq)]
pub struct Float3(pub [f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Float3 {
  pub const fn new(x: f64, y: f64, z: f64) -> Self {
    Self([x, y, z])
  }
}
