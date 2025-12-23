# Problem Description

Write a Go function `func RemoveVowels(text string) string` to solve the following problem:
RemoveVowels is a function that takes string and returns string without vowels.
>>> RemoveVowels('')
''
>>> RemoveVowels("abcdef\nghijklm")
'bcdf\nghjklm'
>>> RemoveVowels('abcdef')
'bcdf'
>>> RemoveVowels('aaaaa')
''
>>> RemoveVowels('aaBAA')
'B'
>>> RemoveVowels('zbcd')
'zbcd'

# Prompt

import (
    "regexp"
)

// RemoveVowels is a function that takes string and returns string without vowels.
// >>> RemoveVowels('')
// ''
// >>> RemoveVowels("abcdef\nghijklm")
// 'bcdf\nghjklm'
// >>> RemoveVowels('abcdef')
// 'bcdf'
// >>> RemoveVowels('aaaaa')
// ''
// >>> RemoveVowels('aaBAA')
// 'B'
// >>> RemoveVowels('zbcd')
// 'zbcd'
func RemoveVowels(text string) string {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
