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
 Given a string, find out how many distinct characters (regardless of case) does it consist of
    
*/
fn count_distinct_characters(str:String) -> i32{let res:HashSet<char> = str.chars().into_iter().map(|x:char| x.to_ascii_lowercase()).collect();
    return res.len() as i32;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_count_distinct_characters() {
        assert!(count_distinct_characters("".to_string()) == 0);
        assert!(count_distinct_characters("abcde".to_string()) == 5);
        assert!(
            count_distinct_characters(
                "abcde".to_string() + &"cade".to_string() + &"CADE".to_string()
            ) == 5
        );
        assert!(count_distinct_characters("aaaaAAAAaaaa".to_string()) == 1);
        assert!(count_distinct_characters("Jerry jERRY JeRRRY".to_string()) == 5);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
