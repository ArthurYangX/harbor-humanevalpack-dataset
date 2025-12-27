# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "math"
)

// For a given list of input numbers, calculate Mean Absolute Deviation
// around the mean of this dataset.
// Mean Absolute Deviation is the average absolute difference between each
// element and a centerpoint (mean in this case):
// MAD = average | x - x_mean |
// >>> MeanAbsoluteDeviation([1.0, 2.0, 3.0, 4.0])
// 1.0
func MeanAbsoluteDeviation(numbers []float64) float64 {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
