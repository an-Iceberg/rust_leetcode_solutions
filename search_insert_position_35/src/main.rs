#[cfg(test)]
mod tests;

fn main() {
  println!("Should: 1");
  println!("{}", search_insert(&vec![1, 3, 5, 6], 2));
}

// [Problem](https://leetcode.com/problems/search-insert-position/)
#[allow(dead_code)]
fn search_insert(numbers: &Vec<i32>, target: i32) -> i32 {
  for (index, number) in numbers
    .iter()
    .enumerate()
  {
    if *number >= target {
      return index as i32;
    }
  }

  return numbers.len() as i32;
}
