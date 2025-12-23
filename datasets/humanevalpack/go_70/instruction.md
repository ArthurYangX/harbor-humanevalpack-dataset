# Problem Description

Write a Go function `func StrangeSortList(lst []int) []int` to solve the following problem:
Given list of integers, return list in strange order.
Strange sorting, is when you start with the minimum value,
then maximum of the remaining integers, then minimum and so on.
Examples:
StrangeSortList([1, 2, 3, 4]) == [1, 4, 2, 3]
StrangeSortList([5, 5, 5, 5]) == [5, 5, 5, 5]
StrangeSortList([]) == []

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

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
