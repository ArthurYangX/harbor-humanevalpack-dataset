# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "sort"
)

// Given list of integers, return list in strange order.
// Strange sorting, is when you start with the minimum value,
// then maximum of the remaining integers, then minimum and so on.
// 
// Examples:
// StrangeSortList([1, 2, 3, 4]) == [1, 4, 2, 3]
// StrangeSortList([5, 5, 5, 5]) == [5, 5, 5, 5]
// StrangeSortList([]) == []
func StrangeSortList(lst []int) []int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
