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
Return the largest prime factor of n. Assume n > 1 and is not a prime.
    
*/
fn largest_prime_factor(n:i32) -> i32{let mut n1 = n.clone();
    for i in 2.. n1{
        while n1%i == 0 && n1>i{n1 = n1/i;}
    }
    return n1;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert!(largest_prime_factor(15) == 5);
        assert!(largest_prime_factor(27) == 3);
        assert!(largest_prime_factor(63) == 7);
        assert!(largest_prime_factor(330) == 11);
        assert!(largest_prime_factor(13195) == 29);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
