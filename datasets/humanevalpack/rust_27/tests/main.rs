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
 For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
    
*/
pub fn flip_case(string: String) -> String{return string.chars().into_iter().fold(String::new(), |res:String, c:char| {if c.is_ascii_lowercase(){return res + &c.to_uppercase().to_string();}else{return res + &c.to_ascii_lowercase().to_string();}});
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_flip_case() {
        assert!(flip_case("".to_string()) == "".to_string());
        assert!(flip_case("Hello!".to_string()) == "hELLO!".to_string());
        assert!(
            flip_case("These violent delights have violent ends".to_string())
                == "tHESE VIOLENT DELIGHTS HAVE VIOLENT ENDS".to_string()
        );
    }

}


// Binary entry point (required for cargo test)
fn main() {}
