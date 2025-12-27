# Context

You are given a reference implementation (canonical solution) to explain.

```js
/*Complete the function that takes two integers and returns 
  the product of their unit digits.
  Assume the input is always valid.
  Examples:
  multiply(148, 412) should return 16.
  multiply(19, 28) should return 72.
  multiply(2020, 1851) should return 0.
  multiply(14,-15) should return 20.
  */
const multiply = (a, b) => {
  if (a < 0) { a = -a }
  if (b < 0) { b = -b }
  return (a % 10) * (b % 10)
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

/*Complete the function that takes two integers and returns 
  the product of their unit digits.
  Assume the input is always valid.
  Examples:
  multiply(148, 412) should return 16.
  multiply(19, 28) should return 72.
  multiply(2020, 1851) should return 0.
  multiply(14,-15) should return 20.
  */
const multiply = (a, b) => {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
