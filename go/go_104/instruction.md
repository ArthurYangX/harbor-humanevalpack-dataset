# Problem Description

Write a Go function `func UniqueDigits(x []int) []int` to solve the following problem:
Given a list of positive integers x. return a sorted list of all
elements that hasn't any even digit.
Note: Returned list should be sorted in increasing order.
For example:
>>> UniqueDigits([15, 33, 1422, 1])
[1, 15, 33]
>>> UniqueDigits([152, 323, 1422, 10])
[]

# Prompt

import (
    "sort"
    "strconv"
)

// Given a list of positive integers x. return a sorted list of all
// elements that hasn't any even digit.
// 
// Note: Returned list should be sorted in increasing order.
// 
// For example:
// >>> UniqueDigits([15, 33, 1422, 1])
// [1, 15, 33]
// >>> UniqueDigits([152, 323, 1422, 10])
// []
func UniqueDigits(x []int) []int {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
