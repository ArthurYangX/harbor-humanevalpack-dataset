# Problem Description

Write a Go function `func FixSpaces(text string) string` to solve the following problem:
Given a string text, replace all spaces in it with underscores,
and if a string has more than 2 consecutive spaces,
then replace all consecutive spaces with -
FixSpaces("Example") == "Example"
FixSpaces("Example 1") == "Example_1"
FixSpaces(" Example 2") == "_Example_2"
FixSpaces(" Example   3") == "_Example-3"

# Prompt


// Given a string text, replace all spaces in it with underscores,
// and if a string has more than 2 consecutive spaces,
// then replace all consecutive spaces with -
// 
// FixSpaces("Example") == "Example"
// FixSpaces("Example 1") == "Example_1"
// FixSpaces(" Example 2") == "_Example_2"
// FixSpaces(" Example   3") == "_Example-3"
func FixSpaces(text string) string {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
