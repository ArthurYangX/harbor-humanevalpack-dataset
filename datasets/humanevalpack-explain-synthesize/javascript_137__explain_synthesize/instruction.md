# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Create a function that takes integers, floats, or strings representing
  real numbers, and returns the larger variable in its given variable type.
  Return null if the values are equal.
  Note: If a real number is represented as a string, the floating point might be . or ,

  compareOne(1, 2.5) ➞ 2.5
  compareOne(1, "2,3") ➞ "2,3"
  compareOne("5,1", "6") ➞ "6"
  compareOne("1", 1) ➞ null
  */
const compareOne = (a, b) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
