# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Given an integer. return a tuple that has the number of even and odd digits respectively.

   Example:
      evenOddCount(-12) ==> (1, 1)
      evenOddCount(123) ==> (1, 2)
  */
const evenOddCount = (num) => {
  let o = 0
  let e = 0
  if (num < 0) { num = -num }
  while (num > 0) {
    if (num % 2 == 0) { e++ }
    else { o++ }
    num = (num - num % 10) / 10
  }
  return (e, o)
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

/*Given an integer. return a tuple that has the number of even and odd digits respectively.

   Example:
      evenOddCount(-12) ==> (1, 1)
      evenOddCount(123) ==> (1, 2)
  */
const evenOddCount = (num) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
