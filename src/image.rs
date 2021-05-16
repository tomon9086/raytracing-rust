use crate::rayt::*;
use std::{
  fs::{create_dir_all, File},
  io::prelude::Write,
  io::Result,
  path::Path,
};

fn create_file(path: &Path) -> Result<File> {
  if let Some(parent) = path.parent() {
    if let Err(why) = create_dir_all(parent) {
      return Err(why);
    }
  }
  return File::create(path);
}

pub fn save_ppm(path: &Path, width: u32, height: u32, pixels: &[Color]) -> Result<()> {
  let mut file = create_file(path)?;
  writeln!(file, "P3")?;
  writeln!(file, "{} {}", width, height)?;
  writeln!(file, "255")?;
  for Float3([r, g, b]) in pixels {
    let f64_to_u8 = |f| (f * 255.) as u8;
    writeln!(file, "{} {} {}", f64_to_u8(r), f64_to_u8(g), f64_to_u8(b))?;
  }
  file.flush()?;
  return Ok(());
}

pub fn save(path: &Path, width: u32, height: u32, pixels: &[Color]) -> Result<String> {
  save_ppm(path, width, height, pixels)?;
  if let Some(path_str) = path.to_str() {
    return Ok(path_str.to_string());
  }
  return Ok("".to_string());
}
