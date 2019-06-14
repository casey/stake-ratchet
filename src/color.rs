#[allow(unused)]
fn color(red: f64, green: f64, blue: f64, alpha: f64) -> String {
  fn quantize(f: f64) -> i64 {
    (f.max(0.0).min(1.0) * 255.0) as i64
  }

  format!(
    "#{:02X}{:02X}{:02X}{:02X}",
    quantize(1.0 - alpha),
    quantize(red),
    quantize(green),
    quantize(blue)
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_formatting() {
    assert_eq!(color(2.0, 3.0, 4.0, 5.0), "#FFFFFFFF");
    assert_eq!(color(1.0, 1.0, 1.0, 1.0), "#FFFFFFFF");
    assert_eq!(color(0.5, 0.5, 0.5, 0.5), "#7F7F7F7F");
    assert_eq!(color(0.0, 0.0, 0.0, 0.0), "#00000000");
    assert_eq!(color(0.0, 0.0, 0.0, 0.0), "#00000000");
  }
}
