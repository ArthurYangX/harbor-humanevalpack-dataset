# Context

You are given a reference implementation (canonical solution) to explain.

```go
// Given the lengths of the three sides of a triangle. Return true if the three
// sides form a right-angled triangle, false otherwise.
// A right-angled triangle is a triangle in which one angle is right angle or
// 90 degree.
// Example:
// RightAngleTriangle(3, 4, 5) == true
// RightAngleTriangle(1, 2, 3) == false
func RightAngleTriangle(a, b, c int) bool {
    return a*a == b*b + c*c || b*b == a*a + c*c || c*c == a*a + b*b
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


// Given the lengths of the three sides of a triangle. Return true if the three
// sides form a right-angled triangle, false otherwise.
// A right-angled triangle is a triangle in which one angle is right angle or
// 90 degree.
// Example:
// RightAngleTriangle(3, 4, 5) == true
// RightAngleTriangle(1, 2, 3) == false
func RightAngleTriangle(a, b, c int) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
