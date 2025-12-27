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
Given a non-empty list of integers lst. add the even elements that are at odd indices..
    
*/
fn add_even_odd(lst: Vec<i32>) -> i32{let mut sum: i32 = 0;

    for (indx, elem) in lst.iter().enumerate() {
        if indx % 2 == 1 {
            if elem % 2 == 0 {
                sum += elem
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
    fn test_add_even_odd() {
        assert!(add_even_odd(vec![4, 88]) == 88);
        assert!(add_even_odd(vec![4, 5, 6, 7, 2, 122]) == 122);
        assert!(add_even_odd(vec![4, 0, 6, 7]) == 0);
        assert!(add_even_odd(vec![4, 4, 6, 8]) == 12);
    }


}


// Binary entry point (required for cargo test)
fn main() {}
