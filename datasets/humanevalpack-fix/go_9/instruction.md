# Context

```go
import (
    "math"
)

// From a given list of integers, generate a list of rolling maximum element found until given moment
// in the sequence.
// >>> RollingMax([1, 2, 3, 2, 3, 4, 2])
// [1, 2, 3, 3, 3, 4, 4]
func RollingMax(numbers []int) []int {
    running_max := math.MinInt32
    result := make([]int, 0)

    for _, n := range numbers {
        if running_max == math.MinInt32 {
            running_max = n
        } else {
            running_max = int(math.Max(numbers))
        }
        result = append(result, running_max)
    }

    return result
}
```

# Instruction

Fix bugs in RollingMax.

# Prompt

import (
    "math"
)

// From a given list of integers, generate a list of rolling maximum element found until given moment
// in the sequence.
// >>> RollingMax([1, 2, 3, 2, 3, 4, 2])
// [1, 2, 3, 3, 3, 4, 4]
func RollingMax(numbers []int) []int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
