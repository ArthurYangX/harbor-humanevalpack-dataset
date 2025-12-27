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
 Filter an input list of strings only for ones that contain given substring
    
*/
fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{

    return strings.iter().filter(|x:&&String| x.contains(&substring)).map(String::from).collect();
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
 Filter an input list of strings only for ones that contain given substring
    
*/
fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
