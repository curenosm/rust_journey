use image::{self, RgbImage};
use rand;

pub fn generate_fractal_triangle() {
  let v1 = (0.0, 0.0);
  let v2 = (1.0, 0.0);
  let v3 = (0.5, 1.0);

  let p0 = (0.5, 0.5);
  let points_v = vec![v1, v2, v3];
  let mut points_t = vec![p0];

  // Lambda function to get the midpoint of two points
  let midpoint = |p1: (f64, f64), p2: (f64, f64)| {
    let x = (p1.0 + p2.0) / 2.0;
    let y = (p1.1 + p2.1) / 2.0;
    (x, y)
  };

  let iters = 100_000;

  for _ in 0..iters {
    let i = rand::random::<usize>() % 3;
    let v_act = points_v[i];
    let p_act = points_t[points_t.len() - 1];
    let p_sig = midpoint(p_act, v_act);
    points_t.push(p_sig);
  }

  for point in &points_t {
    println!("{:?}", point);
  }

  // Generate an image what we just calculated
  let mut img = RgbImage::new(32, 32);

  for p in points_t {
    let x = (p.0 * 1000.0) as u32;
    let y = (p.1 * 1000.0) as u32;
    img.put_pixel(x, y, image::Rgb([255, 255, 255]));
  }
}
