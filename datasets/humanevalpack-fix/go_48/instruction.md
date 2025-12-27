# Context

```go
// Checks if given string is a palindrome
// >>> IsPalindrome('')
// true
// >>> IsPalindrome('aba')
// true
// >>> IsPalindrome('aaaaa')
// true
// >>> IsPalindrome('zbcd')
// false
func IsPalindrome(text string) bool {
    runes := []rune(text)
    result := make([]rune, 0)
    for i := len(runes) - 1; i > 0; i-- {
        result = append(result, runes[i])
    }
    return text == string(result)
}
```

# Instruction

Fix bugs in IsPalindrome.

# Prompt


// Checks if given string is a palindrome
// >>> IsPalindrome('')
// true
// >>> IsPalindrome('aba')
// true
// >>> IsPalindrome('aaaaa')
// true
// >>> IsPalindrome('zbcd')
// false
func IsPalindrome(text string) bool {


# Instructions

Implement your solution in `solution/solution.go`.
Ensure your submission is self-contained and compiles/runs correctly.

```
