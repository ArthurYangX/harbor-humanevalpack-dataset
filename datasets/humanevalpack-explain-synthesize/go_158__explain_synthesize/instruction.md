# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "sort"
)

// Write a function that accepts a list of strings.
// The list contains different words. Return the word with maximum number
// of unique characters. If multiple strings have maximum number of unique
// characters, return the one which comes first in lexicographical order.
// 
// FindMax(["name", "of", "string"]) == "string"
// FindMax(["name", "enam", "game"]) == "enam"
// FindMax(["aaaaaaa", "bb" ,"cc"]) == ""aaaaaaa"
func FindMax(words []string) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
