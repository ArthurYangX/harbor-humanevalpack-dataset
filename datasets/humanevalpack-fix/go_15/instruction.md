# Context

```go
import (
    "strconv"
)

// Return a string containing space-delimited numbers starting from 0 upto n inclusive.
// >>> StringSequence(0)
// '0'
// >>> StringSequence(5)
// '0 1 2 3 4 5'
func StringSequence(n int) string{
    var seq string
    for i := 0; i < n; i++ {
        seq += strconv.Itoa(i)
        if i != n {
            seq += " "
        }
    }
    return seq
}
```

# Instruction

Fix bugs in StringSequence.

# Prompt

import (
    "strconv"
)

// Return a string containing space-delimited numbers starting from 0 upto n inclusive.
// >>> StringSequence(0)
// '0'
// >>> StringSequence(5)
// '0 1 2 3 4 5'
func StringSequence(n int) string{


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
