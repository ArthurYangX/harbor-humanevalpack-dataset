# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// The FibFib number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
// Fibfib(0) == 0
// Fibfib(1) == 0
// Fibfib(2) == 1
// Fibfib(n) == Fibfib(n-1) + Fibfib(n-2) + Fibfib(n-3).
// Please write a function to efficiently compute the n-th element of the Fibfib number sequence.
// >>> Fibfib(1)
// 0
// >>> Fibfib(5)
// 4
// >>> Fibfib(8)
// 24
func Fibfib(n int) int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
