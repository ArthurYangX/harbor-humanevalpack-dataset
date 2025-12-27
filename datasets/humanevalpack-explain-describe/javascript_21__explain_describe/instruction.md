# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Given list of numbers (of at least two elements), apply a linear transform to that list,
  such that the smallest number will become 0 and the largest will become 1
  >>> rescaleToUnit([1.0, 2.0, 3.0, 4.0, 5.0])
  [0.0, 0.25, 0.5, 0.75, 1.0]
  */
const rescaleToUnit = (numbers) => {
  var min_number = Math.min(...numbers);
  var max_number = Math.max(...numbers);
  return numbers.map(x => (x - min_number) / (max_number - min_number));
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

/* Given list of numbers (of at least two elements), apply a linear transform to that list,
  such that the smallest number will become 0 and the largest will become 1
  >>> rescaleToUnit([1.0, 2.0, 3.0, 4.0, 5.0])
  [0.0, 0.25, 0.5, 0.75, 1.0]
  */
const rescaleToUnit = (numbers) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
