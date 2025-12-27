# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  removeVowels is a function that takes string and returns string without vowels.
  >>> removeVowels('')
  ''
  >>> removeVowels("abcdef\nghijklm")
  'bcdf\nghjklm'
  >>> removeVowels('abcdef')
  'bcdf'
  >>> removeVowels('aaaaa')
  ''
  >>> removeVowels('aaBAA')
  'B'
  >>> removeVowels('zbcd')
  'zbcd'
  */
const removeVowels = (text) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
