# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

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

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
