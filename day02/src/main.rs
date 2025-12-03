use std::env::args;

fn main() {
  // parse input file:
  let input = std::fs::read_to_string(
    format!("day02/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  // Day2 part 1:
  let mut invalid_sum = 0;
  input.split(',').for_each(|l| {
    let [ref a, ref b] = l
      .trim()
      .split("-")
      .map(|s| s.to_string())
      .collect::<Vec<String>>()[..] else { println!("{l}"); panic!() };

    let (low, high) = (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap());

    'inner: for n in low..=high {
      let len = num_len(&n);
      if !len.is_multiple_of(2) { continue 'inner; }
      let low = n % (u64::pow(10, len / 2));
      let high = n / (u64::pow(10, len/2));
      if low == high {
        invalid_sum += n;
      }
    }
  });
  println!("{invalid_sum}");
}

#[inline(always)]
fn num_len(num: &u64) -> u32 {
  let mut num = *num;
  let mut i = 0;
  while num != 0 {
    i += 1;
    num /= 10;
  }
  i
}

#[inline(always)]
fn dub_num(num: &u64) -> u64 {
  let len = num_len(num);
  num + num * (u64::pow(10, len))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test] 
  fn half() {
    let x = 1234;
    let n = num_len(&x);
    assert_eq!(34, x % (u64::pow(10, n / 2)));
  }

  #[test] 
  fn double() {
    let x = 34;
    assert_eq!(3434, dub_num(&x));
  }
}
