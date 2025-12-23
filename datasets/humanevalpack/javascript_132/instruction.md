# Problem Description

Write a JavaScript function `const isNested = (string)` to solve the following problem:
Create a function that takes a string as input which contains only square brackets.
The function should return true if and only if there is a valid subsequence of brackets
where at least one bracket in the subsequence is nested.
isNested('[[]]') ➞ true
isNested('[]]]]]]][[[[[]') ➞ false
isNested('[][]') ➞ false
isNested('[]') ➞ false
isNested('[[][]]') ➞ true
isNested('[[]][[') ➞ true

# Prompt

/*
  Create a function that takes a string as input which contains only square brackets.
  The function should return true if and only if there is a valid subsequence of brackets
  where at least one bracket in the subsequence is nested.
  isNested('[[]]') ➞ true
  isNested('[]]]]]]][[[[[]') ➞ false
  isNested('[][]') ➞ false
  isNested('[]') ➞ false
  isNested('[[][]]') ➞ true
  isNested('[[]][[') ➞ true
  */
const isNested = (string) => {


# Instructions

Please write your solution in the file `solution/solution.py`.
Ensure your code is self-contained and compiles/runs correctly.
