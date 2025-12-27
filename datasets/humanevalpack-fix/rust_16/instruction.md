# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Given a string, find out how many distinct characters (regardless of case) does it consist of
    
*/
fn count_distinct_characters(str:String) -> i32{

    let res:HashSet<char> = str.chars().into_iter().collect();
    return res.len() as i32;
}
```

# Instruction

Fix bugs in count_distinct_characters.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Given a string, find out how many distinct characters (regardless of case) does it consist of
    
*/
fn count_distinct_characters(str:String) -> i32{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
