# Problem Description

Write a Go function `func FilterBySubstring(stringList []string, substring string) []string` to solve the following problem:
Filter an input list of strings only for ones that contain given substring
>>> FilterBySubstring([], 'a')
[]
>>> FilterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
['abc', 'bacd', 'array']

# Prompt

import (
    "strings"
)

// Filter an input list of strings only for ones that contain given substring
// >>> FilterBySubstring([], 'a')
// []
// >>> FilterBySubstring(['abc', 'bacd', 'cde', 'array'], 'a')
// ['abc', 'bacd', 'array']
func FilterBySubstring(stringList []string, substring string) []string {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
