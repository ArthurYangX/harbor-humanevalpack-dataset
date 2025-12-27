# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strconv"
)

// Given a positive integer n, return the product of the odd Digits.
// Return 0 if all Digits are even.
// For example:
// Digits(1)  == 1
// Digits(4)  == 0
// Digits(235) == 15
func Digits(n int) int {
    product := 1
    odd_count := 0
    for _, digit := range strconv.Itoa(n) {
        int_digit := int(digit-'0')
        if int_digit&1 == 1 {
            product= product*int_digit
            odd_count++
        }
    }
    if odd_count==0 {
        return 0
    }
    return product
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

// Given a positive integer n, return the product of the odd Digits.
// Return 0 if all Digits are even.
// For example:
// Digits(1)  == 1
// Digits(4)  == 0
// Digits(235) == 15
func Digits(n int) int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
