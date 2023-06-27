use crate::is_isomorphic;

// Examples from leetcode

#[test]
fn example_1() {
  let s = "egg".to_owned();
  let t = "add".to_owned();
  assert!(is_isomorphic(&s, &t));
}

#[test]
fn example_2() {
  let s = "foo".to_owned();
  let t = "bar".to_owned();
  assert!(!is_isomorphic(&s, &t));
}

#[test]
fn example_3() {
  let s = "paper".to_owned();
  let t = "title".to_owned();
  assert!(is_isomorphic(&s, &t));
}

// Own tests

#[test]
fn not_equal_lengths() {
  let s = "paper".to_owned();
  let t = "bar".to_owned();
  assert!(!is_isomorphic(&s, &t));
}
