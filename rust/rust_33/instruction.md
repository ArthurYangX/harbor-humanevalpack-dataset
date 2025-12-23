# Problem Description

Write a Rust function `sort_third(l: Vec<i32>) -> Vec<i32>` to solve the following problem:
This function takes a list l and returns a list l' such that
l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
to the values of the corresponding indicies of l, but sorted.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
This function takes a list l and returns a list l' such that
    l' is identical to l in the indicies that are not divisible by three, while its values at the indicies that are divisible by three are equal
    to the values of the corresponding indicies of l, but sorted.
    
*/
fn sort_third(l: Vec<i32>) -> Vec<i32> {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
