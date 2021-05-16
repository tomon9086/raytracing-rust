use crate::rayt::*;
use std::ops::Sub;

impl Sub<Self> for Float3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    self + -rhs
  }
}
