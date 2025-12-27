# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* From a given list of integers, generate a list of rolling maximum element found until given moment
  in the sequence.
  >>> rollingMax([1, 2, 3, 2, 3, 4, 2])
  [1, 2, 3, 3, 3, 4, 4]
  */
const rollingMax = (numbers) => {
  var running_max, result = [];
  for (const n of numbers) {
    if (running_max == undefined)
      running_max = n;
    else
      running_max = Math.max(running_max, n);
    result.push(running_max);
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

/* From a given list of integers, generate a list of rolling maximum element found until given moment
  in the sequence.
  >>> rollingMax([1, 2, 3, 2, 3, 4, 2])
  [1, 2, 3, 3, 3, 4, 4]
  */
const rollingMax = (numbers) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
