# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a string text, replace all spaces in it with underscores, 
    and if a string has more than 2 consecutive spaces, 
    then replace all consecutive spaces with - 
    
*/
fn fix_spaces(text: &str) -> String {

    let mut out = String::new();
    let mut spacelen = 0;
    for c in text.chars() {
        if c == ' ' {
            spacelen += 1;
        } else {
            if spacelen == 1 {
                out.push('_');
            }
            if spacelen == 2 {
                out.push_str("__");
            }
            if spacelen > 2 {
                out.push_str("---");
            }
            spacelen = 0;
            out.push(c);
        }
    }
    if spacelen == 1 {
        out.push('_');
    }
    if spacelen == 2 {
        out.push_str("__");
    }
    if spacelen > 2 {
        out.push_str("--");
    }
    out
}
```

# Instruction

Fix bugs in fix_spaces.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    Given a string text, replace all spaces in it with underscores, 
    and if a string has more than 2 consecutive spaces, 
    then replace all consecutive spaces with - 
    
*/
fn fix_spaces(text: &str) -> String {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
