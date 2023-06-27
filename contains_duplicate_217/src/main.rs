#[cfg(test)]
mod tests;

use std::collections::HashMap;

fn main() {}

// [Problem](https://leetcode.com/problems/contains-duplicate/)
#[allow(dead_code)]
fn contains_duplicate(numbers: &Vec<i32>) -> bool {
  let mut counter = HashMap::with_capacity(numbers.len());

  numbers
    .iter()
    .for_each(|x| {
      counter
        .entry(*x)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    });

  for (_, count) in counter.iter() {
    if *count != 1 {
      return true;
    }
  }

  return false;
}
