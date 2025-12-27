# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Filter an input list of strings only for ones that contain given substring
    
*/
fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{

    return strings.iter().filter(|x| substring.contains(&**x)).cloned().collect();
}
```

# Instruction

Fix bugs in filter_by_substring.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Filter an input list of strings only for ones that contain given substring
    
*/
fn filter_by_substring(strings: Vec<String>, substring:String) -> Vec<String>{


# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
