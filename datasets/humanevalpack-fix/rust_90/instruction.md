# Context

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    You are given a list of integers.
    Write a function next_smallest() that returns the 2nd smallest element of the list.
    Return None if there is no such element.
    
*/
fn next_smallest(lst:Vec<i32>) -> i32{

    let mut res = 0;
    let mut lst_cp = lst.clone();
    let mut first: i32 = 1;
    let mut second: i32 = 2;

    if lst.iter().min() == None {
        res = 0;
    } else {
        if lst.iter().min() != None {
            first = *lst.iter().min().unwrap();
            let indx = lst.iter().position(|x| *x == first).unwrap();
            lst_cp.remove(indx);

            if lst_cp.iter().min() != None {
                second = *lst_cp.iter().min().unwrap();
            }
            if first != second {
                res = second;
            } else {
                res = 0;
            }
        }
    }
    return res;
}
```

# Instruction

Fix bugs in next_smallest.

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*

    You are given a list of integers.
    Write a function next_smallest() that returns the 2nd smallest element of the list.
    Return None if there is no such element.
    
*/
fn next_smallest(lst:Vec<i32>) -> i32{



# Instructions

Implement your solution in `solution/solution.rs`.
Ensure your submission is self-contained and compiles/runs correctly.

```
