# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Return a string containing space-delimited numbers starting from 0 upto n inclusive.
  >>> stringSequence(0)
  '0'
  >>> stringSequence(5)
  '0 1 2 3 4 5'
  */
const stringSequence = (n) => {
  return [...Array(n).keys(), n].join(' ')
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

/* Return a string containing space-delimited numbers starting from 0 upto n inclusive.
  >>> stringSequence(0)
  '0'
  >>> stringSequence(5)
  '0 1 2 3 4 5'
  */
const stringSequence = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
