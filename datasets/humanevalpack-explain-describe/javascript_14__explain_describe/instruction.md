# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Return list of all prefixes from shortest to longest of the input string
  >>> allPrefixes('abc')
  ['a', 'ab', 'abc']
  */
const allPrefixes = (string) => {
  var result = [];
  for (let i = 0; i < string.length; i++) {
    result.push(string.slice(0, i+1));
  }
  return result;
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

/* Return list of all prefixes from shortest to longest of the input string
  >>> allPrefixes('abc')
  ['a', 'ab', 'abc']
  */
const allPrefixes = (string) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
