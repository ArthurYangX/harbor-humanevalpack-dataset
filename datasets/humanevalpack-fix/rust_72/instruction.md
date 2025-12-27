# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Write a function that returns True if the object q will fly, and False otherwise.
    The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.
    
*/
fn will_it_fly(q:Vec<i32>, w:i32) -> bool{

    if q.iter().sum::<i32>() > w {
        return false;
    }
    let mut i = 0;
    let mut j = q.len() - 1;

    while i < j {
        if q[i] == q[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}
```

# Instruction

Fix bugs in will_it_fly.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Write a function that returns True if the object q will fly, and False otherwise.
    The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.
    
*/
fn will_it_fly(q:Vec<i32>, w:i32) -> bool{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
