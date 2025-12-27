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
Create a function which takes a string representing a file's name, and returns
    'Yes' if the the file's name is valid, and returns 'No' otherwise.
    A file's name is considered to be valid if and only if all the following conditions 
    are met:
    - There should not be more than three digits ('0'-'9') in the file's name.
    - The file's name contains exactly one dot '.'
    - The substring before the dot should not be empty, and it starts with a letter from 
    the latin alphapet ('a'-'z' and 'A'-'Z').
    - The substring after the dot should be one of these: ['txt', 'exe', 'dll']
    
*/
fn file_name_check(file_name: &str) -> &str {

    let mut numdigit = 0;
    let mut numdot = 0;
    if file_name.len() < 5 {
        return "No";
    }
    let w = file_name.chars().nth(0).unwrap();
    if w < 'A' || (w > 'Z' && w < 'a') || w > 'z' {
        return "No";
    }
    let last = &file_name[file_name.len() - 4..];
    if last != ".txt" && last != ".exe" && last != ".dll" {
        return "No";
    }
    for c in file_name.chars() {
        if c >= '0' && c <= '9' {
            numdigit += 1;
        }
        if c == '.' {
            numdot += 1;
        }
    }
    if numdigit > 3 || numdot != 1 {
        return "No";
    }
    return "Yes";
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
Create a function which takes a string representing a file's name, and returns
    'Yes' if the the file's name is valid, and returns 'No' otherwise.
    A file's name is considered to be valid if and only if all the following conditions 
    are met:
    - There should not be more than three digits ('0'-'9') in the file's name.
    - The file's name contains exactly one dot '.'
    - The substring before the dot should not be empty, and it starts with a letter from 
    the latin alphapet ('a'-'z' and 'A'-'Z').
    - The substring after the dot should be one of these: ['txt', 'exe', 'dll']
    
*/
fn file_name_check(file_name: &str) -> &str {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
