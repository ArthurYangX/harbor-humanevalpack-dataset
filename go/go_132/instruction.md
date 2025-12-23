# Problem Description

Write a Go function `func IsNested(s string) bool` to solve the following problem:
Create a function that takes a string as input which contains only square brackets.
The function should return true if and only if there is a valid subsequence of brackets
where at least one bracket in the subsequence is nested.
IsNested('[[]]') ➞ true
IsNested('[]]]]]]][[[[[]') ➞ false
IsNested('[][]') ➞ false
IsNested('[]') ➞ false
IsNested('[[][]]') ➞ true
IsNested('[[]][[') ➞ true

# Prompt


// Create a function that takes a string as input which contains only square brackets.
// The function should return true if and only if there is a valid subsequence of brackets
// where at least one bracket in the subsequence is nested.
// 
// IsNested('[[]]') ➞ true
// IsNested('[]]]]]]][[[[[]') ➞ false
// IsNested('[][]') ➞ false
// IsNested('[]') ➞ false
// IsNested('[[][]]') ➞ true
// IsNested('[[]][[') ➞ true
func IsNested(s string) bool {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
