#[cfg(test)]
mod tests;

fn main() {
  println!("Hello, world!");
}

// [Problem](https://leetcode.com/problems/number-of-1-bits/)
#[allow(dead_code)]
fn hamming_weight(number: &u32) -> u32 {
  return number.count_ones();
}
