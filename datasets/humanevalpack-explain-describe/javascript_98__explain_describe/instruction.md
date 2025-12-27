# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given a string s, count the number of uppercase vowels in even indices.
  
  For example:
  countUpper('aBCdEf') returns 1
  countUpper('abcdefg') returns 0
  countUpper('dBBE') returns 0
  */
const countUpper = (s) => {
  let p = 0
  for (let i = 0; i < s.length; i += 2) {
    if (s[i] == 'A' || s[i] == 'E' || s[i] == 'I' || s[i] == 'O' || s[i] == 'U') { p++ }
  }
  return p
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

/*
  Given a string s, count the number of uppercase vowels in even indices.
  
  For example:
  countUpper('aBCdEf') returns 1
  countUpper('abcdefg') returns 0
  countUpper('dBBE') returns 0
  */
const countUpper = (s) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
