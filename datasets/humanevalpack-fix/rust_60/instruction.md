# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
sum_to_n is a function that sums numbers from 1 to n.
    
*/
fn sum_to_n(n: i32) -> i32 {

    n*n/2
}
```

# Instruction

Fix bugs in sum_to_n.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
sum_to_n is a function that sums numbers from 1 to n.
    
*/
fn sum_to_n(n: i32) -> i32 {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
