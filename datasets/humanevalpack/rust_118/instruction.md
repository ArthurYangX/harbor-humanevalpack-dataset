# Problem Description

Write a Rust function `get_closest_vowel(word: &str) -> String` to solve the following problem:
You are given a word. Your task is to find the closest vowel that stands between
two consonants from the right side of the word (case sensitive).
Vowels in the beginning and ending doesn't count. Return empty string if you didn't
find any vowel met the above condition.
You may assume that the given string contains English letter only.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
You are given a word. Your task is to find the closest vowel that stands between 
    two consonants from the right side of the word (case sensitive).
    
    Vowels in the beginning and ending doesn't count. Return empty string if you didn't
    find any vowel met the above condition. 

    You may assume that the given string contains English letter only.
    
*/
fn get_closest_vowel(word: &str) -> String {



# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
