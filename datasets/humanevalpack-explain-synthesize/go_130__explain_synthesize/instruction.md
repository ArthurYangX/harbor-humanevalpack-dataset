# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Everyone knows Fibonacci sequence, it was studied deeply by mathematicians in
// the last couple centuries. However, what people don't know is Tribonacci sequence.
// Tribonacci sequence is defined by the recurrence:
// Tri(1) = 3
// Tri(n) = 1 + n / 2, if n is even.
// Tri(n) =  Tri(n - 1) + Tri(n - 2) + Tri(n + 1), if n is odd.
// For example:
// Tri(2) = 1 + (2 / 2) = 2
// Tri(4) = 3
// Tri(3) = Tri(2) + Tri(1) + Tri(4)
// = 2 + 3 + 3 = 8
// You are given a non-negative integer number n, you have to a return a list of the
// first n + 1 numbers of the Tribonacci sequence.
// Examples:
// Tri(3) = [1, 3, 2, 8]
func Tri(n int) []float64 {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
