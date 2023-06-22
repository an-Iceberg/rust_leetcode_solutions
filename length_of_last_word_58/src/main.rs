fn main() {
  println!("Hello, world!");
  println!("How are you doing tonight? Not fine, thank you.");
}

// [Problem](https://leetcode.com/problems/length-of-last-word/)
#[allow(dead_code)]
fn length_of_last_word(string: String) -> usize {
  return string
    .trim()
    .split(" ")
    .last()
    .unwrap_or("")
    .len();
}

#[cfg(test)]
mod tests {
  use crate::length_of_last_word;

  // Examples from leetcode

  #[test]
  fn example_1() {
    let string = "Hello World".to_string();
    assert_eq!(length_of_last_word(string), 5);
  }

  #[test]
  fn example_2() {
    let string = "   fly me   to   the moon  ".to_string();
    assert_eq!(length_of_last_word(string), 4);
  }

  #[test]
  fn example_3() {
    let string = "luffy is still joyboy".to_string();
    assert_eq!(length_of_last_word(string), 6);
  }

  // Own tests

  #[test]
  fn empty_string_test() {
    let string = "".to_string();
    assert_eq!(length_of_last_word(string), 0);
  }

  #[test]
  fn special_characters() {
    let string = "+*\" ?=) 0197485 %&/\\^".to_string();
    assert_eq!(length_of_last_word(string), 5);
  }
}
