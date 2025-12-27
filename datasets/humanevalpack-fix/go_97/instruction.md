# Context

```go
import (
    "math"
)

// Complete the function that takes two integers and returns
// the product of their unit digits.
// Assume the input is always valid.
// Examples:
// Multiply(148, 412) should return 16.
// Multiply(19, 28) should return 72.
// Multiply(2020, 1851) should return 0.
// Multiply(14,-15) should return 20.
func Multiply(a, b int) int {
    return int(math.Abs(float64(a%10)) * math.Abs(float64(b%10)) * float64(a) * float64(b))
}
```

# Instruction

Fix bugs in Multiply.

# Prompt

import (
    "math"
)

// Complete the function that takes two integers and returns
// the product of their unit digits.
// Assume the input is always valid.
// Examples:
// Multiply(148, 412) should return 16.
// Multiply(19, 28) should return 72.
// Multiply(2020, 1851) should return 0.
// Multiply(14,-15) should return 20.
func Multiply(a, b int) int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
