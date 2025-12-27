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

    prime_fib returns n-th number that is a Fibonacci number and it's also prime.
    
*/
fn prime_fib(n: i32) -> i32 {

    let mut f1 = 1;
    let mut f2 = 2;
    let mut count = 0;
    while count < n {
        f1 = f1 + f2;
        let m = f1;
        f1 = f2;
        f2 = m;
        let mut isprime = true;
        for w in 2..(f1 as f32).sqrt() as i32 + 1 {
            if f1 % w == 0 {
                isprime = false;
                break;
            }
        }
        if isprime {
            count += 1;
        }
        if count == n {
            return f1;
        }
    }
    0
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

    prime_fib returns n-th number that is a Fibonacci number and it's also prime.
    
*/
fn prime_fib(n: i32) -> i32 {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
