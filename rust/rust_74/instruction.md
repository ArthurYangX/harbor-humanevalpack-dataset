# Problem Description

Write a Rust function `total_match(lst1:Vec<&str>, lst2:Vec<&str>) -> Vec<String>` to solve the following problem:
Write a function that accepts two lists of strings and returns the list that has
total number of chars in the all strings of the list less than the other list.
if the two lists have the same number of chars, return the first list.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Write a function that accepts two lists of strings and returns the list that has 
    total number of chars in the all strings of the list less than the other list.

    if the two lists have the same number of chars, return the first list.
    
*/
fn total_match(lst1:Vec<&str>, lst2:Vec<&str>) -> Vec<String>{



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
