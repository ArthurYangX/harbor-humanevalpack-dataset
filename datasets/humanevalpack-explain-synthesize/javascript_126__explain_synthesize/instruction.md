# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/* Given a list of numbers, return whether or not they are sorted
  in ascending order. If list has more than 1 duplicate of the same
  number, return false. Assume no negative numbers and only integers.
  Examples
  isSorted([5]) ➞ true
  isSorted([1, 2, 3, 4, 5]) ➞ true
  isSorted([1, 3, 2, 4, 5]) ➞ false
  isSorted([1, 2, 3, 4, 5, 6]) ➞ true
  isSorted([1, 2, 3, 4, 5, 6, 7]) ➞ true
  isSorted([1, 3, 2, 4, 5, 6, 7]) ➞ false
  isSorted([1, 2, 2, 3, 3, 4]) ➞ true
  isSorted([1, 2, 2, 2, 3, 4]) ➞ false
  */
const isSorted = (lst) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
