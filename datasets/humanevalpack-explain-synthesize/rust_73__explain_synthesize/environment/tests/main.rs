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

    Given an array arr of integers, find the minimum number of elements that
    need to be changed to make the array palindromic. A palindromic array is an array that
    is read the same backwards and forwards. In one change, you can change one element to any other element.
    
*/
fn smallest_change(arr:Vec<i32>) -> i32{let mut ans: i32 = 0;
    for i in 0..arr.len() / 2 {
        if arr[i] != arr[arr.len() - i - 1] {
            ans += 1
        }
    }
    return ans;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_change() {
        assert!(smallest_change(vec![1, 2, 3, 5, 4, 7, 9, 6]) == 4);
        assert!(smallest_change(vec![1, 2, 3, 4, 3, 2, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 4, 4, 2]) == 1);
        assert!(smallest_change(vec![1, 2, 3, 2, 1]) == 0);
        assert!(smallest_change(vec![3, 1, 1, 3]) == 0);
        assert!(smallest_change(vec![1]) == 0);
        assert!(smallest_change(vec![0, 1]) == 1);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
