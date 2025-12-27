# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
  >>> flipCase('Hello')
  'hELLO'
  */
const flipCase = (string) => {
  return string.split('')
          .map(x => (x.toUpperCase() == x ? x.toLowerCase() : x.toUpperCase()))
          .join('');
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

/* For a given string, flip lowercase characters to uppercase and uppercase to lowercase.
  >>> flipCase('Hello')
  'hELLO'
  */
const flipCase = (string) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
