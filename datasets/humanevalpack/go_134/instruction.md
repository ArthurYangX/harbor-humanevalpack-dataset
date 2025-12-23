# Problem Description

Write a Go function `func CheckIfLastCharIsALetter(txt string) bool` to solve the following problem:
Create a function that returns true if the last character
of a given string is an alphabetical character and is not
a part of a word, and false otherwise.
Note: "word" is a group of characters separated by space.
Examples:
CheckIfLastCharIsALetter("apple pie") ➞ false
CheckIfLastCharIsALetter("apple pi e") ➞ true
CheckIfLastCharIsALetter("apple pi e ") ➞ false
CheckIfLastCharIsALetter("") ➞ false

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

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
