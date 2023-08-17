// leetcode tests

use crate::search_insert;

#[test]
fn example_1() {
  let numbers = vec![1, 3, 5, 6];
  let target = 5;
  assert_eq!(search_insert(&numbers, target), 2);
}

#[test]
fn example_2() {
  let numbers = vec![1, 3, 5, 6];
  let target = 2;
  assert_eq!(search_insert(&numbers, target), 1);
}

#[test]
fn example_3() {
  let numbers = vec![1, 3, 5, 6];
  let target = 7;
  assert_eq!(search_insert(&numbers, target), 4);
}

// Own tests
