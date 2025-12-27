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

    Create a function that takes a string as input which contains only square brackets.
    The function should return True if and only if there is a valid subsequence of brackets 
    where at least one bracket in the subsequence is nested.
    
*/
fn is_nested(str: &str) -> bool {let mut count = 0;
    let mut maxcount = 0;
    for i in 0..str.len() {
        if str.chars().nth(i).unwrap() == '[' {
            count += 1;
        }
        if str.chars().nth(i).unwrap() == ']' {
            count -= 1;
        }
        if count < 0 {
            count = 0;
        }
        if count > maxcount {
            maxcount = count;
        }
        if count <= maxcount - 2 {
            return true;
        }
    }
    return false;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nested() {
        assert_eq!(is_nested("[[]]"), true);
        assert_eq!(is_nested("[]]]]]]][[[[[]"), false);
        assert_eq!(is_nested("[][]"), false);
        assert_eq!(is_nested("[]"), false);
        assert_eq!(is_nested("[[[[]]]]"), true);
        assert_eq!(is_nested("[]]]]]]]]]]"), false);
        assert_eq!(is_nested("[][][[]]"), true);
        assert_eq!(is_nested("[[]"), false);
        assert_eq!(is_nested("[]]"), false);
        assert_eq!(is_nested("[[]][["), true);
        assert_eq!(is_nested("[[][]]"), true);
        assert_eq!(is_nested(""), false);
        assert_eq!(is_nested("[[[[[[[["), false);
        assert_eq!(is_nested("]]]]]]]]"), false);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
