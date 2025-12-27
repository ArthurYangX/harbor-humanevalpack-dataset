# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt


// Return true is list elements are Monotonically increasing or decreasing.
// >>> Monotonic([1, 2, 4, 20])
// true
// >>> Monotonic([1, 20, 4, 10])
// false
// >>> Monotonic([4, 1, 0, -10])
// true
func Monotonic(l []int) bool {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
