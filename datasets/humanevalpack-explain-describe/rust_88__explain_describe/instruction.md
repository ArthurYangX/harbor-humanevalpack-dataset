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

    In this Kata, you have to sort an array of non-negative integers according to
    number of ones in their binary representation in ascending order.
    For similar number of ones, sort based on decimal value.
    
*/
fn sort_array(array:Vec<i32>) -> Vec<i32>{

    let mut res: Vec<i32> = array.clone();

    if array.len() == 0 {
        return res;
    }

    if (array[0] + array[array.len() - 1]) % 2 == 0 {
        res.sort();
        return res.into_iter().rev().collect();
    } else {
        res.sort();
        return res;
    }
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

    In this Kata, you have to sort an array of non-negative integers according to
    number of ones in their binary representation in ascending order.
    For similar number of ones, sort based on decimal value.
    
*/
fn sort_array(array:Vec<i32>) -> Vec<i32>{



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
