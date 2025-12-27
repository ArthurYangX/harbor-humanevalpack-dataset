# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Return true is list elements are monotonically increasing or decreasing.
  >>> monotonic([1, 2, 4, 20])
  true
  >>> monotonic([1, 20, 4, 10])
  false
  >>> monotonic([4, 1, 0, -10])
  true
  */
const monotonic = (l) => {
  var sort1 = [...l].sort((a, b) => a - b);
  var sort2 = [...l].sort((a, b) => b - a);
  if (JSON.stringify(l) === JSON.stringify(sort1) ||
      JSON.stringify(l) === JSON.stringify(sort2))
    return true;
  return false;
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

/*Return true is list elements are monotonically increasing or decreasing.
  >>> monotonic([1, 2, 4, 20])
  true
  >>> monotonic([1, 20, 4, 10])
  false
  >>> monotonic([4, 1, 0, -10])
  true
  */
const monotonic = (l) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
