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

    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
*/
fn words_string(s:&str) -> Vec<String> {

    return s
        .to_string()
        .split(|c: char| c == ',' || c.is_whitespace())
        .into_iter()
        .filter(|x| x != &"")
        .map(|x| x.to_string())
        .collect();
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

    You will be given a string of words separated by commas or spaces. Your task is
    to split the string into words and return an array of the words.
    
*/
fn words_string(s:&str) -> Vec<String> {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
