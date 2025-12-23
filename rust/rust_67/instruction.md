# Problem Description

Write a Rust function `fruit_distribution(s:&str, n:i32) -> i32` to solve the following problem:
In this task, you will be given a string that represents a number of apples and oranges
that are distributed in a basket of fruit this basket contains
apples, oranges, and mango fruits. Given the string that represents the total number of
the oranges and apples and an integer that represent the total number of the fruits
in the basket return the number of the mango fruits in the basket.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    In this task, you will be given a string that represents a number of apples and oranges 
    that are distributed in a basket of fruit this basket contains 
    apples, oranges, and mango fruits. Given the string that represents the total number of 
    the oranges and apples and an integer that represent the total number of the fruits 
    in the basket return the number of the mango fruits in the basket.
    
*/
fn fruit_distribution(s:&str, n:i32) -> i32 {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
