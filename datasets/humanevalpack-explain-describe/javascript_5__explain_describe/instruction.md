# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
  >>> intersperse([], 4)
  []
  >>> intersperse([1, 2, 3], 4)
  [1, 4, 2, 4, 3]
  */
const intersperse = (numbers, delimeter) => {
  if (!Array.isArray(numbers) || numbers.length == 0)
    return [];
  var result = [];
  for (const n of numbers) {
    result.push(n, delimeter);
  }
  result.pop();
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

/* Insert a number 'delimeter' between every two consecutive elements of input list `numbers'
  >>> intersperse([], 4)
  []
  >>> intersperse([1, 2, 3], 4)
  [1, 4, 2, 4, 3]
  */
const intersperse = (numbers, delimeter) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
