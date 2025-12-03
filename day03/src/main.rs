use std::env::args;

fn main() {
  let filename = args().nth(1).unwrap_or("test".to_string());
  let input = std::fs::read_to_string(
    format!("day03/{}", &filename)).expect("no such input");

  println!("{}", 
    input.lines().fold(0, |sum, line| {
      let nums = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
      let mut tiotal = 0;
      let mut max = 0;
      for (i, n) in nums.iter().enumerate() {
        if *n < tiotal { continue; }
        let mut ental = 0;
        for m in &nums[(i+1)..] {
          if n*10 + m > max {
            if *m < ental { continue; }
            max = n*10 + m;
            tiotal = *n;
            ental = *m;
          }

        }
      };
      sum + max
    })
  );

  println!("{}", 
    input.lines().fold(0u64, |sum, line| {
      let nums = line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
      let len = nums.len();
      let mut pos = if filename == "test" {
        let pos: [usize; 12] = std::array::from_fn(|i| i+(15-12));
        pos
      } else {
        let pos: [usize; 12] = std::array::from_fn(|i| i+(100-12));
        pos
      };
      let mut limit = 0;
      for p in pos.iter_mut() {
        let mut max = nums[*p];
        let mut idx = *p;
        for i in (limit..*p).rev() {
          if max <= nums[i] {
            max = nums[i];
            idx = i;
          } 
        }
        *p = idx;
        limit = *p+1;
      }
      sum + pos.iter().fold(0u64, |prod, n| {prod*10 + nums[*n] as u64})
    })
  );
}
