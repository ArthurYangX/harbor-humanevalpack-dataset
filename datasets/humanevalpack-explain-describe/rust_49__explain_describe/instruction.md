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
Return 2^n modulo p (be aware of numerics).
    
*/
fn modp(n: i32, p: i32) -> i32 {

    if n == 0 {
        return 1;
    } else {
        return (modp(n - 1, p) * 2) % p;
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
Return 2^n modulo p (be aware of numerics).
    
*/
fn modp(n: i32, p: i32) -> i32 {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
