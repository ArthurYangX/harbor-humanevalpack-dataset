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
Return True if all numbers in the list l are below threshold t.
    
*/
fn below_threshold(l: Vec<i32>, t: i32) -> bool {for i in l {
        if i >= t {
            return false;
        }
    }
    return true;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_threshold() {
        assert!(below_threshold(vec![1, 2, 4, 10], 100));
        assert!(!below_threshold(vec![1, 20, 4, 10], 5));
        assert!(below_threshold(vec![1, 20, 4, 10], 21));
        assert!(below_threshold(vec![1, 20, 4, 10], 22));
        assert!(below_threshold(vec![1, 8, 4, 10], 11));
        assert!(!below_threshold(vec![1, 8, 4, 10], 10));
    }

}


// Binary entry point (required for cargo test)
fn main() {}
