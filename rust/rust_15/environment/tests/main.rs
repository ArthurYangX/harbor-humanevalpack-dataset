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
 Return a string containing space-delimited numbers starting from 0 upto n inclusive.
    
*/
fn string_sequence(n:i32) -> String{let mut res:String = String::new();

    for number in 0..n + 1{
        res = res + &number.to_string() + " ";
    }
    
    return res.trim_end().to_string();

}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_string_sequence() {
        assert!(string_sequence(0) == "0".to_string());
        assert!(string_sequence(3) == "0 1 2 3".to_string());
        assert!(string_sequence(10) == "0 1 2 3 4 5 6 7 8 9 10".to_string());
    }

}


// Binary entry point (required for cargo test)
fn main() {}
