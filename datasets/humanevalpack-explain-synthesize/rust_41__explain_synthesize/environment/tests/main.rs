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

    Imagine a road that's a perfectly straight infinitely long line.
    n cars are driving left to right;  simultaneously, a different set of n cars
    are driving right to left.   The two sets of cars start out being very far from
    each other.  All cars move in the same speed.  Two cars are said to collide
    when a car that's moving left to right hits a car that's moving right to left.
    However, the cars are infinitely sturdy and strong; as a result, they continue moving
    in their trajectory as if they did not collide.

    This function outputs the number of such collisions.
    
*/
fn car_race_collision(n:i32)-> i32{return n*n;
}

// Test Code

#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_car_race_collision() {
        assert!(car_race_collision(2) == 4);
        assert!(car_race_collision(3) == 9);
        assert!(car_race_collision(4) == 16);
        assert!(car_race_collision(8) == 64);
        assert!(car_race_collision(10) == 100);
    }

}


// Binary entry point (required for cargo test)
fn main() {}
