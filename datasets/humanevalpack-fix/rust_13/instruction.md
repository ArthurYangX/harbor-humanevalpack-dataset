# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Return a greatest common divisor of two integers a and b
    
*/
fn greatest_common_divisor(mut a:i32,mut b:i32) -> i32{

    while b > 0 {
        (a, b) = (b, a % b);
    }
    return b;
}
```

# Instruction

Fix bugs in greatest_common_divisor.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 Return a greatest common divisor of two integers a and b
    
*/
fn greatest_common_divisor(mut a:i32,mut b:i32) -> i32{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
