# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "strings"
)

// Given a positive integer, obtain its roman numeral equivalent as a string,
// and return it in lowercase.
// Restrictions: 1 <= num <= 1000
// 
// Examples:
// >>> IntToMiniRoman(19) == 'xix'
// >>> IntToMiniRoman(152) == 'clii'
// >>> IntToMiniRoman(426) == 'cdxxvi'
func IntToMiniRoman(number int) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
