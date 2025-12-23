# Problem Description

Write a Rust function `get_matrix_triples(n: i32) -> i32` to solve the following problem:
You are given a positive integer n. You have to create an integer array a of length n.
For each i (1 ≤ i ≤ n), the value of a[i] = i * i - i + 1.
Return the number of triples (a[i], a[j], a[k]) of a where i < j < k,
and a[i] + a[j] + a[k] is a multiple of 3.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    You are given a positive integer n. You have to create an integer array a of length n.
        For each i (1 ≤ i ≤ n), the value of a[i] = i * i - i + 1.
        Return the number of triples (a[i], a[j], a[k]) of a where i < j < k, 
    and a[i] + a[j] + a[k] is a multiple of 3.
    
*/
fn get_matrix_triples(n: i32) -> i32 {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
