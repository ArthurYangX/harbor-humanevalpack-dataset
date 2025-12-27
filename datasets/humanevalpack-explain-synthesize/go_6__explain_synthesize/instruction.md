# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "math"
    "strings"
)

// Input to this function is a string represented multiple groups for nested parentheses separated by spaces.
// For each of the group, output the deepest level of nesting of parentheses.
// E.g. (()()) has maximum two levels of nesting while ((())) has three.
// 
// >>> ParseNestedParens('(()()) ((())) () ((())()())')
// [2, 3, 1, 3]
func ParseNestedParens(paren_string string) []int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
