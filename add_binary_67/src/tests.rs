use crate::add_binary;

// Examples from leetcode

#[test]
fn example_1() {
  let (a, b) = ("11".to_owned(), "1".to_owned());
  assert_eq!(add_binary(&a, &b), "100".to_owned());
}

#[test]
fn example_2() {
  let (a, b) = ("1010".to_owned(), "1011".to_owned());
  assert_eq!(add_binary(&a, &b), "10101".to_owned());
}

// Own tests
