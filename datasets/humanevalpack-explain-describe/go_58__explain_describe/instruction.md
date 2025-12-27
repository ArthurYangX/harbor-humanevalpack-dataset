# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "sort"
)

// Return sorted unique Common elements for two lists.
// >>> Common([1, 4, 3, 34, 653, 2, 5], [5, 7, 1, 5, 9, 653, 121])
// [1, 5, 653]
// >>> Common([5, 3, 2, 8], [3, 2])
// [2, 3]
func Common(l1 []int,l2 []int) []int {
    m := make(map[int]bool)
	for _, e1 := range l1 {
		if m[e1] {
			continue
		}
		for _, e2 := range l2 {
			if e1 == e2 {
				m[e1] = true
				break
			}
		}
	}
	res := make([]int, 0, len(m))
	for k, _ := range m {
		res = append(res, k)
	}
	sort.Ints(res)
	return res
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

// Return sorted unique Common elements for two lists.
// >>> Common([1, 4, 3, 34, 653, 2, 5], [5, 7, 1, 5, 9, 653, 121])
// [1, 5, 653]
// >>> Common([5, 3, 2, 8], [3, 2])
// [2, 3]
func Common(l1 []int,l2 []int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
