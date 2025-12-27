# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Create a function that takes a string as input which contains only square brackets.
// The function should return true if and only if there is a valid subsequence of brackets
// where at least one bracket in the subsequence is nested.
// 
// IsNested('[[]]') ➞ true
// IsNested('[]]]]]]][[[[[]') ➞ false
// IsNested('[][]') ➞ false
// IsNested('[]') ➞ false
// IsNested('[[][]]') ➞ true
// IsNested('[[]][[') ➞ true
func IsNested(s string) bool {
    opening_bracket_index := make([]int, 0)
    closing_bracket_index := make([]int, 0)
    for i:=0;i < len(s);i++ {
        if s[i] == '[' {
            opening_bracket_index = append(opening_bracket_index, i)
        } else {
            closing_bracket_index = append(closing_bracket_index, i)
        }
    }
    for i := 0;i < len(closing_bracket_index)>>1;i++ {
        closing_bracket_index[i], closing_bracket_index[len(closing_bracket_index)-i-1] = closing_bracket_index[len(closing_bracket_index)-i-1], closing_bracket_index[i]
    }
    cnt := 0
    i := 0
    l := len(closing_bracket_index)
    for _, idx := range opening_bracket_index {
        if i < l && idx < closing_bracket_index[i] {
            cnt++
            i++
        }
    }
    return cnt >= 2
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


// Create a function that takes a string as input which contains only square brackets.
// The function should return true if and only if there is a valid subsequence of brackets
// where at least one bracket in the subsequence is nested.
// 
// IsNested('[[]]') ➞ true
// IsNested('[]]]]]]][[[[[]') ➞ false
// IsNested('[][]') ➞ false
// IsNested('[]') ➞ false
// IsNested('[[][]]') ➞ true
// IsNested('[[]][[') ➞ true
func IsNested(s string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
