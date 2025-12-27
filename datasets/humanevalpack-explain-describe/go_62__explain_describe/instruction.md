# Context

You are given a reference implementation (canonical solution) to explain.

```go
// xs represent coefficients of a polynomial.
// xs[0] + xs[1] * x + xs[2] * x^2 + ....
// Return Derivative of this polynomial in the same form.
// >>> Derivative([3, 1, 2, 4, 5])
// [1, 4, 12, 20]
// >>> Derivative([1, 2, 3])
// [2, 6]
func Derivative(xs []int) []int {
    l := len(xs)
	y := make([]int, l - 1)
	for i := 0; i < l - 1; i++ {
		y[i] = xs[i + 1] * (i + 1)
	}
	return y
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


// xs represent coefficients of a polynomial.
// xs[0] + xs[1] * x + xs[2] * x^2 + ....
// Return Derivative of this polynomial in the same form.
// >>> Derivative([3, 1, 2, 4, 5])
// [1, 4, 12, 20]
// >>> Derivative([1, 2, 3])
// [2, 6]
func Derivative(xs []int) []int {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
