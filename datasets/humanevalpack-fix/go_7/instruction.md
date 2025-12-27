# Context

```go
import (
    "strings"
)

// Filter an input list of strings only for ones that contain given substring
// >>> FilterBySubstring([], 'a')
// []
// >>> FilterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
// ['abc', 'bacd', 'array']
func FilterBySubstring(stringList []string, substring string) []string {
    result := make([]string, 0)
    for _, x := range stringList {
        if strings.Index(substring, x) != -1 {
            result = append(result, x)
        }
    }
    return result
}
```

# Instruction

Fix bugs in FilterBySubstring.

# Prompt

import (
    "strings"
)

// Filter an input list of strings only for ones that contain given substring
// >>> FilterBySubstring([], 'a')
// []
// >>> FilterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
// ['abc', 'bacd', 'array']
func FilterBySubstring(stringList []string, substring string) []string {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
