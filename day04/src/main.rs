use std::env::{var, args};

fn main() {
  let filename = args().nth(1).unwrap_or("test".to_string());
  let input = std::fs::read_to_string(
    format!("{}/{}", var("CARGO_MANIFEST_DIR").unwrap(), &filename)).expect("no such input");

  let data = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();


  let mut moveable = 0;
  for i in 0..data.len() {
    let mut rows = vec!();
    if i as isize > 0 {
      rows.push(data.get(i-1))
    } else { rows.push(None) }
    rows.push(data.get(i));
    rows.push(data.get(i+1));
    for j in 0..data[0].len() {
      let mut n = vec!();
      for (k, row) in rows.iter().enumerate() {
        if let Some(row) = row {
          if k != 1 { n.push(row.get(j)) }
          if j as isize > 0 { n.push(row.get(j-1)) }
          n.push(row.get(j+1));
        }
      }
      if n.iter().filter(|x| x == &&Some(&'@')).count().lt(&4) {
        moveable += 1;
      }
    }
  }

  println!("{}", moveable);
  //   data.windows(3).fold(0,|sum1, win| {
  //   win.windows(3).fold(0, |mut sum2: i32, dow| {
  //     for i in 0..(dow[0].len()-1) {
  //
  //
  //
  //       sum2 += if dow[1][1+i] == '@' && 
  //                  [dow[0][i], dow[0][1+i], dow[0][2+i],
  //                   dow[1][i]             , dow[1][2+i],
  //                   dow[2][i], dow[2][1+i], dow[2][2+i]]
  //       .iter()
  //       .fold(0, |sum, c| { sum + if * c == '@' {1} else {0}}) < 4 {1} else {0}
  //     }
  //     sum2
  //   }) + sum1
  // })
  // );
}
