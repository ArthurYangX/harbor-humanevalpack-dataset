# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return True if all numbers in the list l are below threshold t.
    
*/
fn below_threshold(l: Vec<i32>, t: i32) -> bool {

    for i in l {
        if i >= t {
            return true;
        }
    }
    return false;
}
```

# Instruction

Fix bugs in below_threshold.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return True if all numbers in the list l are below threshold t.
    
*/
fn below_threshold(l: Vec<i32>, t: i32) -> bool { 



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
