// Tests from leetcode

use crate::generate;

#[test]
fn example_1()
{ assert_eq!(generate(5), vec![
    vec![1],
    vec![1,1],
    vec![1,2,1],
    vec![1,3,3,1],
    vec![1,4,6,4,1]
  ]);
}

#[test]
fn example_2()
{ assert_eq!(generate(1), vec![
    vec![1]
  ]);
}

// Own tests

#[test]
fn size_2()
{ assert_eq!(generate(2), vec![
    vec![1],
    vec![1,1],
  ]);
}

#[test]
fn size_3()
{ assert_eq!(generate(3), vec![
    vec![1],
    vec![1,1],
    vec![1,2,1],
  ]);
}

#[test]
fn size_4()
{ assert_eq!(generate(4), vec![
    vec![1],
    vec![1,1],
    vec![1,2,1],
    vec![1,3,3,1],
  ]);
}

#[test]
fn size_7()
{ assert_eq!(generate(7), vec![
    vec![1],
    vec![1,1],
    vec![1,2,1],
    vec![1,3,3,1],
    vec![1,4,6,4,1],
    vec![1,5,10,10,5,1],
    vec![1,6,15,20,15,6,1]
  ]);
}
