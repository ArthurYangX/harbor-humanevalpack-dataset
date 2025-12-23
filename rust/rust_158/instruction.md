# Problem Description

Write a Rust function `find_max(words: Vec<&str>) -> &str` to solve the following problem:
Write a function that accepts a list of strings.
The list contains different words. Return the word with maximum number
of unique characters. If multiple strings have maximum number of unique
characters, return the one which comes first in lexicographical order.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Write a function that accepts a list of strings.
    The list contains different words. Return the word with maximum number
    of unique characters. If multiple strings have maximum number of unique
    characters, return the one which comes first in lexicographical order.
    
*/
fn find_max(words: Vec<&str>) -> &str {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
