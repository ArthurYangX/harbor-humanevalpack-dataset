# Problem Description

Write a Go function `func HasCloseElements(numbers []float64, threshold float64) bool` to solve the following problem:
Check if in given list of numbers, are any two numbers closer to each other than given threshold.
>>> HasCloseElements([]float64{1.0, 2.0, 3.0}, 0.5)
false
>>> HasCloseElements([]float64{1.0, 2.8, 3.0, 4.0, 5.0, 2.0}, 0.3)
true

# Prompt

import (
    "math"
)

// Check if in given list of numbers, are any two numbers closer to each other than given threshold.
// >>> HasCloseElements([]float64{1.0, 2.0, 3.0}, 0.5)
// false
// >>> HasCloseElements([]float64{1.0, 2.8, 3.0, 4.0, 5.0, 2.0}, 0.3)
// true
func HasCloseElements(numbers []float64, threshold float64) bool {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
