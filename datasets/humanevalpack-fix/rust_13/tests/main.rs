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
 Return a greatest common divisor of two integers a and b
    
*/
fn greatest_common_divisor(mut a:i32,mut b:i32) -> i32{while b > 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_greatest_common_divisor() {
        assert!(greatest_common_divisor(3, 7) == 1);
        assert!(greatest_common_divisor(10, 15) == 5);
        assert!(greatest_common_divisor(49, 14) == 7);
        assert!(greatest_common_divisor(144, 60) == 12);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
