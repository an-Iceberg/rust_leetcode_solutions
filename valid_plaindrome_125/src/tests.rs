// Examples from leetcode

use crate::is_palindrome;

#[test]
fn example_1() {
  let string = "A man, a plan, a canal: Panama".to_owned();
  assert!(is_palindrome(&string));
}

#[test]
fn example_2() {
  let string = "race a car".to_owned();
  assert!(!is_palindrome(&string));
}

#[test]
fn example_3() {
  let string = " ".to_owned();
  assert!(is_palindrome(&string));
}

// Own tests

#[test]
fn example_4() {
  let string = "racecar".to_owned();
  assert!(is_palindrome(&string));
}

#[test]
fn example_5() {
  let string = "suggus".to_owned();
  assert!(is_palindrome(&string));
}

#[test]
fn example_6() {
  let string = "falaffel".to_owned();
  assert!(!is_palindrome(&string));
}

#[test]
fn example_7() {
  let string = "nag a ram".to_owned();
  assert!(!is_palindrome(&string));
}
