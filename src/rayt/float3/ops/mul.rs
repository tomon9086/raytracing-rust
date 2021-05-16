use crate::rayt::*;
use std::{iter::FromIterator, ops::Mul};

impl Mul<f64> for Float3 {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self::Output {
    Self::from_iter(self.iter().map(|item| item * rhs))
  }
}

impl Mul<Float3> for f64 {
  type Output = Float3;
  fn mul(self, rhs: Float3) -> Self::Output {
    Float3::from_iter(rhs.iter().map(|&item| self * item))
  }
}
