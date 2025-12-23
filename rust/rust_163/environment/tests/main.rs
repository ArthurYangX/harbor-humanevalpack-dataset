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

    Given two positive integers a and b, return the even digits between a
    and b, in ascending order.
    
*/
fn generate_integers(a: i32, b: i32) -> Vec<i32> {let mut a = a;
    let mut b = b;
    let mut m;

    if b < a {
        m = a;
        a = b;
        b = m;
    }

    let mut out = vec![];
    for i in a..=b {
        if i < 10 && i % 2 == 0 {
            out.push(i);
        }
    }
    out
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_integers() {
        assert_eq!(generate_integers(2, 10), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(10, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(132, 2), vec![2, 4, 6, 8]);
        assert_eq!(generate_integers(17, 89), vec![]);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
