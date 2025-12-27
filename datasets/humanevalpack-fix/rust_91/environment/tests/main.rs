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

    You'll be given a string of words, and your task is to count the number
    of boredoms. A boredom is a sentence that starts with the word "I".
    Sentences are delimited by '.', '?' or '!'.
    
*/
fn is_bored(s:&str) -> i32 {let mut count = 0;
    let regex = Regex::new(r"[.?!]\s*").expect("Invalid regex");
    let sqn: Vec<&str> = regex.split(s).into_iter().collect();
    for s in sqn {
        if s.starts_with("I ") {
            count += 1;
        }
    }
    return count;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bored() {
        assert!(is_bored("Hello world") == 0);
        assert!(is_bored("Is the sky blue?") == 0);
        assert!(is_bored("I love It !") == 1);
        assert!(is_bored("bIt") == 0);
        assert!(is_bored("I feel good today. I will be productive. will kill It") == 2);
        assert!(is_bored("You and I are going for a walk") == 0);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
