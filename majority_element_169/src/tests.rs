use crate::majority_element;

// Examples from leetcode

#[test]
fn example_1() {
  let numbers = vec![3, 2, 3];
  assert_eq!(majority_element(&numbers), Some(3));
}

#[test]
fn example_2() {
  let numbers = vec![2, 2, 1, 1, 1, 2, 2];
  assert_eq!(majority_element(&numbers), Some(2));
}

// Own tests

#[test]
fn no_majority_element() {
  let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
  assert_eq!(majority_element(&numbers), None);
}

#[test]
fn odd_number_count() {
  let numbers = vec![3, 3, 4, 5, 6];
  assert_eq!(majority_element(&numbers), None);
}
