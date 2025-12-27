# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Given a list of numbers, return the sum of squares of the numbers
  in the list that are odd. Ignore numbers that are negative or not integers.
  doubleTheDifference([1, 3, 2, 0]) == 1 + 9 + 0 + 0 = 10
  doubleTheDifference([-1, -2, 0]) == 0
  doubleTheDifference([9, -2]) == 81
  doubleTheDifference([0]) == 0
  If the input list is empty, return 0.
  */
const doubleTheDifference = (lst) => {
  let p = 0
  for (let i = 0; i < lst.length; i++) {
    if (lst[i] % 2 == 1 && lst[i] > 0) {
      p += lst[i] * lst[i]
    }
  }
  return p
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

/* Given a list of numbers, return the sum of squares of the numbers
  in the list that are odd. Ignore numbers that are negative or not integers.
  doubleTheDifference([1, 3, 2, 0]) == 1 + 9 + 0 + 0 = 10
  doubleTheDifference([-1, -2, 0]) == 0
  doubleTheDifference([9, -2]) == 81
  doubleTheDifference([0]) == 0
  If the input list is empty, return 0.
  */
const doubleTheDifference = (lst) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
