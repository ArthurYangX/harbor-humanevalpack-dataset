# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Given a string text, replace all spaces in it with underscores, 
  and if a string has more than 2 consecutive spaces, 
  then replace all consecutive spaces with - 
  
  fixSpaces("Example") == "Example"
  fixSpaces("Example 1") == "Example_1"
  fixSpaces(" Example 2") == "_Example_2"
  fixSpaces(" Example   3") == "_Example-3"
  */
const fixSpaces = (text) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
