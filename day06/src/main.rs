use std::env::{var, args};
fn main() {
  let filename = args().nth(1).unwrap_or("test".to_string());
  let input = std::fs::read_to_string(
    format!("{}/{}", var("CARGO_MANIFEST_DIR").unwrap(), &filename))
    .expect("no such input");

  // println!("{input:?}");
  let v = input.split('\n').fold(vec!(), |mut arr, s| {
    let r = s.split(" ").fold(vec!(), |mut row, x| {
        let x = x.trim();
        if !x.is_empty() {
          row.push(x.trim());
        }
        row
      });
    if !r.is_empty() {
      arr.push(r);
    }
    arr
  });

  let result = v[v.len()-1].iter().enumerate().fold(0, |s, (i, x)| {
    let arr = &v[..(v.len()-1)];
    s + match *x {
      "*" => {
        arr.iter().fold(1, |mut prod, row| {
          prod *= row[i].parse::<u64>().unwrap();
          prod
        })
      },
      "+" => {
        arr.iter().fold(0, |sum, row| {
          sum + row[i].parse::<u64>().unwrap()
        })
      },
      _ => unreachable!()
    }
  });
  // println!("{result}");
  let ops = v.last().unwrap();
  let nums = input.lines().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut inner_nums = vec!();
  nums.chunks(nums.len()-1).for_each(|x| {
    // println!("{:?}", x);
    for i in 0..x[0].len() {
      let mut num = vec!();
      for row in x {
        num.push(row[i]);
      }
      // println!("{num:?}");
      inner_nums.push(num);
    }
  });

  // println!("{:?}", inner_nums);
  let mut nums = vec!();
  let mut num = vec!();
  for n in &inner_nums {
    if !n.iter().all(|x| *x == ' ') {
      let n = n
        .iter()
        .filter(|x| **x != ' ')
        .rev()
        .enumerate()
        .fold(0, |sum, (i, x)| {

          sum + (x.to_digit(10).unwrap_or(0) as u64 * u64::pow(10, i as u32))
        });
      num.push(n);
    } else {
      nums.push(num.clone());
      num.clear();
    }
  }

  let nums = &nums[..ops.len()];
  
  println!("{}", ops.iter().enumerate().fold(0, |sum, (i, op)| {
    sum + match *op {
      "*" => { 
        if nums[i].last() == Some(&0) { 
          nums[i][..nums[i].len()-1].iter().product::<u64>()
        } else {
          nums[i].iter().product::<u64>()

        }
      },
      "+" => { 
        nums[i].iter().sum()
      },
      _ => unreachable!()
    }
  }));

  println!("{}", ops.len());
}
