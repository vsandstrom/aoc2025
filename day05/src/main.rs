use std::env::{var, args};
use std::cmp::max;

fn main() {
  let filename = args().nth(1).unwrap_or("test".to_string());
  let input = std::fs::read_to_string(
    format!("{}/{}", var("CARGO_MANIFEST_DIR").unwrap(), &filename))
    .expect("no such input");

  let [ranges, id] = input.split("\n\n").collect::<Vec<&str>>()[..2] else { panic!() };
  let mut limits = ranges.lines().fold(vec!(), |mut v, x| {
    let temp: Vec<u64> = x.split("-").map(|x| { x.parse::<u64>().unwrap() }).collect();
    v.push((temp[0], temp[1]));
    v
  });

  println!("{}", id.lines().map(|s| s.parse::<u64>().unwrap()).fold(0, |sum, x| {
    for r in &limits {
      if (r.0..=r.1).contains(&x) {
        return sum + 1
      }
    }
    sum
  }));

  

  limits.sort_unstable_by(|a, b| { a.0.cmp(&b.0) });
  // merge ranges here
}
