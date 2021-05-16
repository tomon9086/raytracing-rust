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

  pub fn cross(&self, rhs: Self) -> Self {
    let l = self.to_array();
    let r = rhs.to_array();
    Self([
      l[1] * r[2] - l[2] * r[1],
      l[2] * r[0] - l[0] * r[2],
      l[0] * r[1] - l[1] * r[0],
    ])
  }

  pub fn norm_sq(&self) -> f64 {
    self.iter().fold(0., |p, c| p + c * c)
  }

  pub fn norm(&self) -> f64 {
    self.norm_sq().sqrt()
  }

  pub fn normalize(&self) -> Self {
    *self / self.norm()
  }
}
