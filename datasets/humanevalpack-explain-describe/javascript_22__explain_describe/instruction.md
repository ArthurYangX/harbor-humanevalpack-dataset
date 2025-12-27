# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Filter given list of any python values only for integers
  >>> filterIntegers(['a', 3.14, 5])
  [5]
  >>> filterIntegers([1, 2, 3, 'abc', {}, []])
  [1, 2, 3]
  */
const filterIntegers = (values) => {
  return values.filter(x => Number.isInteger(x));
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

/* Filter given list of any python values only for integers
  >>> filterIntegers(['a', 3.14, 5])
  [5]
  >>> filterIntegers([1, 2, 3, 'abc', {}, []])
  [1, 2, 3]
  */
const filterIntegers = (values) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
