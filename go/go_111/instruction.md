# Problem Description

Write a Go function `func Histogram(test string) map[rune]int` to solve the following problem:
Given a string representing a space separated lowercase letters, return a dictionary
of the letter with the most repetition and containing the corresponding count.
If several letters have the same occurrence, return all of them.
Example:
Histogram('a b c') == {'a': 1, 'b': 1, 'c': 1}
Histogram('a b b a') == {'a': 2, 'b': 2}
Histogram('a b c a b') == {'a': 2, 'b': 2}
Histogram('b b b b a') == {'b': 4}
Histogram('') == {}

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

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
