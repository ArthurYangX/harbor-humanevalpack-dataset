# Context

```go
import (
    "math"
)

// Given a positive integer n, return the count of the numbers of n-digit
// positive integers that start or end with 1.
func StartsOneEnds(n int) int {
    if n == 1 {
        return 1
    }
    return 18 * n * int(math.Pow(10, float64(n-2)))
}
```

# Instruction

Fix bugs in StartsOneEnds.

# Prompt

import (
    "math"
)

// Given a positive integer n, return the count of the numbers of n-digit
// positive integers that start or end with 1.
func StartsOneEnds(n int) int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
