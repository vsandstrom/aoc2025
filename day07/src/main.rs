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
  let mut start_i = 0;
  let bp = input.lines().fold(vec!(), |mut arr, l| {
    let mut bps = vec![false; len];
    l.chars().enumerate().for_each(|(j, c)| {
      match c {
        'S' => {start[j] = true; start_i = j;},
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
  );

  let mut start = vec![(0, false); len];
  start[start_i]=(1, true);
  
  bp.iter().for_each(|row| {
    let mut temp = vec![(0, false); len];
    start.iter().enumerate().for_each(|(i, (n, beam))| {
      if row[i] && *beam {
        if let Some(val) = temp.get_mut(i-1) { *val = (n + 1, true);}
        if let Some(val) = temp.get_mut(i+1) { *val = (n + 1, true);}
        if n - 1 == 0 {
          temp[i] = (0, false);
        } else {
          temp[i] = (n-1, *beam);
        }
      } else if *beam { temp[i] = (*n, *beam); }
    });
    start = temp;
  });
  println!("{}", start.iter().fold(0, |sum, item| sum + item.0));
}
