# Context

```go
import (
    "sort"
)

// Return sorted Unique elements in a list
// >>> Unique([5, 3, 5, 2, 3, 3, 9, 0, 123])
// [0, 2, 3, 5, 9, 123]
func Unique(l []int) []int {
    sort.Ints(l)
	return l
}
```

# Instruction

Fix bugs in Unique.

# Prompt

import (
    "sort"
)

// Return sorted Unique elements in a list
// >>> Unique([5, 3, 5, 2, 3, 3, 9, 0, 123])
// [0, 2, 3, 5, 9, 123]
func Unique(l []int) []int {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
