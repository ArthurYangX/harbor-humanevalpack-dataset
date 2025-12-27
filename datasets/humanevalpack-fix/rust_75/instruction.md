# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Write a function that returns true if the given number is the multiplication of 3 prime numbers
    and false otherwise.
    Knowing that (a) is less then 100.
    
*/
fn is_multiply_prime(a: i32) -> bool {

    let mut a1 = a;
    let mut num = 0;
    for i in 0..a {
        while a1 % i == 0 && a1 > i {
            a1 /= i;
            num += 1;
        }
    }
    if num == 2 {
        return true;
    }
    return false;
}
```

# Instruction

Fix bugs in is_multiply_prime.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
Write a function that returns true if the given number is the multiplication of 3 prime numbers
    and false otherwise.
    Knowing that (a) is less then 100.
    
*/
fn is_multiply_prime(a: i32) -> bool {



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
