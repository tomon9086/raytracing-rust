use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Float3(pub [f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Float3 {
  pub const fn new(x: f64, y: f64, z: f64) -> Self {
    Self([x, y, z])
  }

  pub const fn zero() -> Self {
    Self([0.; 3])
  }
}

impl FromIterator<f64> for Float3 {
  fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
    let mut it = iter.into_iter();
    let [mut x, mut y, mut z] = [0.; 3];
    if let Some(nx) = it.next() {
      x = nx;
    }
    if let Some(ny) = it.next() {
      y = ny;
    }
    if let Some(nz) = it.next() {
      z = nz;
    }
    Self::new(x, y, z)
  }
}
