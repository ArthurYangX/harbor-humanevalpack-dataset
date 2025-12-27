# Context

You are given a reference implementation (canonical solution) to explain.

```go
// You are given a string s.
// if s[i] is a letter, reverse its case from lower to upper or vise versa,
// otherwise keep it as it is.
// If the string contains no letters, reverse the string.
// The function should return the resulted string.
// Examples
// Solve("1234") = "4321"
// Solve("ab") = "AB"
// Solve("#a@C") = "#A@c"
func Solve(s string) string {
    flg := 0
    new_str := []rune(s)
    for i, r := range new_str {
        if ('a' <= r && r <= 'z') || ('A' <= r && r <= 'Z') {
            if 'a' <= r && r <= 'z' {
                new_str[i] = r - 'a' + 'A'
            } else {
                new_str[i] = r - 'A' + 'a'
            }
            flg = 1
        }
    }
    if flg == 0 {
        for i := 0;i < len(new_str)>>1;i++ {
            new_str[i], new_str[len(new_str)-i-1] = new_str[len(new_str)-i-1], new_str[i]
        }
    }
    return string(new_str)
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


// You are given a string s.
// if s[i] is a letter, reverse its case from lower to upper or vise versa,
// otherwise keep it as it is.
// If the string contains no letters, reverse the string.
// The function should return the resulted string.
// Examples
// Solve("1234") = "4321"
// Solve("ab") = "AB"
// Solve("#a@C") = "#A@c"
func Solve(s string) string {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
