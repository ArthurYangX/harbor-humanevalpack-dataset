# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Write a function that returns true if the object q will fly, and false otherwise.
  The object q will fly if it's balanced (it is a palindromic list) and the sum of its elements is less than or equal the maximum possible weight w.

  Example:
  willItFly([1, 2], 5) ➞ false
  # 1+2 is less than the maximum possible weight, but it's unbalanced.

  willItFly([3, 2, 3], 1) ➞ false
  # it's balanced, but 3+2+3 is more than the maximum possible weight.

  willItFly([3, 2, 3], 9) ➞ true
  # 3+2+3 is less than the maximum possible weight, and it's balanced.

  willItFly([3], 5) ➞ true
  # 3 is less than the maximum possible weight, and it's balanced.
  */
const willItFly = (q, w) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
