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
Implement a function that takes an non-negative integer and returns an array of the first n
    integers that are prime numbers and less than n.
    
*/
fn count_up_to(n:i32) -> Vec<i32> {

    let mut primes: Vec<i32> = vec![];

    for i in 2..n {
        let mut is_prime: bool = true;

        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    return primes;
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
Implement a function that takes an non-negative integer and returns an array of the first n
    integers that are prime numbers and less than n.
    
*/
fn count_up_to(n:i32) -> Vec<i32> {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
