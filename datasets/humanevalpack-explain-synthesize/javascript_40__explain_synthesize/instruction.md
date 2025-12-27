# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  triplesSumToZero takes a list of integers as an input.
  it returns true if there are three distinct elements in the list that
  sum to zero, and false otherwise.

  >>> triplesSumToZero([1, 3, 5, 0])
  false
  >>> triplesSumToZero([1, 3, -2, 1])
  true
  >>> triplesSumToZero([1, 2, 3, 7])
  false
  >>> triplesSumToZero([2, 4, -5, 3, 9, 7])
  true
  >>> triplesSumToZero([1])
  false
  */
const triplesSumToZero = (l) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
