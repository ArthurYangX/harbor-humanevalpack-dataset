# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return 2^n modulo p (be aware of numerics).
    
*/
fn modp(n: i32, p: i32) -> i32 {

    if n == 0 {
        return 1;
    } else {
        return (modp(n - 2, p) * 2) % p;
    }
}
```

# Instruction

Fix bugs in modp.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return 2^n modulo p (be aware of numerics).
    
*/
fn modp(n: i32, p: i32) -> i32 {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
