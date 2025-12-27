# Context

You are given a reference implementation (canonical solution) to explain.

```js
/* Create a function that returns a tuple (a, b), where 'a' is
  the largest of negative integers, and 'b' is the smallest
  of positive integers in a list.
  If there is no negative or positive integers, return them as null.
  Examples:
  largestSmallestIntegers([2, 4, 1, 3, 5, 7]) == (null, 1)
  largestSmallestIntegers([]) == (null, null)
  largestSmallestIntegers([0]) == (null, null)
  */
const largestSmallestIntegers = (lst) => {
  let a = Infinity
  let b = -Infinity
  for (let i = 0; i < lst.length; i++) {
    if (lst[i] > 0 && lst[i] < a) { a = lst[i] }
    if (lst[i] < 0 && lst[i] > b) { b = lst[i] }
  }
  if (a == Infinity) { a = null }
  if (b == -Infinity) { b = null }
  return (b, a)
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

/* Create a function that returns a tuple (a, b), where 'a' is
  the largest of negative integers, and 'b' is the smallest
  of positive integers in a list.
  If there is no negative or positive integers, return them as null.
  Examples:
  largestSmallestIntegers([2, 4, 1, 3, 5, 7]) == (null, 1)
  largestSmallestIntegers([]) == (null, null)
  largestSmallestIntegers([0]) == (null, null)
  */
const largestSmallestIntegers = (lst) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
