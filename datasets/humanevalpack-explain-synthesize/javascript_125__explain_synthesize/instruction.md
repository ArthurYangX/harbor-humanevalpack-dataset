# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/* Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
  should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
  alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
  Examples
  splitWords("Hello world!") ➞ ["Hello", "world!"]
  splitWords("Hello,world!") ➞ ["Hello", "world!"]
  splitWords("abcdef") == 3
  */
const splitWords = (txt) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
