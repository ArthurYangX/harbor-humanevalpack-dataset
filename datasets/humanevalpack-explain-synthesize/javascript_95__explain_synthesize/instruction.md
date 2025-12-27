# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  Given a dictionary, return true if all keys are strings in lower 
  case or all keys are strings in upper case, else return false.
  The function should return false is the given dictionary is empty.
  Examples:
  checkDictCase({"a":"apple", "b":"banana"}) should return true.
  checkDictCase({"a":"apple", "A":"banana", "B":"banana"}) should return false.
  checkDictCase({"a":"apple", 8:"banana", "a":"apple"}) should return false.
  checkDictCase({"Name":"John", "Age":"36", "City":"Houston"}) should return false.
  checkDictCase({"STATE":"NC", "ZIP":"12345" }) should return true.
  */
const checkDictCase = (dict) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
