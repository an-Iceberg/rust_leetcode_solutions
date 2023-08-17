// Examples from leetcode

use crate::is_valid;

#[test]
fn example_1()
{ let string = "()".to_owned();
  assert!(is_valid(&string));
}

#[test]
fn example_2()
{ let string = "()[]{}".to_owned();
  assert!(is_valid(&string));
}

#[test]
fn example_3()
{ let string = "(]".to_owned();
  assert!(!is_valid(&string));
}

// Own tests

#[test]
fn simple_valid_sequence()
{ let string = "(([[{{([{}])}}]]))".to_owned();
  assert!(is_valid(&string));
}

#[test]
fn complex_valid_sequence()
{ let string = "([{((()))}[(){[()](()[{}])}]])".to_owned();
  assert!(is_valid(&string));
}

#[test]
fn wrong_letter()
{ let string = "(([](){}))t".to_owned();
  assert!(!is_valid(&string));
}

#[test]
fn simple_invalid_sequence_1()
{ let string = "(([[{{([{}])}}]])}".to_owned();
  assert!(!is_valid(&string));
}

#[test]
fn simple_invalid_sequence_2()
{ let string = "(([({{([{}])}}]]))".to_owned();
  assert!(!is_valid(&string));
}

#[test]
fn complex_invalid_sequence()
{ let string = "([{((()))}[(){[()](()[{)])}]])".to_owned();
  assert!(!is_valid(&string));
}
