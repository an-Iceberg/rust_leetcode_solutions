fn main() {}

// [Problem](https://leetcode.com/problems/two-sum/)
#[allow(dead_code)]
fn two_sum(numbers: Vec<i32>, target: i32) -> Option<(usize, usize)> {
  for (first_index, _) in numbers
    .iter()
    .enumerate()
  {
    for (second_index, _) in numbers
      .iter()
      .enumerate()
      .skip(first_index + 1)
    {
      if numbers[first_index] + numbers[second_index] == target {
        return Some((first_index, second_index));
      }
    }
  }

  return None;
}

#[cfg(test)]
mod tests {
  use crate::two_sum;

  // Examples from leetcode
  #[test]
  fn example_1() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(two_sum(numbers, target), Some((0, 1)));
  }

  #[test]
  fn example_2() {
    let numbers = vec![3, 2, 4];
    let target = 6;
    assert_eq!(two_sum(numbers, target), Some((1, 2)));
  }

  #[test]
  fn example_3() {
    let numbers = vec![3, 3];
    let target = 6;
    assert_eq!(two_sum(numbers, target), Some((0, 1)));
  }

  // Additional tests
  #[test]
  fn no_match() {
    let numbers = vec![1, 2];
    let target = 6;
    assert_eq!(two_sum(numbers, target), None);
  }

  #[test]
  fn no_match_2() {
    let numbers = vec![2, 4, 6, 8, 10, 12];
    let target = 17;
    assert_eq!(two_sum(numbers, target), None);
  }

  #[test]
  fn match_1() {
    let numbers = vec![2, 5, 6, 7, 11, 12];
    let target = 23;
    assert_eq!(two_sum(numbers, target), Some((4, 5)));
  }

  #[test]
  fn unsorted_list_1() {
    let numbers = vec![12, 6, 7, 5, 11, 2];
    let target = 23;
    assert_eq!(two_sum(numbers, target), Some((0, 4)));
  }
}
