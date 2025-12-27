# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Write a function that accepts two lists of strings and returns the list that has
  total number of chars in the all strings of the list less than the other list.

  if the two lists have the same number of chars, return the first list.

  Examples
  totalMatch([], []) ➞ []
  totalMatch(['hi', 'admin'], ['hI', 'Hi']) ➞ ['hI', 'Hi']
  totalMatch(['hi', 'admin'], ['hi', 'hi', 'admin', 'project']) ➞ ['hi', 'admin']
  totalMatch(['hi', 'admin'], ['hI', 'hi', 'hi']) ➞ ['hI', 'hi', 'hi']
  totalMatch(['4'], ['1', '2', '3', '4', '5']) ➞ ['4']
  */
const totalMatch = (lst1, lst2) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
