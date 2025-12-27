# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strings"
)

// Test if given string is a palindrome.
func IsPalindrome(str string) bool {
    runes := []rune(str)
    result := make([]rune, 0)
    for i := len(runes) - 1; i >= 0; i-- {
        result = append(result, runes[i])
    }
    return str == string(result)
}
// Find the shortest palindrome that begins with a supplied string.
// Algorithm idea is simple:
// - Find the longest postfix of supplied string that is a palindrome.
// - Append to the end of the string reverse of a string prefix that comes before the palindromic suffix.
// >>> MakePalindrome('')
// ''
// >>> MakePalindrome('cat')
// 'catac'
// >>> MakePalindrome('cata')
// 'catac'
func MakePalindrome(str string) string {
    if strings.TrimSpace(str) == "" {
        return ""
    }
    beginning_of_suffix := 0
    runes := []rune(str)
    for !IsPalindrome(string(runes[beginning_of_suffix:])) {
        beginning_of_suffix += 1
    }
    result := make([]rune, 0)
    for i := len(str[:beginning_of_suffix]) - 1; i >= 0; i-- {
        result = append(result, runes[i])
    }
    return str + string(result)
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

// Test if given string is a palindrome.
func IsPalindrome(str string) bool {
    runes := []rune(str)
    result := make([]rune, 0)
    for i := len(runes) - 1; i >= 0; i-- {
        result = append(result, runes[i])
    }
    return str == string(result)
}
// Find the shortest palindrome that begins with a supplied string.
// Algorithm idea is simple:
// - Find the longest postfix of supplied string that is a palindrome.
// - Append to the end of the string reverse of a string prefix that comes before the palindromic suffix.
// >>> MakePalindrome('')
// ''
// >>> MakePalindrome('cat')
// 'catac'
// >>> MakePalindrome('cata')
// 'catac'
func MakePalindrome(str string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
