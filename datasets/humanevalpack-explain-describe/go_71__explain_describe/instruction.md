# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "math"
)

// Given the lengths of the three sides of a triangle. Return the area of
// the triangle rounded to 2 decimal points if the three sides form a valid triangle.
// Otherwise return -1
// Three sides make a valid triangle when the sum of any two sides is greater
// than the third side.
// Example:
// TriangleArea(3, 4, 5) == 6.00
// TriangleArea(1, 2, 10) == -1
func TriangleArea(a float64, b float64, c float64) interface{} {
    if a+b <= c || a+c <= b || b+c <= a {
		return -1
	}
	s := (a + b + c) / 2
	area := math.Pow(s * (s - a) * (s - b) * (s - c), 0.5)
	area = math.Round(area*100)/100
	return area
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
    "math"
)

// Given the lengths of the three sides of a triangle. Return the area of
// the triangle rounded to 2 decimal points if the three sides form a valid triangle.
// Otherwise return -1
// Three sides make a valid triangle when the sum of any two sides is greater
// than the third side.
// Example:
// TriangleArea(3, 4, 5) == 6.00
// TriangleArea(1, 2, 10) == -1
func TriangleArea(a float64, b float64, c float64) interface{} {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
