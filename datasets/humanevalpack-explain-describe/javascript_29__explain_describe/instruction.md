# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Filter an input list of strings only for ones that start with a given prefix.
  >>> filterByPrefix([], 'a')
  []
  >>> filterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
  ['abc', 'array']
  */
const filterByPrefix = (strings, prefix) => {
  return strings.filter(x => x.startsWith(prefix));
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

/* Filter an input list of strings only for ones that start with a given prefix.
  >>> filterByPrefix([], 'a')
  []
  >>> filterByPrefix(['abc', 'bcd', 'cde', 'array'], 'a')
  ['abc', 'array']
  */
const filterByPrefix = (strings, prefix) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
