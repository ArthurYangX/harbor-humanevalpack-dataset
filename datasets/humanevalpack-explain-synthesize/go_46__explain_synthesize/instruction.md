# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// The Fib4 number sequence is a sequence similar to the Fibbonacci sequnece that's defined as follows:
// Fib4(0) -> 0
// Fib4(1) -> 0
// Fib4(2) -> 2
// Fib4(3) -> 0
// Fib4(n) -> Fib4(n-1) + Fib4(n-2) + Fib4(n-3) + Fib4(n-4).
// Please write a function to efficiently compute the n-th element of the Fib4 number sequence.  Do not use recursion.
// >>> Fib4(5)
// 4
// >>> Fib4(6)
// 8
// >>> Fib4(7)
// 14
func Fib4(n int) int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
