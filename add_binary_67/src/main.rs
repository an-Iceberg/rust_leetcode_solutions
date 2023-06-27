#[cfg(test)]
mod tests;

fn main() {}

// [Proble](https://leetcode.com/problems/add-binary/)
#[allow(dead_code)]
fn add_binary(a: &String, b: &String) -> String {
  let a_as_number = binary_string_to_number(a);
  let b_as_number = binary_string_to_number(b);

  return format!("{:b}", a_as_number + b_as_number);
}

fn binary_string_to_number(string: &String) -> u32 {
  return string
    .chars()
    // Converting each character to their binary value
    .map(|letter| {
      letter
        .to_digit(2)
        .unwrap()
    })
    // Concatenating all numbers to one big number
    .fold(0, |acc, e| (2 * acc) + e);
}
