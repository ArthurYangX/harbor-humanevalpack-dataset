# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Task
    Write a function that takes a string as input and returns the sum of the upper characters only'
    ASCII codes.
    
*/
fn digitSum(s:&str) -> i32{

    return s.chars().into_iter().filter(|c:&char| c.is_lowercase()).map(|c:char| c as i32).sum();
}
```

# Instruction

Fix bugs in digitSum.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Task
    Write a function that takes a string as input and returns the sum of the upper characters only'
    ASCII codes.
    
*/
fn digitSum(s:&str) -> i32{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
