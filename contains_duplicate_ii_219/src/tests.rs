// Examples from leetcode

use crate::contains_nearby_duplicate;

#[test]
fn example_1()
{ assert!(contains_nearby_duplicate(&vec![1, 2, 3, 1], 3));
}

#[test]
fn example_2()
{ assert!(contains_nearby_duplicate(&vec![1, 0, 1, 1], 1));
}

#[test]
fn example_3()
{ assert!(!contains_nearby_duplicate(&vec![1, 2, 3, 1, 2, 3], 2));
}

// Own tests
