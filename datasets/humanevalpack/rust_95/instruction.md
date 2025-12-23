# Problem Description

Write a Rust function `check_dict_case(dict:HashMap<&str, &str>) -> bool` to solve the following problem:
Given a dictionary, return True if all keys are strings in lower
case or all keys are strings in upper case, else return False.
The function should return False is the given dictionary is empty.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a dictionary, return True if all keys are strings in lower 
    case or all keys are strings in upper case, else return False.
    The function should return False is the given dictionary is empty.
    
*/
fn check_dict_case(dict:HashMap<&str, &str>) -> bool{



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
