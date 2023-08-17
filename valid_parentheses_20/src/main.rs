#[cfg(test)]
mod tests;

fn main() {}

// [Problem](https://leetcode.com/problems/valid-parentheses/)
fn is_valid(string: &String) -> bool
{ let mut stack = vec![];

  for letter in string.chars()
  { match letter
    { '(' | '[' | '{' => stack.push(letter),
      ')' =>
      { if stack.pop() != Some('(')
        { return false;
        }
      }
      ']' =>
      { if stack.pop() != Some('[')
        { return false;
        }
      }
      '}' =>
      { if stack.pop() != Some('{')
        { return false;
        }
      }
      _ => return false
    }
  }

  return true;
}
