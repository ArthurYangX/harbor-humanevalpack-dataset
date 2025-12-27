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
Return sorted unique common elements for two lists.
    
*/
fn common(l1:Vec<i32>, l2:Vec<i32>) -> Vec<i32>{

let mut res:Vec<i32> = l1.into_iter().filter(|n:&i32| l2.contains(n)).collect();
res.sort();
return res;
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
Return sorted unique common elements for two lists.
    
*/
fn common(l1:Vec<i32>, l2:Vec<i32>) -> Vec<i32>{



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
