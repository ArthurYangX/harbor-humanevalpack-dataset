# Context

You are given a natural-language explanation of a function.

Placeholder explanation for oracle/dataset generation. Implement the function described by the unit tests and the prompt.

# Instruction

Write a correct implementation in javascript that matches the explanation and passes the unit tests.

# Prompt

/*
  returns encoded string by shifting every character by 5 in the alphabet.
  */
const encodeShift = (s) => {
  return s.split("").map(ch => String.fromCharCode(
    ((ch.charCodeAt(0) + 5 - "a".charCodeAt(0)) % 26) + "a".charCodeAt(0)
  )).join("");
}

/*
  takes as input string encoded with encode_shift function. Returns decoded string.
  */
const decodeShift = (s) => {


# Instructions

Please write your solution in the file `solution/solution.js`.
Ensure your code is self-contained and runs correctly.
