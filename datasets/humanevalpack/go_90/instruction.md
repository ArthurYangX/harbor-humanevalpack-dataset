# Problem Description

Write a Go function `func NextSmallest(lst []int) interface{}` to solve the following problem:
You are given a list of integers.
Write a function NextSmallest() that returns the 2nd smallest element of the list.
Return nil if there is no such element.
NextSmallest([1, 2, 3, 4, 5]) == 2
NextSmallest([5, 1, 4, 3, 2]) == 2
NextSmallest([]) == nil
NextSmallest([1, 1]) == nil

# Prompt

import (
    "math"
    "sort"
)

// You are given a list of integers.
// Write a function NextSmallest() that returns the 2nd smallest element of the list.
// Return nil if there is no such element.
// 
// NextSmallest([1, 2, 3, 4, 5]) == 2
// NextSmallest([5, 1, 4, 3, 2]) == 2
// NextSmallest([]) == nil
// NextSmallest([1, 1]) == nil
func NextSmallest(lst []int) interface{} {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
