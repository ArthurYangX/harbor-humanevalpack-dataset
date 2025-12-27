# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    remove_vowels is a function that takes string and returns string without vowels.
    
*/
fn remove_vowels(text: &str) -> String {

    let vowels = "AEIOUWYaeiouwy";
    let mut out = String::new();
    for c in text.chars() {
        if !vowels.contains(c) {
            out.push(c);
        }
    }
    out
}
```

# Instruction

Fix bugs in remove_vowels.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    remove_vowels is a function that takes string and returns string without vowels.
    
*/
fn remove_vowels(text: &str) -> String {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
