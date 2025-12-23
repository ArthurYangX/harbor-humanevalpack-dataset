# Problem Description

Write a Go function `func CompareOne(a, b interface{}) interface{}` to solve the following problem:
Create a function that takes integers, floats, or strings representing
real numbers, and returns the larger variable in its given variable type.
Return nil if the values are equal.
Note: If a real number is represented as a string, the floating point might be . or ,
CompareOne(1, 2.5) ➞ 2.5
CompareOne(1, "2,3") ➞ "2,3"
CompareOne("5,1", "6") ➞ "6"
CompareOne("1", 1) ➞ nil

# Prompt

import (
    "fmt"
    "strconv"
    "strings"
)

// Create a function that takes integers, floats, or strings representing
// real numbers, and returns the larger variable in its given variable type.
// Return nil if the values are equal.
// Note: If a real number is represented as a string, the floating point might be . or ,
// 
// CompareOne(1, 2.5) ➞ 2.5
// CompareOne(1, "2,3") ➞ "2,3"
// CompareOne("5,1", "6") ➞ "6"
// CompareOne("1", 1) ➞ nil
func CompareOne(a, b interface{}) interface{} {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
