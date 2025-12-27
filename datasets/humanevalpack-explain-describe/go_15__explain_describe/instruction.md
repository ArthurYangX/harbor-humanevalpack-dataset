# Context

You are given a reference implementation (canonical solution) to explain.

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
    for i := 0; i <= n; i++ {
        seq += strconv.Itoa(i)
        if i != n {
            seq += " "
        }
    }
    return seq
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
    "strconv"
)

// Return a string containing space-delimited numbers starting from 0 upto n inclusive.
// >>> StringSequence(0)
// '0'
// >>> StringSequence(5)
// '0 1 2 3 4 5'
func StringSequence(n int) string{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
