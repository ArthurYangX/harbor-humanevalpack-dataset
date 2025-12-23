#![allow(unused)]
use std::collections::*;
use std::cmp::*;
use std::io::*;
use std::str::*;
use std::any::Any;
use std::mem::replace;
use rand::prelude::*;
use regex::Regex;
use md5;


// Solution Code
/*
Return only positive numbers in the list.
    
*/
fn get_positive(numbers:Vec<i32>) -> Vec<i32>{return numbers.into_iter().filter(|n| n.is_positive()).collect();

}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
    fn test_get_positive() {
        assert!(get_positive(vec![-1, -2, 4, 5, 6]) == [4, 5, 6]);
        assert!(
            get_positive(vec![5, 3, -5, 2, 3, 3, 9, 0, 123, 1, -10]) == [5, 3, 2, 3, 3, 9, 123, 1]
        );
        assert!(get_positive(vec![-1, -2]) == []);
        assert!(get_positive(vec![]) == []);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
