use std::env::args;

fn main() {
  // parse input file:
  let input = std::fs::read_to_string(
    format!("day02/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  println!("{}",
    input.split(',').fold(0, |mut sum, r| {
        r
        .trim()
        .split("-")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .for_each(|x| {
          for num in x[0].parse::<u64>().unwrap()..=x[1].parse().unwrap() {
            let len = num_len(&num);
            if !len.is_multiple_of(2) { continue; }
            let low = num % (u64::pow(10, len / 2));
            let high = num / (u64::pow(10, len/2));
            if low == high {
              sum += num;
            }
          }
        });
      sum
    })
  );
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
