use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float3(pub [f64; 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Float3;

impl Float3 {
  pub fn x(&self) -> f64 {
    self.to_array()[0]
  }

  pub fn y(&self) -> f64 {
    self.to_array()[1]
  }

  pub fn z(&self) -> f64 {
    self.to_array()[2]
  }
}

impl Display for Float3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    let a = self.to_array();
    write!(f, "({}, {}, {})", a[0], a[1], a[2])
  }
}
