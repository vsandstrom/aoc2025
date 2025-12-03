use std::env::args;

fn main() {
  // parse input file:
  let input = std::fs::read_to_string(
    format!("day02/{}", args().nth(1).unwrap_or("test".to_string()))).expect("no such input");

  println!("{}",
    input
    .split(',') // splitta varje range
    .fold(0, |mut sum, range| { // loopa och accumulate varje range
      range
      .trim() // trimma whitespace ifall det finns
      .split("-") // splitta de två värdena
      .map(|s| s.parse::<u64>().unwrap()) // konvertera till STOR int
      .collect::<Vec<u64>>() // samla alla (2) värdena
      .chunks(2) // bunta ihop 2 och 2
      .for_each(|x| { // iterera över paren
        for num in x[0]..=x[1] { // för varje nummer i rangen
          let len  = num_len(&num); // nummerlängden
          let low  = num % (u64::pow(10, len / 2)); // extrahera botten
          let high = num / (u64::pow(10, len / 2)); // extrahera toppen
          if low == high { // jämför
            sum += num;
          }
        }
      });
      sum
    })
  );


  println!("{}",
    input.split(",")
      .fold(0u64, |mut sum, range| {
      range
        .trim() // trimma whitespace ifall det finns
        .split("-") // splitta de två värdena
        .map(|s| s.parse::<u64>().unwrap()) // konvertera till STOR int
        .collect::<Vec<u64>>() // samla alla (2) värdena
        .chunks(2) // bunta ihop 2 och 2
        .for_each(|x| {
          'outer: for num in x[0]..=x[1] { // för varje nummer i rangen
            let len  = num_len(&num); // nummerlängden
            'inner: for i in 2..=len {
              let mut temp = num; // temp kopia
              let mut curr = vec!(); // comparison vector
              while temp > 0 {
                let n  = temp % u64::pow(10, len / i); // extrahera tal
                if num_len(&n) != len/i { continue 'inner; } // filtrera 'leading zeroes'
                curr.push(n); // lägg till i vector för comparison
                temp /= u64::pow(10, len / i); // decrement temp
              }
              if curr.len() >= 2 && curr[1..].iter().all(|c| *c == curr[0]) { 
                // if at least 2 och alla är samma siffra
                sum += num;
                continue 'outer;
              } 
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
