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
Add two numbers x and y
*/
fn add(x:i32, y:i32) -> i32{return x + y;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert!(add(0, 1) == 1);
        assert!(add(1, 0) == 1);
        assert!(add(2, 3) == 5);
        assert!(add(5, 7) == 12);
        assert!(add(7, 5) == 12);
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let mut x: i32 = rng.random();
            x = x % 1000;
            let mut y: i32 = rng.random();
            y = y % 1000;

            assert!(add(x, y) == x + y);
        }
    }

}


// Binary entry point (required for cargo test)
fn main() {}
