# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Create a function that returns true if the last character
  of a given string is an alphabetical character and is not
  a part of a word, and false otherwise.
  Note: "word" is a group of characters separated by space.
  Examples:
  checkIfLastCharIsALetter("apple pie") ➞ false
  checkIfLastCharIsALetter("apple pi e") ➞ true
  checkIfLastCharIsALetter("apple pi e ") ➞ false
  checkIfLastCharIsALetter("") ➞ false
  */
const checkIfLastCharIsALetter = (txt) => {
  let len = txt.length
  if (len == 0) { return false }
  let y = txt[len - 1].charCodeAt()
  if (len == 1) {
    if ((y >= 65 && y <= 90) || (y >= 97 && y <= 122)) { return true }
    return false
  }
  if (txt[len - 2] == ' ' && ((y >= 65 && y <= 90) || (y >= 97 && y <= 122))) { return true }
  return false
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

/* Create a function that returns true if the last character
  of a given string is an alphabetical character and is not
  a part of a word, and false otherwise.
  Note: "word" is a group of characters separated by space.
  Examples:
  checkIfLastCharIsALetter("apple pie") ➞ false
  checkIfLastCharIsALetter("apple pi e") ➞ true
  checkIfLastCharIsALetter("apple pi e ") ➞ false
  checkIfLastCharIsALetter("") ➞ false
  */
const checkIfLastCharIsALetter = (txt) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
