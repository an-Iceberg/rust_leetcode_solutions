use std::{collections::HashMap, ops::Div};

#[cfg(test)]
mod tests;

fn main() {
  println!("Hello, world!");
}

// [Problem](https://leetcode.com/problems/majority-element/)
#[allow(dead_code)]
fn majority_element(numbers: &Vec<i32>) -> Option<i32> {
  let mut counts = HashMap::with_capacity(numbers.len());

  // Counting each number
  numbers
    .iter()
    .for_each(|x| {
      counts
        .entry(x)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    });

  // Finding a majority element (if it exists)
  for (number, count) in counts.iter() {
    if count
      > &numbers
        .len()
        .div(2)
    {
      return Some(**number);
    }
  }

  return None;
}
