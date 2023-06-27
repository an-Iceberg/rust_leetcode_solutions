use std::collections::HashMap;

fn main() {}

// [Problem](https://leetcode.com/problems/single-number/)
#[allow(dead_code)]
fn single_number(numbers: Vec<i32>) -> Option<i32> {
  let mut counter: HashMap<i32, u32> = HashMap::new();

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

#[cfg(test)]
mod tests {

  // Examples from leetcode

  use crate::single_number;

  #[test]
  fn example_1() {
    let numbers = vec![2, 2, 1];
    assert_eq!(single_number(numbers), Some(1));
  }

  #[test]
  fn example_2() {
    let numbers = vec![4, 1, 2, 1, 2];
    assert_eq!(single_number(numbers), Some(4));
  }

  #[test]
  fn example_3() {
    let numbers = vec![1];
    assert_eq!(single_number(numbers), Some(1));
  }

  // Own tests
  #[test]
  fn no_single_number() {
    let numbers = vec![1, 1, 1, 1, 1, 1];
    assert_eq!(single_number(numbers), None);
  }

  #[test]
  fn two_single_numbers() {
    let numbers = vec![1, 2, 2, 3, 4, 4];
    let result = single_number(numbers);
    assert!(result == Some(1) || result == Some(3));
  }
}
