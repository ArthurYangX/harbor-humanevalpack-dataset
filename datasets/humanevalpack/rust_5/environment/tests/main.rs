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
 Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
    
*/
fn intersperse(numbers:Vec<u32>, delimeter: u32) -> Vec<u32>{let mut res:Vec<u32> = vec![];
    numbers.iter().for_each(|item:&u32| {res.push(*item); res.push(delimeter);});
    res.pop();
    return res;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_intersperse() {
        assert!(intersperse(vec![], 7) == vec![]);
        assert!(intersperse(vec![5, 6, 3, 2], 8) == vec![5, 8, 6, 8, 3, 8, 2]);
        assert!(intersperse(vec![2, 2, 2], 2) == vec![2, 2, 2, 2, 2]);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
