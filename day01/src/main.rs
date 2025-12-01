use std::env::args;

fn main() {
  let mut input = std::fs::read_to_string(
    format!("day01/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  let mut i = 50;
  let mut z = 0;

  input.lines().for_each(|s| {
    let num = s[1..].parse::<i32>().unwrap();
    match &s[..1] {
      "R" => {
        i += num;
        i %= 100;
      },
      "L" => {
        i -= num;
        while i < 0 { i += 100; }
      },
      _ => ()
    }
    if i == 0 { z += 1; }
  });
  println!("{z}");

  i = 50;
  z = 0;
  input.lines().for_each(|s| {
    let mut num = s[1..].parse::<i32>().unwrap();
    let mut mult = 0;
    match &s[..1] {
      "R" => {
        for _ in 0..num {
          i += 1;
          i %= 100;
          if i == 0 { z += 1; }
        }
      },
      "L" => {
        for _ in 0..num {
          i -= 1;
          if i < 0 { i += 100; }
          if i == 0 { z += 1; }
        }
      },
      _ => ()
    }
    println!("{i}");
  });
  println!("{z}");
}
