# Context

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
    count := 0
	set := make(map[rune]bool)
	for _, i := range str {
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

Fix bugs in CountDistinctCharacters.

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

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
