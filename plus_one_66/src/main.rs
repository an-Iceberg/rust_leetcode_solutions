fn main() {}

// [Problem](https://leetcode.com/problems/plus-one/)
// #[allow(dead_code)]
fn plus_one(digits: &mut Vec<i32>) {
  todo!()
}

#[cfg(test)]
mod tests {
  use crate::plus_one;

  // Examples from leetcode

  #[test]
  fn example_1() {
    let mut digits: Vec<i32> = vec![1, 2, 3];
    plus_one(&mut digits);
    assert_eq!(digits, vec![1, 2, 4]);
  }

  #[test]
  fn example_2() {
    let mut digits: Vec<i32> = vec![4, 3, 2, 1];
    plus_one(&mut digits);
    assert_eq!(digits, vec![4, 3, 2, 2]);
  }

  #[test]
  fn example_3() {
    let mut digits: Vec<i32> = vec![9];
    plus_one(&mut digits);
    assert_eq!(digits, vec![1, 0]);
  }
}
