# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in rust that matches the explanation and passes the unit tests.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 From a given list of integers, generate a list of rolling maximum element found until given moment
    in the sequence.
    
*/
fn rolling_max(numbers:Vec<i32>) -> Vec<i32>{


# Instructions

Please write your solution in the file `solution/solution.rs`.
Ensure your code is self-contained and runs correctly.
