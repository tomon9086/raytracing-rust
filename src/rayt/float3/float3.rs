use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Float3(pub [f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Display for Float3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let a = self.to_array();
    write!(f, "({}, {}, {})", a[0], a[1], a[2])
  }
}
