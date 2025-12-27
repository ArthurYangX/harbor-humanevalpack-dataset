# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "sort"
)

// Given an array of non-negative integers, return a copy of the given array after sorting,
// you will sort the given array in ascending order if the sum( first index value, last index value) is odd,
// or sort it in descending order if the sum( first index value, last index value) is even.
// 
// Note:
// * don't change the given array.
// 
// Examples:
// * SortArray([]) => []
// * SortArray([5]) => [5]
// * SortArray([2, 4, 3, 0, 1, 5]) => [0, 1, 2, 3, 4, 5]
// * SortArray([2, 4, 3, 0, 1, 5, 6]) => [6, 5, 4, 3, 2, 1, 0]
func SortArray(array []int) []int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
