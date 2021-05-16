use crate::rayt::*;
use std::{iter::FromIterator, ops::Add};

impl Add<Self> for Float3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self::from_iter(
      self
        .iter()
        .zip(rhs.iter())
        .collect::<Vec<_>>()
        .iter()
        .map(|(&l, &r)| l + r),
    )
  }
}
