#[cfg(test)]
mod tests;

fn main() {}

// [Problem](https://leetcode.com/problems/happy-number/)
fn is_happy(number: u32) -> bool
{ let mut number = number;
  let mut previous_numbers = vec![];

  loop
  { number = number.to_string()
      .chars()
      .map(|number| number.to_digit(10).unwrap())
      .map(|digit| digit.pow(2))
      .sum::<u32>();

    // TODO: analyse sequence for patterns (4 seems to always show up)

    if number != 1
    { if previous_numbers.contains(&number)
      { return false;
      }
      previous_numbers.push(number);
    } else
    { return true;
    }
  }
}
