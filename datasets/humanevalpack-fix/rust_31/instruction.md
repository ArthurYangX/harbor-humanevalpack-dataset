# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return true if a given number is prime, and false otherwise.
    
*/
fn is_prime(n:i32) -> bool{

    if n < 1{
    return false;
}
for k in 1..n-1 {
    if n % k == 0{
        return false;
    }
}
return true;

}
```

# Instruction

Fix bugs in is_prime.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Return true if a given number is prime, and false otherwise.
    
*/
fn is_prime(n:i32) -> bool{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
