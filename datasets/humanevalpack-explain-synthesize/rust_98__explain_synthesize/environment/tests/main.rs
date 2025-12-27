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

    Given a string s, count the number of uppercase vowels in even indices.
    
*/
fn count_upper(s:&str) -> i32 {let uvowel: &str = "AEIOU";
    let mut count: i32 = 0;

    for (indx, elem) in s.chars().into_iter().enumerate() {
        if indx % 2 == 0 {
            if uvowel.contains(elem) {
                count += 1;
            }
        }
    }
    return count;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_upper() {
        assert!(count_upper("aBCdEf") == 1);
        assert!(count_upper("abcdefg") == 0);
        assert!(count_upper("dBBE") == 0);
        assert!(count_upper("B") == 0);
        assert!(count_upper("U") == 1);
        assert!(count_upper("") == 0);
        assert!(count_upper("EEEE") == 2);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
