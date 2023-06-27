// Examples from leetcode

use crate::hamming_weight;

#[test]
fn example_1() {
  let number: u32 = 11;
  assert_eq!(hamming_weight(&number), 3);
}

#[test]
fn example_2() {
  let number: u32 = 128;
  assert_eq!(hamming_weight(&number), 1);
}

#[test]
fn example_3() {
  let number: u32 = 4_294_967_293;
  assert_eq!(hamming_weight(&number), 31);
}

// Own tests
