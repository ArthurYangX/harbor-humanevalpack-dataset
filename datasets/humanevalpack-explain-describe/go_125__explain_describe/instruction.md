# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
// should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
// alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
// Examples
// SplitWords("Hello world!") ➞ ["Hello", "world!"]
// SplitWords("Hello,world!") ➞ ["Hello", "world!"]
// SplitWords("abcdef") == 3
func SplitWords(txt string) interface{} {
    if strings.Contains(txt, " ") {
        return strings.Fields(txt)
    } else if strings.Contains(txt, ",") {
        return strings.Split(txt, ",")
    }
    cnt := 0
    for _, r := range txt {
        if 'a' <= r && r <= 'z' && (r-'a')&1==1 {
            cnt++
        }
    }
    return cnt
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

// Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
// should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
// alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
// Examples
// SplitWords("Hello world!") ➞ ["Hello", "world!"]
// SplitWords("Hello,world!") ➞ ["Hello", "world!"]
// SplitWords("abcdef") == 3
func SplitWords(txt string) interface{} {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
