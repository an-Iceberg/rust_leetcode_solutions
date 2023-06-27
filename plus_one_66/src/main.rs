fn main() {}

// [Problem](https://leetcode.com/problems/plus-one/)
#[allow(dead_code)]
fn plus_one(digits: &mut Vec<i32>) {
  let mut carry = false;

  // Increment last digit (with carry if applicable)
  let last_index = digits.len() - 1;
  digits[last_index] += 1;

  if digits[last_index] > 9 {
    carry = true;
    digits[last_index] = 0;
  } else {
    return;
  }

  // Increment other digits as long as they create carries
  for i in (0..digits.len() - 1).rev() {
    if carry {
      digits[i] += 1;
      carry = false;
    }

    if digits[i] > 9 {
      carry = true;
      digits[i] = 0;
    }

    if !carry {
      break;
    }
  }

  // If first digit creates carry, prepend a 1
  if carry {
    digits.insert(0, 1);
  }
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

  // Own tests

  #[test]
  fn test_for_9999() {
    let mut digits: Vec<i32> = vec![9, 9, 9, 9];
    plus_one(&mut digits);
    assert_eq!(digits, vec![1, 0, 0, 0, 0]);
  }
}
