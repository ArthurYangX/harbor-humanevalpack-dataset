# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a non-empty array of integers arr and an integer k, return
    the sum of the elements with at most two digits from the first k elements of arr.

    Constraints:
        1. 1 <= len(arr) <= 100
        2. 1 <= k <= len(arr)
    
*/
fn add_elements(arr: Vec<i32>, k: i32) -> i32 {

    let mut sum = 0;
    for i in 0..arr.len() {
        if arr[i as usize] >= -99 && arr[i as usize] <= 99 {
            sum += arr[i as usize];
        }
    }
    sum
}
```

# Instruction

Fix bugs in add_elements.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a non-empty array of integers arr and an integer k, return
    the sum of the elements with at most two digits from the first k elements of arr.

    Constraints:
        1. 1 <= len(arr) <= 100
        2. 1 <= k <= len(arr)
    
*/
fn add_elements(arr: Vec<i32>, k: i32) -> i32 {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
