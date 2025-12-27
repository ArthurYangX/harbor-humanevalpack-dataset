# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
    
*/
fn solutions(lst: Vec<i32>) -> i32 {

    let mut sum = 0;
    for (indx, elem) in lst.iter().enumerate() {
        if indx % 2 == 1 {
            if elem % 2 == 1 {
                sum += elem;
            }
        }
    }
    return sum;
}
```

# Instruction

Fix bugs in solution.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Given a non-empty list of integers, return the sum of all of the odd elements that are in even positions.
    
*/
fn solutions(lst: Vec<i32>) -> i32 {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
