# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "math"
)

// Given a list of numbers, return the sum of squares of the numbers
// in the list that are odd. Ignore numbers that are negative or not integers.
// 
// DoubleTheDifference([1, 3, 2, 0]) == 1 + 9 + 0 + 0 = 10
// DoubleTheDifference([-1, -2, 0]) == 0
// DoubleTheDifference([9, -2]) == 81
// DoubleTheDifference([0]) == 0
// 
// If the input list is empty, return 0.
func DoubleTheDifference(lst []float64) int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
