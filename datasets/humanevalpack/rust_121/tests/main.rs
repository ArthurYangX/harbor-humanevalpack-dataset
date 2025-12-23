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
Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
    
*/
fn solutions(lst: Vec<i32>) -> i32 {let mut sum = 0;
    for (indx, elem) in lst.iter().enumerate() {
        if indx % 2 == 0 {
            if elem % 2 == 1 {
                sum += elem;
            }
        }
    }
    return sum;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(solutions(vec![5, 8, 7, 1]), 12);
        assert_eq!(solutions(vec![3, 3, 3, 3, 3]), 9);
        assert_eq!(solutions(vec![30, 13, 24, 321]), 0);
        assert_eq!(solutions(vec![5, 9]), 5);
        assert_eq!(solutions(vec![2, 4, 8]), 0);
        assert_eq!(solutions(vec![30, 13, 23, 32]), 23);
        assert_eq!(solutions(vec![3, 13, 2, 9]), 3);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
