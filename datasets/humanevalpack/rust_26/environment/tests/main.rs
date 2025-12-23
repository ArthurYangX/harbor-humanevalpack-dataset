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
 From a list of integers, remove all elements that occur more than once.
    Keep order of elements left the same as in the input.
    
*/
fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32>{let mut m: HashMap<i32, i32> = HashMap::new();

    for n in &numbers {
        *m.entry(*n).or_default() += 1;
    }
    let res:Vec<i32> = numbers.into_iter().filter(|x| m.get(x) == Some(&1)).collect();
    return res;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_remove_duplicates() {
        assert!(remove_duplicates(vec![]) == []);
        assert!(remove_duplicates(vec![1, 2, 3, 4]) == vec![1, 2, 3, 4]);
        assert!(remove_duplicates(vec![1, 2, 3, 2, 4, 3, 5]) == [1, 4, 5]);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
