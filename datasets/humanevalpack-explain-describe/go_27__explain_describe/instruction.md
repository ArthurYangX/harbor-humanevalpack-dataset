# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
// >>> FlipCase('Hello')
// 'hELLO'
func FlipCase(str string) string {
    result := []rune{}
    for _, c := range str {
        if c >= 'A' && c <= 'Z' {
            result = append(result, 'a' + ((c - 'A' + 26) % 26))
        } else if c >= 'a' && c <= 'z' {
            result = append(result, 'A' + ((c - 'a' + 26) % 26))
        } else {
            result = append(result, c)
        }
    }
    return string(result)
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
    "strings"
)

// For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
// >>> FlipCase('Hello')
// 'hELLO'
func FlipCase(str string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
