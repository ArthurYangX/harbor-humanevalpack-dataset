# Problem Description

Write a Go function `func Encode(message string) string` to solve the following problem:
Write a function that takes a message, and Encodes in such a
way that it swaps case of all letters, replaces all vowels in
the message with the letter that appears 2 places ahead of that
vowel in the english alphabet.
Assume only letters.
Examples:
>>> Encode('test')
'TGST'
>>> Encode('This is a message')
'tHKS KS C MGSSCGG'

# Prompt

import (
    "strings"
)

// Write a function that takes a message, and Encodes in such a
// way that it swaps case of all letters, replaces all vowels in
// the message with the letter that appears 2 places ahead of that
// vowel in the english alphabet.
// Assume only letters.
// 
// Examples:
// >>> Encode('test')
// 'TGST'
// >>> Encode('This is a message')
// 'tHKS KS C MGSSCGG'
func Encode(message string) string {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
