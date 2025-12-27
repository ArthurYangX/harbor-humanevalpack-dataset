# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "sort"
    "strconv"
)

// Write a function which sorts the given list of integers
// in ascending order according to the sum of their digits.
// Note: if there are several items with similar sum of their digits,
// order them based on their index in original list.
// 
// For example:
// >>> OrderByPoints([1, 11, -1, -11, -12]) == [-1, -11, 1, -12, 11]
// >>> OrderByPoints([]) == []
func OrderByPoints(nums []int) []int {
    digits_sum := func (n int) int {
        neg := 1
        if n < 0 {
            n, neg = -1 * n, -1 
        }
        sum := 0
        for i, c := range strconv.Itoa(n) {
            if i == 0 {
                sum += int(c-'0')*neg
            } else {
                sum += int(c-'0')
            }
        }
        return sum
    }
    sort.SliceStable(nums, func(i, j int) bool {
        return digits_sum(nums[i]) < digits_sum(nums[j])
    })
    return nums
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
    "sort"
    "strconv"
)

// Write a function which sorts the given list of integers
// in ascending order according to the sum of their digits.
// Note: if there are several items with similar sum of their digits,
// order them based on their index in original list.
// 
// For example:
// >>> OrderByPoints([1, 11, -1, -11, -12]) == [-1, -11, 1, -12, 11]
// >>> OrderByPoints([]) == []
func OrderByPoints(nums []int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
