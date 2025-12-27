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
Task
    Write a function that takes a string as input and returns the sum of the upper characters only'
    ASCII codes.
    
*/
fn digitSum(s:&str) -> i32{return s.chars().into_iter().filter(|c:&char| c.is_uppercase()).map(|c:char| c as i32).sum();
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digitSum() {
        assert!(digitSum("") == 0);
        assert!(digitSum("abAB") == 131);
        assert!(digitSum("abcCd") == 67);
        assert!(digitSum("helloE") == 69);
        assert!(digitSum("woArBld") == 131);
        assert!(digitSum("aAaaaXa") == 153);
        assert!(digitSum(" How are yOu?") == 151);
        assert!(digitSum("You arE Very Smart") == 327);
    }


}


// Binary entry point (required for cargo test)
fn main() {}
