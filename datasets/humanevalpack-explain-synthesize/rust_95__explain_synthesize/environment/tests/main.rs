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

    Given a dictionary, return True if all keys are strings in lower 
    case or all keys are strings in upper case, else return False.
    The function should return False is the given dictionary is empty.
    
*/
fn check_dict_case(dict:HashMap<&str, &str>) -> bool{if dict.is_empty() {
        return false;
    }
    let string_lower: fn(str: &str) -> bool = |str: &str| {
        return str.chars().into_iter().all(|c| c.is_ascii_lowercase());
    };
    let string_upper: fn(str: &str) -> bool = |str: &str| {
        return str.chars().into_iter().all(|c| c.is_ascii_uppercase());
    };

    let lower: bool = dict.keys().into_iter().all(|str| string_lower(str));
    let upper: bool = dict.keys().into_iter().all(|str| string_upper(str));
    return lower || upper;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dict_case() {
        assert!(check_dict_case(HashMap::from([("p", "pineapple"), ("b", "banana")])) == true);
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("A", "banana"),
                ("B", "banana")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("p", "pineapple"),
                ("5", "banana"),
                ("a", "apple")
            ])) == false
        );
        assert!(
            check_dict_case(HashMap::from([
                ("Name", "John"),
                ("Age", "36"),
                ("City", "Houston")
            ])) == false
        );
        assert!(check_dict_case(HashMap::from([("STATE", "NC"), ("ZIP", "12345")])) == true);
        assert!(check_dict_case(HashMap::from([("fruit", "Orange"), ("taste", "Sweet")])) == true);
        assert!(check_dict_case(HashMap::new()) == false);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
