use crate::rayt::*;
use std::{iter::FromIterator, ops::Neg};

impl Neg for Float3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self::from_iter(self.iter().map(|item| item * -1.))
  }
}
