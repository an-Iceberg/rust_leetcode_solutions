#[cfg(test)]
mod tests;

fn main() {}

// [Problem](https://leetcode.com/problems/valid-palindrome/)
#[allow(dead_code)]
fn is_palindrome(string: &String) -> bool {
  let valid_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_owned();

  let mut plain_string = string.clone();

  plain_string.retain(|c| valid_chars.contains(c));
  plain_string = plain_string.to_lowercase();

  for (a, b) in plain_string
    .chars()
    .take(plain_string.len() / 2)
    .zip(
      plain_string
        .chars()
        .rev()
        .take(plain_string.len() / 2),
    )
  {
    if a != b {
      return false;
    }
  }

  return true;
}
