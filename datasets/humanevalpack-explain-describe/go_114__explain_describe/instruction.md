# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "math"
)

// Given an array of integers nums, find the minimum sum of any non-empty sub-array
// of nums.
// Example
// Minsubarraysum([2, 3, 4, 1, 2, 4]) == 1
// Minsubarraysum([-1, -2, -3]) == -6
func Minsubarraysum(nums []int) int {
    max_sum := 0
    s := 0
    for _, num := range nums {
        s += -num
        if s < 0 {
            s = 0
        }
        if s > max_sum {
            max_sum = s
        }
    }
    if max_sum == 0 {
        max_sum = math.MinInt
        for _, i := range nums {
            if -i > max_sum {
                max_sum = -i
            }
        }
    }
    return -max_sum
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
    "math"
)

// Given an array of integers nums, find the minimum sum of any non-empty sub-array
// of nums.
// Example
// Minsubarraysum([2, 3, 4, 1, 2, 4]) == 1
// Minsubarraysum([-1, -2, -3]) == -6
func Minsubarraysum(nums []int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
