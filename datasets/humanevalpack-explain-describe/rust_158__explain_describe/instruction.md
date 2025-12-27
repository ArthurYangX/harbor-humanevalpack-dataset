# Context

You are given a reference implementation (canonical solution) to explain.

```rust
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

    let mut max = "";
    let mut maxu = 0;
    for i in 0..words.len() {
        let mut unique = String::from("");
        for j in 0..words[i].len() {
            if !unique.contains(words[i].chars().nth(j).unwrap()) {
                unique.push(words[i].chars().nth(j).unwrap());
            }
        }
        if unique.len() > maxu || (unique.len() == maxu && words[i] < max) {
            max = words[i];
            maxu = unique.len();
        }
    }
    max
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

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

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
