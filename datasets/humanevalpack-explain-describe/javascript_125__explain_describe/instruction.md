# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Given a string of words, return a list of words split on whitespace, if no whitespaces exists in the text you
  should split on commas ',' if no commas exists you should return the number of lower-case letters with odd order in the
  alphabet, ord('a') = 0, ord('b') = 1, ... ord('z') = 25
  Examples
  splitWords("Hello world!") ➞ ["Hello", "world!"]
  splitWords("Hello,world!") ➞ ["Hello", "world!"]
  splitWords("abcdef") == 3
  */
const splitWords = (txt) => {
  let t = txt.split(/\s/)
  if (t.length > 1) {
    return t
  } else {
    t = txt.split(/,/)
    if (t.length > 1) {
      return t
    } else {
      let p = 0
      for (let i = 0; i < txt.length; i++) {
        let m = txt[i].charCodeAt()
        if (m >= 97 && m <= 122 && m % 2 == 0) {
          p++
        }
      }
      return p
    }
  }
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

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
