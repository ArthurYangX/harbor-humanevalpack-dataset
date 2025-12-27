# Context

You are given a reference implementation (canonical solution) to explain.

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
            running_max = int(math.Max(float64(running_max), float64(n)))
        }
        result = append(result, running_max)
    }

    return result
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

// From a given list of integers, generate a list of rolling maximum element found until given moment
// in the sequence.
// >>> RollingMax([1, 2, 3, 2, 3, 4, 2])
// [1, 2, 3, 3, 3, 4, 4]
func RollingMax(numbers []int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
