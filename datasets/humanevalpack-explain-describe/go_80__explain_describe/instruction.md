# Context

You are given a reference implementation (canonical solution) to explain.

```go
// You are given a string s.
// Your task is to check if the string is happy or not.
// A string is happy if its length is at least 3 and every 3 consecutive letters are distinct
// For example:
// IsHappy(a) => false
// IsHappy(aa) => false
// IsHappy(abcd) => true
// IsHappy(aabb) => false
// IsHappy(adb) => true
// IsHappy(xyy) => false
func IsHappy(s string) bool {
    if len(s) < 3 {
        return false
    }
    for i := 0; i < len(s)-2; i++ {
        if s[i] == s[i+1] || s[i+1] == s[i+2] || s[i] == s[i+2] {
            return false
        }
    }
    return true
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
// Your task is to check if the string is happy or not.
// A string is happy if its length is at least 3 and every 3 consecutive letters are distinct
// For example:
// IsHappy(a) => false
// IsHappy(aa) => false
// IsHappy(abcd) => true
// IsHappy(aabb) => false
// IsHappy(adb) => true
// IsHappy(xyy) => false
func IsHappy(s string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
