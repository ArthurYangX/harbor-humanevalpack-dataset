# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Check if two words have the same characters.
    
*/
fn same_chars(str1:&str, str2:&str) -> bool{

    let mut v1:Vec<char> = str1.chars().into_iter().collect();
    v1.sort();

    let mut v2:Vec<char> = str2.chars().into_iter().collect();
    v2.sort();

    return v1 == v2;
}
```

# Instruction

Fix bugs in same_chars.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Check if two words have the same characters.
    
*/
fn same_chars(str1:&str, str2:&str) -> bool{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
