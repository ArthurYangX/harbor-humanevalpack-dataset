# Context

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
        if c >= 'a' && c <= 'z' {
            result = append(result, 'a' + ((c - 'A' + 26) % 26))
        } else if c >= 'A' && c <= 'Z' {
            result = append(result, 'A' + ((c - 'a' + 26) % 26))
        } else {
            result = append(result, c)
        }
    }
    return string(result)
}
```

# Instruction

Fix bugs in FlipCase.

# Prompt

import (
    "strings"
)

// For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
// >>> FlipCase('Hello')
// 'hELLO'
func FlipCase(str string) string {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
