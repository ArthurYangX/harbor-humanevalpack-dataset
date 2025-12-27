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
Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.
    
*/
fn change_base(x:i32, base:i32) -> String{



# Instructions

Please write your solution in the file `solution/solution.rs`.
Ensure your code is self-contained and runs correctly.
