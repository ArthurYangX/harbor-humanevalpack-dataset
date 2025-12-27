# Context

You are given a reference implementation (canonical solution) to explain.

```go
import (
    "strconv"
    "strings"
)

// Your task is to implement a function that will Simplify the expression
// x * n. The function returns true if x * n evaluates to a whole number and false
// otherwise. Both x and n, are string representation of a fraction, and have the following format,
// <numerator>/<denominator> where both numerator and denominator are positive whole numbers.
// 
// You can assume that x, and n are valid fractions, and do not have zero as denominator.
// 
// Simplify("1/5", "5/1") = true
// Simplify("1/6", "2/1") = false
// Simplify("7/10", "10/2") = false
func Simplify(x, n string) bool {
    xx := strings.Split(x, "/")
    nn := strings.Split(n, "/")
    a, _ := strconv.Atoi(xx[0])
    b, _ := strconv.Atoi(xx[1])
    c, _ := strconv.Atoi(nn[0])
    d, _ := strconv.Atoi(nn[1])
    numerator := float64(a*c)
    denom := float64(b*d)
    return numerator/denom == float64(int(numerator/denom))
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
    "strconv"
    "strings"
)

// Your task is to implement a function that will Simplify the expression
// x * n. The function returns true if x * n evaluates to a whole number and false
// otherwise. Both x and n, are string representation of a fraction, and have the following format,
// <numerator>/<denominator> where both numerator and denominator are positive whole numbers.
// 
// You can assume that x, and n are valid fractions, and do not have zero as denominator.
// 
// Simplify("1/5", "5/1") = true
// Simplify("1/6", "2/1") = false
// Simplify("7/10", "10/2") = false
func Simplify(x, n string) bool {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
