# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Concatenate list of strings into a single string
    
*/
fn concatenate(strings:Vec<String>) -> String{

    return strings.iter().fold(String::new(), |res, x| format!("{} {}", res, x));

}
```

# Instruction

Fix bugs in concatenate.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Concatenate list of strings into a single string
    
*/
fn concatenate(strings:Vec<String>) -> String{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
