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
 Given list of numbers (of at least two elements), apply a linear transform to that list,
    such that the smallest number will become 0 and the largest will become 1
    
*/
fn rescale_to_unit(numbers:Vec<f32>) -> Vec<f32> {

    let min_number= *numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_number=  *numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    return numbers.iter().map(|x:&f32| (x-min_number) / (max_number - min_number)).collect();
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
 Given list of numbers (of at least two elements), apply a linear transform to that list,
    such that the smallest number will become 0 and the largest will become 1
    
*/
fn rescale_to_unit(numbers:Vec<f32>) -> Vec<f32> {



# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
