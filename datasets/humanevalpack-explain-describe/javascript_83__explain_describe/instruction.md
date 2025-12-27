# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*
  Given a positive integer n, return the count of the numbers of n-digit
  positive integers that start or end with 1.
  */
const startsOneEnds = (n) => {
  if (n == 1) { return 1 }
  let t = 18
  for (let i = 2; i < n; i++) {
    t = t * 10
  }
  return t
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

/*
  Given a positive integer n, return the count of the numbers of n-digit
  positive integers that start or end with 1.
  */
const startsOneEnds = (n) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
