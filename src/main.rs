mod image;

use chrono::Local;
use image::*;
use std::path::Path;

fn main() {
  let (width, height): (u32, u32) = (200, 100);
  let pixels: Vec<Color> = (0..width * height)
    .into_iter()
    .enumerate()
    .map(|(i, _)| {
      let x = (i as u32 % width) as f64;
      let y = (i as u32 / width) as f64;
      return Color([x / width as f64, y / height as f64, 0.5]);
    })
    .collect::<Vec<Color>>();

  let now_str = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
  let filename = format!("{}.ppm", now_str);
  let result = save(
    Path::new(&*format!("./output/{}", filename)),
    width,
    height,
    &pixels,
  );
  match result {
    Ok(path) => println!("output -> {}", path),
    Err(why) => println!("{:?}", why),
  }
}
