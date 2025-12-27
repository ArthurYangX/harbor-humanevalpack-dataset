# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// Given a string, find out how many distinct characters (regardless of case) does it consist of
// >>> CountDistinctCharacters('xyzXYZ')
// 3
// >>> CountDistinctCharacters('Jerry')
// 4
func CountDistinctCharacters(str string) int{
    lower := strings.ToLower(str)
	count := 0
	set := make(map[rune]bool)
	for _, i := range lower {
		if set[i] == true {
			continue
		} else {
			set[i] = true
			count++
		}
	}
	return count
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

// Given a string, find out how many distinct characters (regardless of case) does it consist of
// >>> CountDistinctCharacters('xyzXYZ')
// 3
// >>> CountDistinctCharacters('Jerry')
// 4
func CountDistinctCharacters(str string) int{


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
