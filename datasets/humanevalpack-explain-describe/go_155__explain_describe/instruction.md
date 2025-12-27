# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strconv"
)

// Given an integer. return a tuple that has the number of even and odd digits respectively.
// 
// Example:
// EvenOddCount(-12) ==> (1, 1)
// EvenOddCount(123) ==> (1, 2)
func EvenOddCount(num int) [2]int {
    even_count := 0
    odd_count := 0
    if num < 0 {
        num = -num
    }
    for _, r := range strconv.Itoa(num) {
        if r&1==0 {
            even_count++
        } else {
            odd_count++
        }
    }
    return [2]int{even_count, odd_count}
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

import (
    "strconv"
)

// Given an integer. return a tuple that has the number of even and odd digits respectively.
// 
// Example:
// EvenOddCount(-12) ==> (1, 1)
// EvenOddCount(123) ==> (1, 2)
func EvenOddCount(num int) [2]int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
