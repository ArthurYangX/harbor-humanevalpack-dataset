# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
	"sort"
)

// Return Median of elements in the list l.
// >>> Median([3, 1, 2, 4, 5])
// 3.0
// >>> Median([-10, 4, 6, 1000, 10, 20])
// 15.0
func Median(l []int) float64 {
    sort.Ints(l)
	if len(l)%2==1{
		return float64(l[len(l)/2])
	}else{
		return float64(l[len(l)/2-1]+l[len(l)/2])/2.0
	}
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
)

// Return Median of elements in the list l.
// >>> Median([3, 1, 2, 4, 5])
// 3.0
// >>> Median([-10, 4, 6, 1000, 10, 20])
// 15.0
func Median(l []int) float64 {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
