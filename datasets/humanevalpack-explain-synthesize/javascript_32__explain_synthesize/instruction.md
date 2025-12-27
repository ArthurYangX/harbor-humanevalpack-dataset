# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Evaluates polynomial with coefficients xs at point x.
  return xs[0] + xs[1] * x + xs[1] * x^2 + .... xs[n] * x^n
  */
const poly = (xs, x) => {
  return xs.reduce((prev, item, index) => {
    return prev + item * Math.pow(x, index);
  }, 0);
}

/*
  xs are coefficients of a polynomial.
  findZero find x such that poly(x) = 0.
  findZero returns only only zero point, even if there are many.
  Moreover, findZero only takes list xs having even number of coefficients
  and largest non zero coefficient as it guarantees
  a solution.
  >>> round(findZero([1, 2]), 2) # f(x) = 1 + 2x
  -0.5
  >>> round(findZero([-6, 11, -6, 1]), 2) # (x - 1) * (x - 2) * (x - 3) = -6 + 11x - 6x^2 + x^3
  1.0
  */
const findZero = (xs) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
