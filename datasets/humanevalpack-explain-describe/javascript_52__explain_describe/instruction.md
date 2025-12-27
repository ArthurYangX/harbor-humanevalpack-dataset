# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Return true if all numbers in the list l are below threshold t.
  >>> belowThreshold([1, 2, 4, 10], 100)
  true
  >>> belowThreshold([1, 20, 4, 10], 5)
  false
  */
const belowThreshold = (l, t) => {
  for (const e of l)
    if (e >= t)
      return false;
  return true;
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

/*Return true if all numbers in the list l are below threshold t.
  >>> belowThreshold([1, 2, 4, 10], 100)
  true
  >>> belowThreshold([1, 20, 4, 10], 5)
  false
  */
const belowThreshold = (l, t) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
