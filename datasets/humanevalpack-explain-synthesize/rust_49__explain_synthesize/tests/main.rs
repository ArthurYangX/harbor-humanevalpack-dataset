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
Return 2^n modulo p (be aware of numerics).
    
*/
fn modp(n: i32, p: i32) -> i32 {if n == 0 {
        return 1;
    } else {
        return (modp(n - 1, p) * 2) % p;
    }
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modp() {
        assert!(modp(3, 5) == 3);
        assert!(modp(1101, 101) == 2);
        assert!(modp(0, 101) == 1);
        assert!(modp(3, 11) == 8);
        assert!(modp(100, 101) == 1);
        assert!(modp(30, 5) == 4);
        assert!(modp(31, 5) == 3);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
