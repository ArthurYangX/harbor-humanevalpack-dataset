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

    Write a function which sorts the given list of integers
    in ascending order according to the sum of their digits.
    Note: if there are several items with similar sum of their digits,
    order them based on their index in original list.
    
*/
fn order_by_points(arr: Vec<i32>) -> Vec<i32> {

    let mut result = arr.clone();
    result.sort_by_key(|&x| (sum_of_digits(x)));
    result
}

pub fn sum_of_digits(n: i32) -> i32 {
    let mut sum = 0;
    let mut n = n;
    if n < 0 {
        let right = n / 10;
        let mut left;

        if right != 0 {
            left = n % 10;
            left = -1 * left;
        } else {
            left = n % 10;
        }
        sum = right + left;
        return sum;
    }

    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
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

    Write a function which sorts the given list of integers
    in ascending order according to the sum of their digits.
    Note: if there are several items with similar sum of their digits,
    order them based on their index in original list.
    
*/
fn order_by_points(arr: Vec<i32>) -> Vec<i32> {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
