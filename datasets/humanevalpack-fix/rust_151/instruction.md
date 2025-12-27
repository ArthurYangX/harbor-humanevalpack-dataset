# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a list of numbers, return the sum of squares of the numbers
    in the list that are odd. Ignore numbers that are negative or not integers.
   
    If the input list is empty, return 0.
    
*/
fn double_the_difference(lst: Vec<f32>) -> i64 {

    let mut sum: i64 = 0;
    for i in 0..lst.len() {
        if (lst[i] - lst[i].round()).abs() < 1e-4 {
            if lst[i] > 0.0 {
                sum += (lst[i].round() as i64) * (lst[i].round() as i64);
            }
        }
    }
    return sum;
}
```

# Instruction

Fix bugs in double_the_difference.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a list of numbers, return the sum of squares of the numbers
    in the list that are odd. Ignore numbers that are negative or not integers.
   
    If the input list is empty, return 0.
    
*/
fn double_the_difference(lst: Vec<f32>) -> i64 {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
