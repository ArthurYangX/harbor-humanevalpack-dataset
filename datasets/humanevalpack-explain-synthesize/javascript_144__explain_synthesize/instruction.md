# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*Your task is to implement a function that will simplify the expression
  x * n. The function returns true if x * n evaluates to a whole number and false
  otherwise. Both x and n, are string representation of a fraction, and have the following format,
  <numerator>/<denominator> where both numerator and denominator are positive whole numbers.

  You can assume that x, and n are valid fractions, and do not have zero as denominator.

  simplify("1/5", "5/1") = true
  simplify("1/6", "2/1") = false
  simplify("7/10", "10/2") = false
  */
const simplify = (x, n) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
