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

    Write a function that takes an integer a and returns True 
    if this ingeger is a cube of some integer number.
    Note: you may assume the input is always valid.
    
*/
fn iscube(a:i32) -> bool{let a1: f64 = i32::abs(a) as f64;
    let sqrt_3 = f64::powf(a1, 1.0 / 3.0).ceil();

    return i32::pow(sqrt_3 as i32, 3) == a1 as i32;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iscube() {
        assert!(iscube(1) == true);
        assert!(iscube(2) == false);
        assert!(iscube(-1) == true);
        assert!(iscube(64) == true);
        assert!(iscube(180) == false);
        assert!(iscube(1000) == true);
        assert!(iscube(0) == true);
        assert!(iscube(1729) == false);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
