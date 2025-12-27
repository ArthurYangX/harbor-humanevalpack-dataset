# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "strings"
)

// Given a string representing a space separated lowercase letters, return a dictionary
// of the letter with the most repetition and containing the corresponding count.
// If several letters have the same occurrence, return all of them.
// 
// Example:
// Histogram('a b c') == {'a': 1, 'b': 1, 'c': 1}
// Histogram('a b b a') == {'a': 2, 'b': 2}
// Histogram('a b c a b') == {'a': 2, 'b': 2}
// Histogram('b b b b a') == {'b': 4}
// Histogram('') == {}
func Histogram(test string) map[rune]int {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
