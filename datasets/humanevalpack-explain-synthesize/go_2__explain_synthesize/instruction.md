# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "math"
)

// Given a positive floating point number, it can be decomposed into
// and integer part (largest integer smaller than given number) and decimals
// (leftover part always smaller than 1).
// 
// Return the decimal part of the number.
// >>> TruncateNumber(3.5)
// 0.5
func TruncateNumber(number float64) float64 {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
