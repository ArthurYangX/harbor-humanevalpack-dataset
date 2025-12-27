# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "math"
    "strconv"
)

// Write a function CountNums which takes an array of integers and returns
// the number of elements which has a sum of digits > 0.
// If a number is negative, then its first signed digit will be negative:
// e.g. -123 has signed digits -1, 2, and 3.
// >>> CountNums([]) == 0
// >>> CountNums([-1, 11, -11]) == 1
// >>> CountNums([1, 1, 2]) == 3
func CountNums(arr []int) int {
    digits_sum:= func (n int) int {
        neg := 1
        if n < 0 {
             n, neg = -1 * n, -1 
        }
        r := make([]int,0)
        for _, c := range strconv.Itoa(n) {
            r = append(r, int(c-'0'))
        }
        r[0] *= neg
        sum := 0
        for _, i := range r {
            sum += i
        }
        return sum
    }
    count := 0
    for _, i := range arr {
        x := digits_sum(i)
        if x > 0 {
            count++
        }
    }
    return count
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
    "strconv"
)

// Write a function CountNums which takes an array of integers and returns
// the number of elements which has a sum of digits > 0.
// If a number is negative, then its first signed digit will be negative:
// e.g. -123 has signed digits -1, 2, and 3.
// >>> CountNums([]) == 0
// >>> CountNums([-1, 11, -11]) == 1
// >>> CountNums([1, 1, 2]) == 3
func CountNums(arr []int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
