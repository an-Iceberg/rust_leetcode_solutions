#[cfg(test)]
mod tests;

use std::collections::HashMap;

fn main() {}

// [Problem](https://leetcode.com/problems/single-number/)
#[allow(dead_code)]
fn single_number(numbers: &Vec<i32>) -> Option<i32> {
  let mut counter: HashMap<i32, u32> = HashMap::with_capacity(numbers.len());

  numbers
    .iter()
    .for_each(|x| {
      counter
        .entry(*x)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    });

  for (number, count) in counter.iter() {
    if *count == 1 {
      return Some(*number);
    }
  }

  return None;
}
