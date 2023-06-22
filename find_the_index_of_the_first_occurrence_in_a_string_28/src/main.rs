fn main() {}

// [Problem](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/)
#[allow(dead_code)]
fn str_str(haystack: String, needle: String) -> Option<usize> {
  return haystack.find(&needle);
}

#[cfg(test)]
mod tests {
  use crate::str_str;

  // Examples from leetcode
  #[test]
  fn example_1() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();
    assert_eq!(str_str(haystack, needle), Some(0));
  }

  #[test]
  fn example_2() {
    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();
    assert_eq!(str_str(haystack, needle), None);
  }

  // Own tests
  #[test]
  fn needle_too_large() {
    let haystack = "OT".to_string();
    let needle = "One Topic at a Time".to_string();
    assert_eq!(str_str(haystack, needle), None);
  }
}
