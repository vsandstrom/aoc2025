use std::env::{var, args};
use std::collections::HashSet;
use std::{cmp, include_str};

fn main() {
  let test = args().nth(1).unwrap_or("test".to_string());
  let input = match test.as_str() {
    "test" | "" => include_str!("../../day07/test"), // test och
    "input" => include_str!("../../day07/input"),    // input inkluderad i binary;
    _ => panic!()
  };


  let len = input.lines().next().unwrap().len();
  let mut start = vec![false; len];
  let bp = input.lines().fold(vec!(), |mut arr, l| {
    let mut bps = vec![false; len];
    l.chars().enumerate().for_each(|(j, c)| {
      match c {
        'S' => {start[j] = true},
        '^' => {bps[j] = true},
        '.' => (),
        _ => unreachable!()
      };
    });
    arr.push(bps);
    arr
  });

  println!("{}",
    bp.iter().fold(0, |mut sum, row| {
      let mut temp = vec![false; len];
      sum += start.iter().enumerate().fold(0, |sum, (i, beam)| {
        sum + if row[i] && *beam {
          if let Some(b) = temp.get_mut(i-1) { *b = true;}
          if let Some(b) = temp.get_mut(i+1) { *b = true;}
          1
        } else {
          if *beam { temp[i] = true; }
          0
        }
      });
      start = temp;
      sum
    })
  )
}
