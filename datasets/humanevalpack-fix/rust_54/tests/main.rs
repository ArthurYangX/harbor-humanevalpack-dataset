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

    Check if two words have the same characters.
    
*/
fn same_chars(str1:&str, str2:&str) -> bool{let mut v1:Vec<char> = str1.chars().into_iter().collect();
    v1.sort();
    v1.dedup();

    let mut v2:Vec<char> = str2.chars().into_iter().collect();
    v2.sort();
    v2.dedup();

    return v1 == v2;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_same_chars() {
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddeddabc") == true);
        assert!(same_chars("abcd", "dddddddabc") == true);
        assert!(same_chars("dddddddabc", "abcd") == true);
        assert!(same_chars("eabcd", "dddddddabc") == false);
        assert!(same_chars("abcd", "dddddddabcf") == false);
        assert!(same_chars("eabcdzzzz", "dddzzzzzzzddddabc") == false);
        assert!(same_chars("aabb", "aaccc") == false);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
