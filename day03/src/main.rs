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
      let nums = line // konvertera alla nummer till siffror
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
      let len = nums.len();
      let mut pos: [usize; 12] = [0; 12]; // skapa en array av alla 12 positioner
      pos.iter_mut() // starta med alla positioner i slutet av arrayen.
        .enumerate()
        .for_each(|(i, n)| { *n = i + (len - 12) });
      let mut limit = 0;
      for p in pos.iter_mut() { // för varje position...
        let mut max = nums[*p]; // sätt nuvarande nummer som max
        let mut idx = *p; // spara nuvarande position
        for i in (limit..*p).rev() { // stega baklänges
          if max <= nums[i] { // leta efter större tal än nuvarande. 
            max = nums[i]; // uppdatera ifall ny max funnen
            idx = i; // spara index av nya max
          } 
        }
        *p = idx; // spara slutgiltiga max
        limit = *p+1; // uppdatera avgränsning.
      }
      sum + pos.iter().fold(0u64, |prod, n| {prod*10 + nums[*n] as u64}) // konvertera ental till
                                                                         // resultat
    })
  );
}
