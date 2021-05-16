use crate::rayt::*;
use std::{iter::FromIterator, slice::Iter};

impl Float3 {
  pub fn to_array(&self) -> [f64; 3] {
    self.0
  }

  pub fn iter(&self) -> Iter<'_, f64> {
    self.0.iter()
  }

  pub fn sqrt(&self) -> Self {
    Self::from_iter(self.iter().map(|item| item.sqrt()))
  }

  pub fn near_zero(&self) -> bool {
    self.iter().all(|item| item.abs() < EPS)
  }

  pub fn saturate(&self) -> Self {
    Self::from_iter(self.iter().map(|item| item.min(1.).max(0.)))
  }

  pub fn dot(&self, rhs: Self) -> f64 {
    self
      .iter()
      .zip(rhs.iter())
      .collect::<Vec<_>>()
      .iter()
      .fold(0., |p, (&l, &r)| p + l * r)
  }
}
