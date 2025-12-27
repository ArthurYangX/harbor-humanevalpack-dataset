# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "math"
    "sort"
)

// You are given a list of integers.
// Write a function NextSmallest() that returns the 2nd smallest element of the list.
// Return nil if there is no such element.
// 
// NextSmallest([1, 2, 3, 4, 5]) == 2
// NextSmallest([5, 1, 4, 3, 2]) == 2
// NextSmallest([]) == nil
// NextSmallest([1, 1]) == nil
func NextSmallest(lst []int) interface{} {
    set := make(map[int]struct{})
    for _, i := range lst {
        set[i] = struct{}{}
    }
    vals := make([]int, 0, len(set))
    for k := range set {
        vals = append(vals, k)
    }
    sort.Slice(vals, func(i, j int) bool {
        return vals[i] < vals[j]
    })
    if len(vals) < 2 {
        return nil
    }
    return vals[1]
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
    "sort"
)

// You are given a list of integers.
// Write a function NextSmallest() that returns the 2nd smallest element of the list.
// Return nil if there is no such element.
// 
// NextSmallest([1, 2, 3, 4, 5]) == 2
// NextSmallest([5, 1, 4, 3, 2]) == 2
// NextSmallest([]) == nil
// NextSmallest([1, 1]) == nil
func NextSmallest(lst []int) interface{} {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
