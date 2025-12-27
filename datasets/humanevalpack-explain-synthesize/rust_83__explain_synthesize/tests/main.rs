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

    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    
*/
fn starts_one_ends(n:i32) -> i32{if n == 1 {
        return 1;
    };
    return 18 * i32::pow(10, (n - 2) as u32);
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_one_ends() {
        assert!(starts_one_ends(1) == 1);
        assert!(starts_one_ends(2) == 18);
        assert!(starts_one_ends(3) == 180);
        assert!(starts_one_ends(4) == 1800);
        assert!(starts_one_ends(5) == 18000);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
