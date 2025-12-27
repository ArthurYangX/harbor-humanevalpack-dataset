# Context

```go
import (
    "regexp"
)

// RemoveVowels is a function that takes string and returns string without vowels.
// >>> RemoveVowels('')
// ''
// >>> RemoveVowels("abcdef\nghijklm")
// 'bcdf\nghjklm'
// >>> RemoveVowels('abcdef')
// 'bcdf'
// >>> RemoveVowels('aaaaa')
// ''
// >>> RemoveVowels('aaBAA')
// 'B'
// >>> RemoveVowels('zbcd')
// 'zbcd'
func RemoveVowels(text string) string {
    var re = regexp.MustCompile("[aeiouwyAEIOUWY]")
	text = re.ReplaceAllString(text, "")
	return text
}
```

# Instruction

Fix bugs in RemoveVowels.

# Prompt

import (
    "regexp"
)

// RemoveVowels is a function that takes string and returns string without vowels.
// >>> RemoveVowels('')
// ''
// >>> RemoveVowels("abcdef\nghijklm")
// 'bcdf\nghjklm'
// >>> RemoveVowels('abcdef')
// 'bcdf'
// >>> RemoveVowels('aaaaa')
// ''
// >>> RemoveVowels('aaBAA')
// 'B'
// >>> RemoveVowels('zbcd')
// 'zbcd'
func RemoveVowels(text string) string {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
