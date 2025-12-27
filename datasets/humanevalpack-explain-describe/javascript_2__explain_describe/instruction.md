# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Given a positive floating point number, it can be decomposed into
  and integer part (largest integer smaller than given number) and decimals
  (leftover part always smaller than 1).

  Return the decimal part of the number.
  >>> truncateNumber(3.5)
  0.5
  */
const truncateNumber = (number) => {
  return number % 1.0;
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

/* Given a positive floating point number, it can be decomposed into
  and integer part (largest integer smaller than given number) and decimals
  (leftover part always smaller than 1).

  Return the decimal part of the number.
  >>> truncateNumber(3.5)
  0.5
  */
const truncateNumber = (number) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
