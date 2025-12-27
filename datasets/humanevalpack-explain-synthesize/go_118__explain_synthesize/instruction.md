# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in go that matches the explanation and passes the unit tests.

# Prompt

import (
    "bytes"
)

// You are given a word. Your task is to find the closest vowel that stands between
// two consonants from the right side of the word (case sensitive).
// 
// Vowels in the beginning and ending doesn't count. Return empty string if you didn't
// find any vowel met the above condition.
// 
// You may assume that the given string contains English letter only.
// 
// Example:
// GetClosestVowel("yogurt") ==> "u"
// GetClosestVowel("FULL") ==> "U"
// GetClosestVowel("quick") ==> ""
// GetClosestVowel("ab") ==> ""
func GetClosestVowel(word string) string {


# Instructions

Please write your solution in the file `solution/solution.go`.
Ensure your code is self-contained and runs correctly.
