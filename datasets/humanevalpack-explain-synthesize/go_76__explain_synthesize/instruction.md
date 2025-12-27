# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Your task is to write a function that returns true if a number x is a simple
// power of n and false in other cases.
// x is a simple power of n if n**int=x
// For example:
// IsSimplePower(1, 4) => true
// IsSimplePower(2, 2) => true
// IsSimplePower(8, 2) => true
// IsSimplePower(3, 2) => false
// IsSimplePower(3, 1) => false
// IsSimplePower(5, 3) => false
func IsSimplePower(x int,n int) bool {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
