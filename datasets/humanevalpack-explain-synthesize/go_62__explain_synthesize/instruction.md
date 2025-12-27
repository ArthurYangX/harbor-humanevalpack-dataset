# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

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

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
