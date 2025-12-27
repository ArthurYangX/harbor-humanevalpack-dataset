# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Given a non-empty list of integers lst. add the even elements that are at odd indices..
    
*/
fn add_even_odd(lst: Vec<i32>) -> i32{

    let mut sum: i32 = 0;

    for (indx, elem) in lst.iter().enumerate() {
        if indx % 2 == 1 {
                sum += elem
            }
    }
    return sum;
}
```

# Instruction

Fix bugs in add.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Given a non-empty list of integers lst. add the even elements that are at odd indices..
    
*/
fn add_even_odd(lst: Vec<i32>) -> i32{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
