// Examples from leetcode

use crate::contains_duplicate;

#[test]
fn example_1() {
  let numbers = vec![1, 2, 3, 1];
  assert!(contains_duplicate(&numbers));
}

#[test]
fn example_2() {
  let numbers = vec![1, 2, 3, 4];
  assert!(!contains_duplicate(&numbers));
}

#[test]
fn example_3() {
  let numbers = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
  assert!(contains_duplicate(&numbers));
}

// Own tests
