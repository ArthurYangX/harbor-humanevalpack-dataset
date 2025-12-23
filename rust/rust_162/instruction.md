# Problem Description

Write a Rust function `string_to_md5(text: &str) -> String` to solve the following problem:
Given a string 'text', return its md5 hash equivalent string.
If 'text' is an empty string, return None.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a string 'text', return its md5 hash equivalent string.
    If 'text' is an empty string, return None.
    
*/
fn string_to_md5(text: &str) -> String {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
