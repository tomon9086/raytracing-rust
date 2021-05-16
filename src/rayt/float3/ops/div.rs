use crate::rayt::*;
use std::{iter::FromIterator, ops::Div};

impl Div<f64> for Float3 {
  type Output = Self;
  fn div(self, rhs: f64) -> Self::Output {
    Self::from_iter(self.iter().map(|item| item / rhs))
  }
}
