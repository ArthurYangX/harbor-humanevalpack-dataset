# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "sort"
    "strings"
)

// Write a function that takes a string and returns an ordered version of it.
// Ordered version of string, is a string where all words (separated by space)
// are replaced by a new word where all the characters arranged in
// ascending order based on ascii value.
// Note: You should keep the order of words and blank spaces in the sentence.
// 
// For example:
// AntiShuffle('Hi') returns 'Hi'
// AntiShuffle('hello') returns 'ehllo'
// AntiShuffle('Hello World!!!') returns 'Hello !!!Wdlor'
func AntiShuffle(s string) string {
    strs := make([]string, 0)
    for _, i := range strings.Fields(s) {
        word := []rune(i)
        sort.Slice(word, func(i, j int) bool {
            return word[i] < word[j]
        })
        strs = append(strs, string(word))
    }
    return strings.Join(strs, " ")
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
    "sort"
    "strings"
)

// Write a function that takes a string and returns an ordered version of it.
// Ordered version of string, is a string where all words (separated by space)
// are replaced by a new word where all the characters arranged in
// ascending order based on ascii value.
// Note: You should keep the order of words and blank spaces in the sentence.
// 
// For example:
// AntiShuffle('Hi') returns 'Hi'
// AntiShuffle('hello') returns 'ehllo'
// AntiShuffle('Hello World!!!') returns 'Hello !!!Wdlor'
func AntiShuffle(s string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
