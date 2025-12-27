# Context

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
            product=odd_count*product*int_digit
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

Fix bugs in Digits.

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

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
