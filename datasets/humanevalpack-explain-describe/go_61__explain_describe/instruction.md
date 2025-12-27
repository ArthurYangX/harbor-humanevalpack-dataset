# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// brackets is a string of "(" and ")".
// return true if every opening bracket has a corresponding closing bracket.
// 
// >>> CorrectBracketing("(")
// false
// >>> CorrectBracketing("()")
// true
// >>> CorrectBracketing("(()())")
// true
// >>> CorrectBracketing(")(()")
// false
func CorrectBracketing(brackets string) bool {
    brackets = strings.Replace(brackets, "(", " ( ", -1)
	brackets = strings.Replace(brackets, ")", ") ", -1)
	open := 0
	for _, b := range brackets {
		if b == '(' {
			open++
		} else if b == ')' {
			open--
		}
		if open < 0 {
			return false
		}
	}
	return open == 0
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

// brackets is a string of "(" and ")".
// return true if every opening bracket has a corresponding closing bracket.
// 
// >>> CorrectBracketing("(")
// false
// >>> CorrectBracketing("()")
// true
// >>> CorrectBracketing("(()())")
// true
// >>> CorrectBracketing(")(()")
// false
func CorrectBracketing(brackets string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
