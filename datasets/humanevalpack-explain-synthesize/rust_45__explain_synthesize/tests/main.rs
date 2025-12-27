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

    Given the lengths of the three sides of a triangle. Return the area of
    the triangle rounded to 2 decimal points if the three sides form a valid triangle. 
    Otherwise return -1
    Three sides make a valid triangle when the sum of any two sides is greater 
    than the third side.
    
*/
fn triangle_area(a:i32, h:i32) -> f64{return (a * h) as f64 / 2.0;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        assert!(triangle_area(5, 3) == 7.5);
        assert!(triangle_area(2, 2) == 2.0);
        assert!(triangle_area(10, 8) == 40.0);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
