# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "strings"
)

// Create a function that returns true if the last character
// of a given string is an alphabetical character and is not
// a part of a word, and false otherwise.
// Note: "word" is a group of characters separated by space.
// 
// Examples:
// CheckIfLastCharIsALetter("apple pie") ➞ false
// CheckIfLastCharIsALetter("apple pi e") ➞ true
// CheckIfLastCharIsALetter("apple pi e ") ➞ false
// CheckIfLastCharIsALetter("") ➞ false
func CheckIfLastCharIsALetter(txt string) bool {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
