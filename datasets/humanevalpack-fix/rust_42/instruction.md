# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return list with elements incremented by 1.
    
*/
fn incr_list(l:Vec<i32>) -> Vec<i32>{

    return l.into_iter().map(|n:i32| n + 2).collect();
}
```

# Instruction

Fix bugs in incr_list.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return list with elements incremented by 1.
    
*/
fn incr_list(l:Vec<i32>) -> Vec<i32>{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
