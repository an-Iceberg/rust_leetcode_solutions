#[cfg(test)]
mod tests;

fn main() {}

// [Problem](https://leetcode.com/problems/contains-duplicate-ii/)
#[allow(dead_code)]
fn contains_nearby_duplicate(numbers: &Vec<i32>, k: usize) -> bool
{
  for (index, current_number) in numbers.iter().enumerate()
  { // Iterate over a window around the current number with max distance of k
    for (other_index, other_number) in numbers.iter().enumerate()
      .skip((index as i32 - k as i32).max(0) as usize)
      .take(index + k)
    { if index == other_index { continue; }
      if current_number == other_number { return true;}
    }
  }

  return false;
}
