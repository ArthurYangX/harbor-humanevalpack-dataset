# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Write a function that takes a string and returns an ordered version of it.
    Ordered version of string, is a string where all words (separated by space)
    are replaced by a new word where all the characters arranged in
    ascending order based on ascii value.
    Note: You should keep the order of words and blank spaces in the sentence.
    
*/
fn anti_shuffle(s:&str) -> String{

    let mut res: String = String::new();

    for i in s.split_ascii_whitespace() {
        let mut str: Vec<char> = i.chars().into_iter().collect();
        str.sort_by(|a, b| (*a as u32).cmp(&(*b as u32)));
        let str_sorted: String = str.into_iter().collect();
        res.push_str(&str_sorted);
    }
    res = res.trim_end().to_string();
    return res;
}
```

# Instruction

Fix bugs in anti_shuffle.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Write a function that takes a string and returns an ordered version of it.
    Ordered version of string, is a string where all words (separated by space)
    are replaced by a new word where all the characters arranged in
    ascending order based on ascii value.
    Note: You should keep the order of words and blank spaces in the sentence.
    
*/
fn anti_shuffle(s:&str) -> String{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
