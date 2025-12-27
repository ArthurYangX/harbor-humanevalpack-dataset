# Context

You are given a reference implementation (canonical solution) to explain.

```rust
fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 From a given list of integers, generate a list of rolling maximum element found until given moment
    in the sequence.
    
*/
fn rolling_max(numbers:Vec<i32>) -> Vec<i32>{

    let mut running_max :Option<i32> = None;
    let mut result:Vec<i32> = vec![];

    for n in numbers{
        if running_max == None {
            running_max = Some(n);

        }else{
            running_max = max(running_max, Some(n));
        }

        result.push(running_max.unwrap());
    }
    return result;

}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

fn main(){}

use std::{slice::Iter, cmp::{max, self}, mem::replace, collections::{HashSet, HashMap}, ops::Index, ascii::AsciiExt};
use rand::Rng;
use regex::Regex;
use md5;
use std::any::{Any, TypeId};

/*
 From a given list of integers, generate a list of rolling maximum element found until given moment
    in the sequence.
    
*/
fn rolling_max(numbers:Vec<i32>) -> Vec<i32>{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
